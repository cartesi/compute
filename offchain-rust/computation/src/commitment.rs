use cryptography::merkle_builder::MerkleBuilder;
use crate::constants;
use super::{interval::Interval, machine::Machine};
use ruint::Uint;
use cryptography::merkle_tree::MerkleTree;

async fn build_small_machine_commitment<'a>(interval: Interval, machine: std::sync::Arc<std::sync::Mutex<Machine>>) -> MerkleTree<'a> {
    let mut outer_builder = MerkleBuilder::new();

    for remaining_big_strides in interval.big_strides().2 {
        let mut inner_builder = MerkleBuilder::new();

        for (ucycle, remaining_strides) in interval.ucycles_in_cycle().1.zip(interval.ucycles_in_cycle().2) {
            machine.lock().unwrap().uadvance(ucycle);
            let state = machine.lock().unwrap().result().await;

            if !state.uhalted {
                inner_builder.add(state.state, None);
            } else {
                inner_builder.add(state.state, remaining_strides);
                break;
            }
        }

        machine.lock().unwrap().uadvance(interval.total_ucycles_in_cycle() as u64);
        machine.lock().unwrap().ureset();
        let state = machine.lock().unwrap().result().await;
        inner_builder.add(state.state, None);

        if !state.halted {
            outer_builder.add(inner_builder.build(), None);
        } else {
            outer_builder.add(inner_builder.build(), remaining_big_strides);
            break;
        }
    }

    outer_builder.build()
}

async fn build_big_machine_commitment<'a>(interval: Interval, machine: std::sync::Arc<std::sync::Mutex<Machine>>) -> MerkleTree<'a> {
    let mut builder = MerkleBuilder::new();

    for (cycle, remaining_strides) in interval.strides() {
        let mut machine = machine.lock().unwrap();
        machine.advance(cycle);
        let state = machine.result().await;

        if !state.halted {
            builder.add(state.state, None);
        } else {
            builder.add(state.state, remaining_strides + 1);
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

#[tokio::main]
async fn main() {
    let mc = Uint::<256, 4>::from(0);
    let i = Interval::new(mc, 0, 64);
    let path = "program/simple-program";

    let tree = build_commitment(i, path).await;
    println!("{:?}", tree.root_hash);
}
