use cryptography::merkle_builder::MerkleBuilder;
use crate::constants;
use super::{interval::Interval, machine::Machine};
use ruint::Uint;
use cryptography::merkle_tree::MerkleTree;

async fn build_small_machine_commitment(interval: Interval, machine: std::sync::Arc<std::sync::Mutex<Machine>>) -> MerkleTree {
    let mut outer_builder = MerkleBuilder::new();

    for stride_counter in interval.iter(interval.log2_stride_count - constants::A) {
        let mut inner_builder = MerkleBuilder::new();

        for ucycles_stride_counter in interval.ucycles_in_cycle() {
            let ucycle = ucycles_stride_counter.cycle();
        let remaining_strides = ucycles_stride_counter.remaining_strides();
            machine.lock().unwrap().uadvance(ucycle as u64);
            let state = machine.lock().unwrap().result().await;

            if !state.halted {
                inner_builder.add(state.state, None);
            } else {
                inner_builder.add(state.state, Some(remaining_strides as u64));
                break;
            }
        }

        machine.lock().unwrap().uadvance(interval.total_ucycles_in_cycle() as u64);
        machine.lock().unwrap().ureset();
        let state = machine.lock().unwrap().result().await;
        inner_builder.add(state.state, None);

        if !state.halted {
            outer_builder.add(inner_builder.build().root_hash, None);
        } else {
            let remaining_big_strides = stride_counter.remaining_strides();
            outer_builder.add(inner_builder.build().root_hash, Some(remaining_big_strides as u64));
            break;
        }
    }

    outer_builder.build()
}

async fn build_big_machine_commitment(interval: Interval, machine: std::sync::Arc<std::sync::Mutex<Machine>>) -> MerkleTree {
    let mut builder = MerkleBuilder::new();

    for stride_counter in interval.iter(interval.log2_stride_count) {
        let cycle = stride_counter.cycle();
        let remaining_strides = stride_counter.remaining_strides();

        let mut machine = machine.lock().unwrap();
        machine.advance(cycle as u64);
        let state = machine.result().await;

        if !state.halted {
            builder.add(state.state, None);
        } else {
            builder.add(state.state, Some(remaining_strides as u64 + 1));
            break;
        }
    }

    builder.build()
}

async fn build_commitment(interval: Interval, path: &str) -> MerkleTree {
    let machine = std::sync::Arc::new(std::sync::Mutex::new(Machine::new_from_path(path).await));
    if interval.log2_stride >= constants::A {
        build_big_machine_commitment(interval, machine).await
    } else {
        build_small_machine_commitment(interval, machine).await
    }
}

pub async fn commitment_execution() {
    let mc = Uint::<256, 4>::from(0);
    let i = Interval::new(mc, 0, 64);
    let path = "program/simple-program";

    let tree = build_commitment(i, path).await;
    println!("{:?}", tree.root_hash);
}
