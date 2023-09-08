use std::{
    error::Error,
    sync::Arc,
    path::Path,
};

use sha3::{Digest, Keccak256};

use tokio::sync::Mutex;

use cartesi_machine_json_rpc::client::{
    JsonRpcCartesiMachineClient,
    MachineRuntimeConfig,
    AccessLogType,
    AccessType,
    AccessLog,
};

use crate::{
    merkle::Hash,
    commitment::constants,
    utils::arithmetic,
};

#[derive(Debug)]
pub struct MachineState {
    pub root_hash: Hash,
    pub halted: bool,
    pub uhalted: bool,
}

impl std::fmt::Display for MachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{root_hash = {:?}, halted = {}, uhalted = {}}}",
            self.root_hash, self.halted, self.uhalted
        )
    }
}

pub struct RemoteMachine {
    rpc_client: Arc<Mutex<JsonRpcCartesiMachineClient>>,
    root_hash: [u8; 32],
    start_cycle: u64,
    cycle: u64,
    ucycle: u64,
}

impl RemoteMachine {
    pub async fn new(json_rpc_url: &str, snapshot_path: &Path) -> Result<RemoteMachine, Box<dyn Error>> {
        let mut rpc_client = JsonRpcCartesiMachineClient::new(json_rpc_url.to_string()).await?;
        
        let snapshot_path = snapshot_path.to_str().unwrap();
        rpc_client.load_machine(snapshot_path, &MachineRuntimeConfig::default()).await?;
        
        let root_hash = rpc_client.get_root_hash().await?;
        let start_cycle = rpc_client.read_csr("mcycle".to_string()).await?;

        // Machine can never be advanced on the micro arch.
        // Validators must verify this first
        assert_eq!(rpc_client.read_csr("uarch_cycle".to_string()).await?, 0);
        
        Ok(RemoteMachine {
            rpc_client: Arc::new(Mutex::new(rpc_client)),
            start_cycle: start_cycle,
            root_hash: root_hash,
            cycle: 0,
            ucycle: 0,
        })
    }

    pub async fn get_logs(&mut self, cycle: u64, ucycle: u64) -> Result<Vec<u8>, Box<dyn Error>> {
        let rpc_client_lock = self.rpc_client.clone();
        let mut rpc_client = rpc_client_lock.lock().await;
        
        rpc_client.run(cycle).await?;
        rpc_client.run_uarch(ucycle).await?;

        if ucycle as i64 == constants::UARCH_SPAN {
            rpc_client.run_uarch(constants::UARCH_SPAN as u64).await?;
            // TODO: log warn/error or retrn error.
            eprintln!("ureset, not implemented");
        }

        let log_type = AccessLogType {
            annotations: true,
            proofs: true,
        };
        let log = rpc_client.step(&log_type, false).await?;

        Ok(encode_access_log(&log))
    }

    pub async fn run(&mut self, cycle: u64) -> Result<(), Box<dyn Error>> {
        assert!(arithmetic::ulte(self.cycle, cycle));
        
        let physical_cycle = add_and_clamp(self.start_cycle, cycle);
        let rpc_client_lock = self.rpc_client.clone();
        let mut rpc_client = rpc_client_lock.lock().await;
        
        loop {
            let halted = rpc_client.read_iflags_h().await?; 
            if halted {
                break;
            }

            let mcycle = rpc_client.read_csr("mcycle".to_string()).await?;
            if mcycle == physical_cycle {
                break;
            }
        }
        
        self.cycle = cycle;

        Ok(())
    }

    pub async fn run_uarch(&mut self, ucycle: u64) -> Result<(), Box<dyn Error>> {
        assert!(
            arithmetic::ulte(self.ucycle, ucycle),
            "{}",
            format!("{}, {}", self.ucycle, ucycle)
        );

        self.rpc_client
            .clone()
            .lock()
            .await
            .run_uarch(ucycle)
            .await?;
        
        self.ucycle = ucycle;

        Ok(())
    }

    pub async fn increment_uarch(&mut self) -> Result<(), Box<dyn Error>> {
        self.rpc_client
            .clone()
            .lock()
            .await
            .run_uarch(self.ucycle + 1)
            .await?;

        self.ucycle = self.ucycle + 1;

        Ok(())
    }

    pub async fn ureset(&mut self) -> Result<(), Box<dyn Error>> {
        self.rpc_client
            .clone()
            .lock()
            .await
            .reset_uarch_state()
            .await?;

        self.cycle += 1;
        self.ucycle = 0;

        Ok(())
    }

    pub async fn machine_state(&self) -> Result<MachineState, Box<dyn Error>> {
        let rpc_client_lock = self.rpc_client.clone();
        let mut rpc_client = rpc_client_lock.lock().await;

        let root_hash = rpc_client.get_root_hash().await?;
        let halted = rpc_client.read_iflags_h().await?;
        let uhalted = rpc_client.read_uarch_halt_flag().await?;

        Ok(MachineState{
            root_hash: Hash::new(root_hash),
            halted: halted,
            uhalted: uhalted,
        })
    }
}

fn add_and_clamp(x: u64, y: u64) -> u64 {
    if arithmetic::ult(x, arithmetic::max_uint(64) as u64 - y) {
        x + y
    } else {
        arithmetic::max_uint(64) as u64
    }
}

fn encode_access_log(log: &AccessLog) -> Vec<u8> {
    let mut encoded = Vec::new();
    
    for a in log.accesses {
        assert_eq!(a.log2_size, 3);
        if a.r#type == AccessType::Read {
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
    
    encoded.iter().cloned().flatten().collect()
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

