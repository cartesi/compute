use super::machine::Machine;
use crate::constants;
use cryptography::hash::Hash;
use cryptography::merkle_builder::MerkleBuilder;
use cryptography::merkle_tree::MerkleTree;
use utils::arithmetic;

async fn run_uarch_span(machine: std::sync::Arc<std::sync::Mutex<Machine>>) -> MerkleTree {
    assert!(machine.lock().unwrap().ucycle == 0);
    machine.lock().unwrap().increment_uarch().await;
    let mut builder = MerkleBuilder::new();
    let mut i = 0;
    loop {
        builder.add(machine.lock().unwrap().state().await.root_hash, None);
        machine.lock().unwrap().increment_uarch().await;
        i += 1;
        if machine.lock().unwrap().state().await.uhalted {
            break;
        }
    }

    builder.add(
        machine.lock().unwrap().state().await.root_hash,
        Some((constants::UARCH_SPAN - i) as u64),
    );

    machine.lock().unwrap().ureset().await;
    builder.add(machine.lock().unwrap().state().await.root_hash, None);
    return builder.build();
}

async fn build_small_machine_commitment(
    base_cycle: u64,
    log2_stride_count: u8,
    machine: std::sync::Arc<std::sync::Mutex<Machine>>,
) -> (cryptography::hash::Hash, MerkleTree) {
    std::sync::Arc::clone(&machine)
        .lock()
        .unwrap()
        .run(base_cycle)
        .await;
    let initial_state = machine.lock().unwrap().state().await.root_hash;
    let mut builder = MerkleBuilder::new();
    let instruction_count =
        arithmetic::max_uint(log2_stride_count as u32 - constants::LOG2_UARCH_SPAN);
    let mut instructions = 0;

    loop {
        if !arithmetic::ulte(instructions as u64, instruction_count as u64) {
            break;
        }
        builder.add(
            run_uarch_span(std::sync::Arc::clone(&machine))
                .await
                .root_hash,
            None,
        );
        instructions += 1;
        if std::sync::Arc::clone(&machine)
            .lock()
            .unwrap()
            .state()
            .await
            .halted
        {
            builder.add(
                run_uarch_span(std::sync::Arc::clone(&machine))
                    .await
                    .root_hash,
                Some(instruction_count as u64 - instructions + 1),
            );
            break;
        }
    }
    return (initial_state, builder.build());
}

async fn build_big_machine_commitment(
    base_cycle: u64,
    log2_stride: u32,
    log2_stride_count: u8,
    machine: std::sync::Arc<std::sync::Mutex<Machine>>,
) -> (cryptography::hash::Hash, MerkleTree) {
    std::sync::Arc::clone(&machine)
        .lock()
        .unwrap()
        .run(base_cycle)
        .await;
    let initial_state = machine.lock().unwrap().state().await.root_hash;
    let mut builder = MerkleBuilder::new();
    let instruction_count = arithmetic::max_uint(log2_stride_count as u32);
    let mut instruction = 0;

    while arithmetic::ulte(instruction as u64, instruction_count as u64) {
        let cycle = (instruction + 1) << (log2_stride - constants::LOG2_UARCH_SPAN);
        std::sync::Arc::clone(&machine)
            .lock()
            .unwrap()
            .run(base_cycle + cycle)
            .await;
        if !machine.lock().unwrap().state().await.halted {
            builder.add(machine.lock().unwrap().state().await.root_hash, None);
            instruction = instruction + 1
        } else {
            builder.add(
                machine.lock().unwrap().state().await.root_hash,
                Some(instruction_count as u64 - instruction + 1),
            );
            break;
        }
    }
    return (initial_state, builder.build());
}

pub struct MachineJsonRpcClient {
    pub machine: std::sync::Arc<std::sync::Mutex<Machine>>,
}

impl MachineJsonRpcClient {
    pub async fn new(url: &str, machine_path: &str) -> Self {
        MachineJsonRpcClient {
            machine: std::sync::Arc::new(std::sync::Mutex::new(
                Machine::new_from_path(url, machine_path).await,
            )),
        }
    }

    pub async fn build_commitment(
        &self,
        base_cycle: u64,
        log2_stride: u32,
        log2_stride_count: u8,
    ) -> (cryptography::hash::Hash, MerkleTree) {
        if log2_stride >= constants::LOG2_UARCH_SPAN {
            assert!(
                log2_stride + log2_stride_count as u32
                    <= constants::LOG2_EMULATOR_SPAN + constants::LOG2_UARCH_SPAN
            );
            build_big_machine_commitment(
                base_cycle,
                log2_stride,
                log2_stride_count,
                std::sync::Arc::clone(&self.machine),
            )
            .await
        } else {
            build_small_machine_commitment(
                base_cycle,
                log2_stride_count,
                std::sync::Arc::clone(&self.machine),
            )
            .await
        }
    }

    pub async fn initial_hash(&self) -> Hash {
        self.machine.lock().unwrap().initial_hash.clone()
    }
}

pub async fn commitment_execution() {
    let path = "simple-program";
    let url = "http://127.0.0.1:50051";
    let machine = MachineJsonRpcClient::new(url, path).await;
    let tree = machine.build_commitment(0, 0, 64).await;
    println!("{:?}  {:?}", hex::encode(tree.0.digest), hex::encode(tree.1.root_hash.digest));
}