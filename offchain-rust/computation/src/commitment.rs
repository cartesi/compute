use super::{interval::Interval, machine::Machine};
use crate::constants;
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
    interval: Interval,
    machine: std::sync::Arc<std::sync::Mutex<Machine>>,
) -> (cryptography::hash::Hash, MerkleTree) {
    std::sync::Arc::clone(&machine)
        .lock()
        .unwrap()
        .run(interval.base_meta_counter as u64)
        .await;
    let initial_state = machine.lock().unwrap().state().await.root_hash;
    let mut builder = MerkleBuilder::new();
    let instruction_count =
        arithmetic::max_uint(interval.log2_stride_count - constants::LOG2_UARCH_SPAN);
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
    interval: Interval,
    machine: std::sync::Arc<std::sync::Mutex<Machine>>,
) -> (cryptography::hash::Hash, MerkleTree) {
    std::sync::Arc::clone(&machine)
        .lock()
        .unwrap()
        .run(interval.base_meta_counter as u64)
        .await;
    let initial_state = machine.lock().unwrap().state().await.root_hash;
    let mut builder = MerkleBuilder::new();
    let instruction_count =
        arithmetic::max_uint(interval.log2_stride_count - constants::LOG2_UARCH_SPAN);
    let mut instruction = 0;

    while arithmetic::ulte(instruction as u64, instruction_count as u64) {
        let cycle = ((instruction + 1) << (interval.log2_stride - constants::LOG2_UARCH_SPAN));
        std::sync::Arc::clone(&machine)
            .lock()
            .unwrap()
            .run(interval.base_meta_counter as u64 + cycle)
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

async fn build_commitment(
    interval: Interval,
    path: &str,
) -> (cryptography::hash::Hash, MerkleTree) {
    let machine = std::sync::Arc::new(std::sync::Mutex::new(Machine::new_from_path(path).await));
    if interval.log2_stride >= constants::LOG2_UARCH_SPAN {
        assert!(
            interval.log2_stride - constants::LOG2_UARCH_SPAN + interval.log2_stride_count <= 63
        );
        build_big_machine_commitment(interval, machine).await
    } else {
        build_small_machine_commitment(interval, machine).await
    }
}

pub async fn commitment_execution() {
    let i = Interval::new(0, 0, 64);
    let path = "simple-program";
    let tree = build_commitment(i, path).await;

    println!("{:?}  {:?}", tree.0.digest_hex, tree.1.root_hash.digest_hex);
}
