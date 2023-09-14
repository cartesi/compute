use std::{
    error::Error,
    sync::Arc,
};

use tokio::sync::{Mutex, MutexGuard};

use crate::{
    merkle::{Hash, MerkleTree, MerkleBuilder},
    commitment::{
        constants,
        RemoteMachine,
        MachineState,
    },
    utils::arithmetic,
};

pub struct MachineCommitment {
    pub initial_hash: Hash,
    pub merkle: Arc<MerkleTree>,
}

pub async fn build_commitment(
    machine: Arc<Mutex<RemoteMachine>>,
    base_cycle: u64,
    log2_stride: u32,
    log2_stride_count: u8,
) -> Result<MachineCommitment, Box<dyn Error>> {
    if log2_stride >= constants::LOG2_UARCH_SPAN {
        assert!(
            log2_stride + log2_stride_count as u32
                <= constants::LOG2_EMULATOR_SPAN + constants::LOG2_UARCH_SPAN
        );
        build_big_machine_commitment(
            machine,
            base_cycle,
            log2_stride,
            log2_stride_count,
        ).await
    } else {
        build_small_machine_commitment(
            machine,
            base_cycle,
            log2_stride_count,
        ).await
    }
}

pub async fn build_big_machine_commitment(
    machine: Arc<Mutex<RemoteMachine>>,
    base_cycle: u64,
    log2_stride: u32,
    log2_stride_count: u8,
) -> Result<MachineCommitment, Box<dyn Error>> {
    let machine_lock = machine.clone();
    let mut machine  = machine_lock.lock().await; 
    
    machine.run(base_cycle).await?;
    let initial_state = machine.machine_state().await?;
    
    let mut builder = MerkleBuilder::new();
    let instruction_count = arithmetic::max_uint(log2_stride_count as u32);
    let mut instruction = 0;
    while arithmetic::ulte(instruction as u64, instruction_count as u64) {
        let cycle = (instruction + 1) << (log2_stride - constants::LOG2_UARCH_SPAN);
        machine.run(base_cycle + cycle).await?;
        let state = machine.machine_state().await?;
        if state.halted {
            builder.add(state.root_hash, None);
            instruction = instruction + 1
        } else {
            builder.add(
                state.root_hash,
                Some(instruction_count as u64 - instruction + 1),
            );
            break;
        }
    }
    let merkle = builder.build();

    Ok(MachineCommitment{
        initial_hash: initial_state.root_hash,
        merkle: Arc::new(merkle),
    })
}

pub async fn build_small_machine_commitment(
    machine: Arc<Mutex<RemoteMachine>>,
    base_cycle: u64,
    log2_stride_count: u8,
) -> Result<MachineCommitment, Box<dyn Error>> {
    let machine_lock = machine.clone();
    let mut machine  = machine_lock.lock().await; 
    
    machine.run(base_cycle).await?;
    let initial_state = machine.machine_state().await?;

    let mut builder = MerkleBuilder::new();
    let instruction_count =
        arithmetic::max_uint(log2_stride_count as u32 - constants::LOG2_UARCH_SPAN);
    let mut instructions = 0;
    loop {
        if !arithmetic::ulte(instructions as u64, instruction_count as u64) {
            break;
        }
        
        builder.add(
            run_uarch_span(&machine).await?.root_hash(),
            None,
        );
        instructions += 1;
        
        let state = machine.machine_state().await?;
        if state.halted {
            builder.add(
                run_uarch_span(&machine).await?.root_hash(),
                Some(instruction_count as u64 - instructions + 1),
            );
            break;
        }
    }
    let merkle = builder.build();

    Ok(MachineCommitment{
        initial_hash: initial_state.root_hash,
        merkle: Arc::new(merkle),
    })
}

async fn run_uarch_span<'a>(machine: &MutexGuard<'a, RemoteMachine>) -> Result<MerkleTree, Box<dyn Error>> {
    assert!(machine.ucycle == 0);

    machine.increment_uarch().await?;
    
    let mut builder = MerkleBuilder::new();
    let mut i = 0;
    let mut state: MachineState;
    loop {
        state = machine.machine_state().await?;
        builder.add(state.root_hash, None);
        
        machine.increment_uarch().await?;
        i += 1;
        
        state = machine.machine_state().await?;
        if state.uhalted {
            break;
        }
    }
    builder.add(state.root_hash, Some((constants::UARCH_SPAN - i) as u64));

    machine.ureset().await?;
    state = machine.machine_state().await?;
    builder.add(state.root_hash, None);

    Ok(builder.build())
}

