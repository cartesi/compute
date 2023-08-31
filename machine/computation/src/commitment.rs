use super::machine::Machine;
use crate::constants;
use cryptography::hash::Hash;
use cryptography::merkle_builder::MerkleBuilder;
use cryptography::merkle_tree::MerkleTree;
use std::collections::HashMap;
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
    return builder.build(None);
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
    return (initial_state.clone(), builder.build(Some(initial_state)));
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
    return (initial_state.clone(), builder.build(Some(initial_state)));
}

pub struct FatMachineClient {
    pub machine: std::sync::Arc<std::sync::Mutex<Machine>>,
}

impl FatMachineClient {
    pub async fn new(url: &str, machine_path: &str) -> Self {
        FatMachineClient {
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

struct CommitmentBuilder {
    machine_path: String,
    url: String,
    pub commitments: HashMap<usize, HashMap<usize, Hash>>,
}

impl CommitmentBuilder {
    pub fn new(url: String, machine_path: String) -> Self {
        CommitmentBuilder {
            machine_path,
            url,
            commitments: HashMap::new(),
        }
    }

    pub async fn build(&mut self, base_cycle: u64, level: usize) -> Hash {
        assert!(level <= constants::LEVELS);
        if !self.commitments.contains_key(&level) {
            self.commitments.insert(level, HashMap::new());
        } else if self.commitments[&level].contains_key(&(base_cycle as usize)) {
            return self.commitments[&level][&(base_cycle as usize)].clone();
        }

        let l = (constants::LEVELS - level + 1) as usize;
        let log2_stride = constants::LOG2STEP[l];
        let log2_stride_count = constants::HEIGHTS[l];
        let machine = FatMachineClient::new(&self.url, &self.machine_path).await;
        let (_, commitment) = machine
            .build_commitment(base_cycle, log2_stride, log2_stride_count)
            .await;
        self.commitments
            .entry(level)
            .or_insert_with(HashMap::new)
            .insert(base_cycle as usize, commitment.root_hash.clone());
        commitment.root_hash
    }
}
