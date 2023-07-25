use super::result::ComputationResult;
use utils::arithmetic;
use jsonrpc_cartesi_machine::{JsonRpcCartesiMachineClient, MachineRuntimeConfig};
use std::sync::{Arc, Mutex};
pub struct Machine {
    pub machine: Arc<Mutex<JsonRpcCartesiMachineClient>>,
    cycle: u64,
    ucycle: u64,
    base_cycle: u64,
}

impl Machine {
    pub async fn new_from_path(path: &str) -> Machine {
        let url = "http://0.0.0.0:7001".to_string();

        let machine = Arc::new(Mutex::new(JsonRpcCartesiMachineClient::new(url).await.unwrap()));
        machine.lock().unwrap().load_machine(path, &MachineRuntimeConfig::default());
        let start_cycle = machine.lock().unwrap().get_csr_address("mcycle".to_string()).await.unwrap();

        // Machine can never be advanced on the micro arch.
        // Validators must verify this first
        //assert_eq!(machine.lock().unwrap().get_csr_address("uarch_cycle".to_string()).await.unwrap(), 0);

        Machine {
            machine,
            cycle: 0,
            ucycle: 0,
            base_cycle: start_cycle,
        }
    }

    pub async fn result(&self) -> ComputationResult {
        ComputationResult::from_current_machine_state(Arc::clone(&self.machine)).await
    }

    pub fn advance(&mut self, cycle: u64) {
        assert!(self.cycle <= cycle);
        self.machine.lock().unwrap().run(self.base_cycle + cycle);
        self.cycle = cycle;
    }

    pub fn uadvance(&mut self, ucycle: u64) {
        assert!(arithmetic::ulte(self.ucycle, ucycle), "{}", format!("{}, {}", self.ucycle, ucycle));
        self.machine.lock().unwrap().run_uarch(ucycle);
        self.ucycle = ucycle;
    }

    pub fn ureset(&mut self) {
        self.machine.lock().unwrap().reset_uarch_state();
        self.cycle += 1;
        self.ucycle = 0;
    }
}
