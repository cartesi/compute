use super::machine::Machine;
use cryptography::hash::Hash;
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

    pub async fn from_current_machine_state(machine: &Machine) -> ComputationResult {
        let hash = Hash::from_digest_bin(machine.machine.get_root_hash().await.unwrap().as_vec());
        ComputationResult::new(
            hash,
            machine.machine.read_iflags_h().await.unwrap(),
            machine.machine.read_uarch_halt_flag().await,
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

