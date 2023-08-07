use cryptography::hash::Hash;
use jsonrpc_cartesi_machine::JsonRpcCartesiMachineClient;
#[derive(Debug)]
pub struct ComputationState  {
    pub root_hash: Hash,
    pub halted: bool,
    pub uhalted: bool,
}

impl ComputationState  {
    pub fn new(root_hash: Hash, halted: bool, uhalted: bool) -> ComputationState  {
        ComputationState  {
            root_hash,
            halted,
            uhalted,
        }
    }

    pub async fn from_current_machine_state(machine: std::sync::Arc<std::sync::Mutex<JsonRpcCartesiMachineClient>>) -> ComputationState  {
        let root_hash = Hash::from_digest_bin(&machine.lock().unwrap().get_root_hash().await.unwrap());
        let halted = machine.lock().unwrap().read_iflags_h().await.unwrap();
        let unhalted = machine.lock().unwrap().read_uarch_halt_flag().await.unwrap();
        //println!("from_current_machine_state: {:?}; {:?}; {:?}", &machine.lock().unwrap().get_root_hash().await.unwrap(), halted, unhalted);
        ComputationState::new(
            root_hash,
            halted,
            unhalted,
        )
    }
}

impl std::fmt::Display for ComputationState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{root_hash = {:?}, halted = {}, uhalted = {}}}",
            self.root_hash,
            self.halted,
            self.uhalted
        )
    }
}

