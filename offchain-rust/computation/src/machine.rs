use super::result::ComputationState;
use cryptography::hash;
use utils::arithmetic;
use jsonrpc_cartesi_machine::{JsonRpcCartesiMachineClient, MachineRuntimeConfig};
use std::sync::{Arc, Mutex};
pub struct Machine {
    pub machine: Arc<Mutex<JsonRpcCartesiMachineClient>>,
    cycle: u64,
    pub ucycle: u64,
    base_cycle: u64,
}

impl Machine {
    pub async fn new_from_path(path: &str) -> Machine {
        let url = "http://127.0.0.1:50051".to_string();
        let machine = Arc::new(Mutex::new(
            JsonRpcCartesiMachineClient::new(url).await.unwrap(),
        ));
        machine
            .lock()
            .unwrap()
            .load_machine(path, &MachineRuntimeConfig::default())
            .await
            .unwrap();

        println!(
            "start root {:?}",
            hash::Hash::from_digest_bin(&machine.lock().unwrap().get_root_hash().await.unwrap())
        );

        let start_cycle = machine
            .lock()
            .unwrap()
            .get_csr_address("mcycle".to_string())
            .await
            .unwrap();

        println!("start cycle {:?}", start_cycle);

        // Machine can never be advanced on the micro arch.
        // Validators must verify this first
        assert_eq!(machine.lock().unwrap().get_csr_address("uarch_cycle".to_string()).await.unwrap(), 800);
        Machine {
            machine,
            cycle: 0,
            ucycle: 0,
            base_cycle: start_cycle,
        }
    }

    pub async fn state(&self) -> ComputationState {
        ComputationState::from_current_machine_state(Arc::clone(&self.machine)).await
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
        let physical_cycle = Machine::add_and_clamp(self.base_cycle, cycle);
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
        // this assert panics even in lua version
        //assert!(self.ucycle == utils::arithmetic::max_uint(64) as u64);
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
