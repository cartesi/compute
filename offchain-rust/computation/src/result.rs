use super::machine::Machine;
use cryptography::hash::Hash;
use jsonrpc_cartesi_machine::JsonRpcCartesiMachineClient;
pub struct ComputationResult {
    pub state: Hash,
    pub halted: bool,
    pub uhalted: bool,
}

impl ComputationResult {
    pub fn new(state: Hash, halted: bool, uhalted: bool) -> ComputationResult {
        ComputationResult {
            state,
            halted,
            uhalted,
        }
    }

    pub async fn from_current_machine_state(machine: std::sync::Arc<std::sync::Mutex<JsonRpcCartesiMachineClient>>) -> ComputationResult {
        let hash = Hash::from_digest_bin(&machine.lock().unwrap().get_root_hash().await.unwrap());
        ComputationResult::new(
            hash,
            machine.lock().unwrap().read_iflags_h().await.unwrap(),
            machine.lock().unwrap().read_uarch_halt_flag().await.unwrap(),
        )
    }
}

impl std::fmt::Display for ComputationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{state = {:?}, halted = {}, uhalted = {}}}",
            self.state,
            self.halted,
            self.uhalted
        )
    }
}

