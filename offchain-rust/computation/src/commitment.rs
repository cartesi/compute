use cryptography::merkle_builder::MerkleBuilder;
use crate::constants;
use super::{interval::Interval, machine::Machine};
use ruint::Uint;
use cryptography::merkle_tree::MerkleTree;

async fn build_small_machine_commitment(interval: Interval, machine: &mut Machine) -> MerkleTree {
    let mut outer_builder = MerkleBuilder::new();

    for (_, _, remaining_big_strides) in interval.big_strides() {
        let mut inner_builder = MerkleBuilder::new();

        for (_, ucycle, remaining_strides) in interval.ucycles_in_cycle() {
            machine.uadvance(ucycle);
            let state = machine.result().await;

            if !state.uhalted {
                inner_builder.add(state.state, None);
            } else {
                inner_builder.add_with_strides(state.state, remaining_strides);
                break;
            }
        }

        machine.uadvance(interval.total_ucycles_in_cycle() as u64);
        machine.ureset();
        let state = machine.result().await;
        inner_builder.add(state.state, None);

        if !state.halted {
            outer_builder.add(inner_builder.build(), None);
        } else {
            outer_builder.add_with_strides(inner_builder.build(), remaining_big_strides);
            break;
        }
    }

    outer_builder.build()
}

async fn build_big_machine_commitment(interval: Interval, machine: &mut Machine) -> MerkleTree {
    let mut builder = MerkleBuilder::new();

    for (_, cycle, remaining_strides) in interval.strides() {
        machine.advance(cycle);
        let state = machine.result().await;

        if !state.halted {
            builder.add(state.state, None);
        } else {
            builder.add_with_strides(state.state, remaining_strides + 1);
            break;
        }
    }

    builder.build()
}

async fn build_commitment(interval: Interval, path: &str) -> MerkleTree {
    let mut machine = Machine::new_from_path(path).await;
    if interval.log2_stride >= constants::A {
        build_big_machine_commitment(interval, &mut machine).await
    } else {
        build_small_machine_commitment(interval, &mut machine).await
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
