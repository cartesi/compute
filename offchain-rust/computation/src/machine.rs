use super::result::ComputationResult;
use utils::arithmetic;
use jsonrpc_cartesi_machine::{JsonRpcCartesiMachineClient, MachineRuntimeConfig};

pub struct Machine {
    pub machine: JsonRpcCartesiMachineClient,
    cycle: u64,
    ucycle: u64,
    base_cycle: u64,
}

impl Machine {
    pub async fn new_from_path(path: &str) -> Machine {
        let machine = JsonRpcCartesiMachineClient::new().await.unwrap();
        machine.load_machine(path, MachineRuntimeConfig::default());
        let start_cycle = machine.get_csr_address("mcycle".to_string()).await;

        // Machine can never be advanced on the micro arch.
        // Validators must verify this first
        assert_eq!(machine.read_uarch_cycle().await, 0);

        Machine {
            machine,
            cycle: 0,
            ucycle: 0,
            base_cycle: start_cycle,
        }
    }

    pub async fn result(&self) -> ComputationResult {
        ComputationResult::from_current_machine_state(&self.machine).await
    }

    pub fn advance(&mut self, cycle: u64) {
        assert!(self.cycle <= cycle);
        self.machine.run(self.base_cycle + cycle);
        self.cycle = cycle;
    }

    pub fn uadvance(&mut self, ucycle: u64) {
        assert!(arithmetic::ulte(self.ucycle, ucycle), "{}", format!("{}, {}", self.ucycle, ucycle));
        self.machine.run_uarch(ucycle);
        self.ucycle = ucycle;
    }

    pub fn ureset(&mut self) {
        self.machine.reset_uarch_state();
        self.cycle += 1;
        self.ucycle = 0;
    }
}
