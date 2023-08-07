use super::{interval::Interval, machine::Machine};
use crate::constants;
use cryptography::merkle_builder::MerkleBuilder;
use cryptography::merkle_tree::MerkleTree;
use ruint::Uint;
use utils::arithmetic;

async fn run_uarch_span(machine: std::sync::Arc<std::sync::Mutex<Machine>>) -> MerkleTree{
    assert!(machine.lock().unwrap().ucycle == 0);
    machine.lock().unwrap().increment_uarch();
    let mut builder = MerkleBuilder::new();
    let mut i = 0;
    loop {
        builder.add(machine.lock().unwrap().state().await.root_hash, None);
        machine.lock().unwrap().increment_uarch();
        i += 1;
        if machine.lock().unwrap().state().await.uhalted {
            break;
        }
    }

    builder.add(machine.lock().unwrap().state().await.root_hash, Some((constants::UARCH_SPAN - i) as u64));

    machine.lock().unwrap().ureset();
    builder.add(machine.lock().unwrap().state().await.root_hash, None);

    return builder.build()

}

async fn build_small_machine_commitment(
    interval: Interval,
    machine: std::sync::Arc<std::sync::Mutex<Machine>>,
) -> (cryptography::hash::Hash, MerkleTree) {
    std::sync::Arc::clone(&machine).lock().unwrap().run(interval.base_meta_counter.to::<u64>()).await;
    let initial_state = machine.lock().unwrap().state().await.root_hash;
    let mut builder = MerkleBuilder::new();
    let instruction_count = arithmetic::max_uint(interval.log2_stride_count - constants::LOG2_UARCH_SPAN);
    let mut instructions = 0;

    while arithmetic::ulte(instructions as u64, instruction_count as u64) {
        builder.add(run_uarch_span(std::sync::Arc::clone(&machine)).await.root_hash, None);
        instructions += 1;
    
        if machine.lock().unwrap().state().await.halted {
            builder.add(run_uarch_span(machine).await.root_hash, Some(instruction_count as u64 - instructions + 1));
            break;
        }
    }
    return (initial_state, builder.build())
    /*let mut outer_builder = MerkleBuilder::new();
        for stride_counter in interval.big_strides_iter(){
        //println!("stride_counter: {}", stride_counter);
        interval._build_iter(interval.log2_stride_count - constants::A);
        let mut inner_builder = MerkleBuilder::new();

         for remaining_ucycles_in_cycle in interval.ucycles_in_cycle_iter() {

            let ucycle = remaining_ucycles_in_cycle.0;
            let remaining_strides: i64 = remaining_ucycles_in_cycle.1 as i64;
            //println!("ucycle {}", ucycle);
            //println!("remaining_strides {}", remaining_strides);

            machine.lock().unwrap().uadvance(ucycle as u64).await;

            let state = machine.lock().unwrap().result().await;
            if !state.uhalted {
                println!("halted {:?} ucycle {:?}", state.state.clone(), ucycle);

                inner_builder.add(state.state.clone(), None);
            } else {

                println!("unhalted {:?} ucycle {:?}", state.state.clone(), ucycle);

                inner_builder.add(state.state.clone(), Some(remaining_strides as u64));
                break;
            }
        }

        machine
            .lock()
            .unwrap()
            .uadvance(interval.total_ucycles_in_cycle() as u64)
            .await;

        let state = machine.lock().unwrap().result().await;
        inner_builder.add(state.state.clone(), None);
        if !state.halted {
            outer_builder.add(inner_builder.build().root_hash, None);
        } else {
            let remaining_big_strides = stride_counter;
            outer_builder.add(
                inner_builder.build().root_hash,
                Some(remaining_big_strides as u64),
            );
            break;
        }
    }
    outer_builder.build()*/
}

async fn build_big_machine_commitment(
    interval: Interval,
    machine: std::sync::Arc<std::sync::Mutex<Machine>>,
) -> (cryptography::hash::Hash, MerkleTree) {
    std::sync::Arc::clone(&machine).lock().unwrap().run(interval.base_meta_counter.to::<u64>()).await;
    let initial_state = machine.lock().unwrap().state().await.root_hash;

    let mut builder = MerkleBuilder::new();
    let instruction_count = arithmetic::max_uint(interval.log2_stride_count - constants::LOG2_UARCH_SPAN);
    let mut instruction = 0;

    while arithmetic::ulte(instruction as u64, instruction_count as u64) {
        let cycle = ((instruction + 1) << (interval.log2_stride - constants::LOG2_UARCH_SPAN));
        std::sync::Arc::clone(&machine).lock().unwrap().run(interval.base_meta_counter.to::<u64>() + cycle).await;
        if !machine.lock().unwrap().state().await.halted {
            builder.add(machine.lock().unwrap().state().await.root_hash, None);
            instruction = instruction + 1
        } else {
            builder.add(machine.lock().unwrap().state().await.root_hash, Some(instruction_count as u64 - instruction + 1));
            break
        }
    }
    return (initial_state, builder.build())

    /*let mut builder = MerkleBuilder::new();

    for stride_counter in interval.iter(interval.log2_stride_count as i32) {
        let cycle = stride_counter.cycle();
        let remaining_strides = stride_counter.remaining_strides();

        let mut machine = machine.lock().unwrap();
        machine.run(cycle as u64).await;
        let state = machine.state().await;

        if !state.halted {
            builder.add(state.root_hash, None);
        } else {
            builder.add(state.root_hash, Some(remaining_strides as u64 + 1));
            break;
        }
    }

    builder.build()*/
}

async fn build_commitment(interval: Interval, path: &str) -> (cryptography::hash::Hash, MerkleTree) {
    let machine = std::sync::Arc::new(std::sync::Mutex::new(Machine::new_from_path(path).await));
    if interval.log2_stride >= constants::LOG2_UARCH_SPAN {
        assert!(interval.log2_stride - constants::LOG2_UARCH_SPAN + interval.log2_stride_count <= 63);
        build_big_machine_commitment(interval, machine).await
    } else {
        build_small_machine_commitment(interval, machine).await
    }
}

pub async fn commitment_execution() {
    let mc = Uint::<256, 4>::from(0);
    let i = Interval::new(mc, 0, 64);
    let path = "program/simple-program";
    println!("before build commitment");

    let tree = build_commitment(i, path).await;

    println!("{:?}", tree.1.root_hash);
}
