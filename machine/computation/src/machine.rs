use crate::constants;
use cryptography::hash::Hash;
use jsonrpc_cartesi_machine::{JsonRpcCartesiMachineClient, MachineRuntimeConfig};
use sha3::{Digest, Keccak256};
use std::sync::{Arc, Mutex};
use utils::arithmetic;
pub struct Machine {
    pub path: String,
    pub machine: Arc<Mutex<JsonRpcCartesiMachineClient>>,
    cycle: u64,
    pub ucycle: u64,
    start_cycle: u64,
    pub initial_hash: Hash,
}

impl Machine {
    pub async fn new_from_path(url: &str, path: &str) -> Machine {
        let machine = Arc::new(Mutex::new(
            JsonRpcCartesiMachineClient::new(url.to_string())
                .await
                .unwrap(),
        ));
        machine
            .lock()
            .unwrap()
            .load_machine(path, &MachineRuntimeConfig::default())
            .await
            .unwrap();
        let start_cycle = machine
            .lock()
            .unwrap()
            .get_csr_address("mcycle".to_string())
            .await
            .unwrap();

        // Machine can never be advanced on the micro arch.
        // Validators must verify this first
        assert_eq!(
            machine
                .lock()
                .unwrap()
                .get_csr_address("uarch_cycle".to_string())
                .await
                .unwrap(),
            800
        );
        Machine {
            path: path.to_string(),
            machine: Arc::clone(&machine),
            cycle: 0,
            ucycle: 0,
            start_cycle,
            initial_hash: Hash::from_digest(
                Arc::clone(&machine)
                    .lock()
                    .unwrap()
                    .get_root_hash()
                    .await
                    .unwrap(),
            ),
        }
    }

    pub async fn state(&self) -> ComputationState {
        ComputationState::from_current_machine_state(Arc::clone(&self.machine)).await
    }

    async fn get_logs(url: &str, path: &str, cycle: u64, ucycle: u64) -> String {
        let mut machine = Machine::new_from_path(path, url).await;
        machine.run(cycle).await;
        machine.run_uarch(ucycle).await;

        if ucycle as i64 == constants::UARCH_SPAN {
            machine.run_uarch(constants::UARCH_SPAN as u64).await;
            eprintln!("ureset, not implemented");
        }

        let access_log = jsonrpc_cartesi_machine::AccessLogType {
            annotations: true,
            proofs: true,
        };
        let logs = machine
            .machine
            .lock()
            .unwrap()
            .step(&access_log, false)
            .await
            .unwrap();

        let mut encoded = Vec::new();

        for a in &logs.accesses {
            assert_eq!(a.log2_size, 3);
            if a.r#type == jsonrpc_cartesi_machine::AccessType::Read {
                encoded.push(a.read_data.clone());
            }

            encoded.push(hex::decode(a.proof.target_hash.clone()).unwrap());

            let decoded_sibling_hashes: Result<Vec<Vec<u8>>, hex::FromHexError> = a
                .proof
                .sibling_hashes
                .iter()
                .map(|hex_string| hex::decode(hex_string))
                .collect();

            let mut decoded = decoded_sibling_hashes.unwrap();
            decoded.reverse();
            encoded.extend_from_slice(&decoded.clone());

            assert_eq!(
                ver(
                    hex::decode(a.proof.target_hash.clone()).unwrap(),
                    a.address,
                    decoded.clone()
                ),
                hex::decode(a.proof.root_hash.clone()).unwrap()
            );
        }
        let data: Vec<u8> = encoded.iter().cloned().flatten().collect();

        let hex_data = hex::encode(data);

        format!("\"{}\"", hex_data)
    }

    pub fn add_and_clamp(x: u64, y: u64) -> u64 {
        if arithmetic::ult(x, arithmetic::max_uint(64) as u64 - y) {
            x + y
        } else {
            arithmetic::max_uint(64) as u64
        }
    }
    pub async fn run(&mut self, cycle: u64) {
        assert!(arithmetic::ulte(self.cycle, cycle));
        let physical_cycle = Machine::add_and_clamp(self.start_cycle, cycle);
        let machine = Arc::clone(&self.machine);
        while !(machine.lock().unwrap().read_iflags_h().await.unwrap()
            || machine
                .lock()
                .unwrap()
                .get_csr_address("mcycle".to_string())
                .await
                .unwrap()
                == physical_cycle)
        {
            machine.lock().unwrap().run(physical_cycle).await.unwrap();
        }
        self.cycle = cycle;
    }

    pub async fn run_uarch(&mut self, ucycle: u64) {
        assert!(
            arithmetic::ulte(self.ucycle, ucycle),
            "{}",
            format!("{}, {}", self.ucycle, ucycle)
        );
        self.machine
            .lock()
            .unwrap()
            .run_uarch(ucycle)
            .await
            .unwrap();
        self.ucycle = ucycle;
    }

    pub async fn increment_uarch(&mut self) {
        self.machine
            .lock()
            .unwrap()
            .run_uarch(self.ucycle + 1)
            .await
            .unwrap();
        self.ucycle = self.ucycle + 1;
    }

    pub async fn ureset(&mut self) {
        self.machine
            .lock()
            .unwrap()
            .reset_uarch_state()
            .await
            .unwrap();
        self.cycle += 1;
        self.ucycle = 0;
    }
}

fn ver(mut t: Vec<u8>, p: u64, s: Vec<Vec<u8>>) -> Vec<u8> {
    let stride = p >> 3;
    for (k, v) in s.iter().enumerate() {
        if (stride >> k) % 2 == 0 {
            let mut keccak = Keccak256::new();
            keccak.update(&t);
            keccak.update(v);
            t = keccak.finalize().to_vec();
        } else {
            let mut keccak = Keccak256::new();
            keccak.update(v);
            keccak.update(&t);
            t = keccak.finalize().to_vec();
        }
    }

    t
}

#[derive(Debug)]
pub struct ComputationState {
    pub root_hash: Hash,
    pub halted: bool,
    pub uhalted: bool,
}

impl ComputationState {
    pub fn new(root_hash: Hash, halted: bool, uhalted: bool) -> ComputationState {
        ComputationState {
            root_hash,
            halted,
            uhalted,
        }
    }

    pub async fn from_current_machine_state(
        machine: std::sync::Arc<std::sync::Mutex<JsonRpcCartesiMachineClient>>,
    ) -> ComputationState {
        let root_hash = Hash::from_digest(machine.lock().unwrap().get_root_hash().await.unwrap());
        let halted = machine.lock().unwrap().read_iflags_h().await.unwrap();
        let unhalted = machine
            .lock()
            .unwrap()
            .read_uarch_halt_flag()
            .await
            .unwrap();
        ComputationState::new(root_hash, halted, unhalted)
    }
}

impl std::fmt::Display for ComputationState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{root_hash = {:?}, halted = {}, uhalted = {}}}",
            self.root_hash, self.halted, self.uhalted
        )
    }
}
