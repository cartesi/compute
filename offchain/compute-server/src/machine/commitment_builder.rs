use std::{
    error::Error,
    sync::Arc,
    collections::HashMap,
};

use tokio::sync::Mutex;
use async_trait::async_trait;

use crate::{
    merkle::{Hash, MerkleBuilder},
    machine::{
        constants,
        MachineRpc,
        MachineCommitment,
        build_machine_commitment,
    }
};

#[async_trait]
pub trait MachineCommitmentBuilder {
    async fn build_commitment(
        &mut self,
        base_cycle: u64,
        level: u64,
    ) -> Result<MachineCommitment, Box<dyn Error>>;
}

pub struct CachingMachineCommitmentBuilder {
    machine: Arc<Mutex<MachineRpc>>,
    commitments: HashMap<u64, HashMap<u64, MachineCommitment>>,
}

impl CachingMachineCommitmentBuilder {
    pub fn new(machine: Arc<Mutex<MachineRpc>>) -> Self {
        CachingMachineCommitmentBuilder { 
            machine: machine,
            commitments: HashMap::new(),
        }
    }
}

#[async_trait]
impl MachineCommitmentBuilder for CachingMachineCommitmentBuilder {
    async fn build_commitment(
        &mut self,
        base_cycle: u64,
        level: u64,
    ) -> Result<MachineCommitment, Box<dyn Error>> {
        assert!(level <= constants::LEVELS);
        
        if !self.commitments.contains_key(&level) {
            self.commitments.insert(level, HashMap::new());
        } else if self.commitments[&level].contains_key(&base_cycle) {
            return Ok(self.commitments[&level][&base_cycle].clone());
        }

        let l = constants::LEVELS - level + 1;
        let log2_stride = constants::LOG2_STEP[l as usize];
        let log2_stride_count = constants::HEIGHTS[l as usize];        
        let commitment = build_machine_commitment(
            self.machine.clone(),
            base_cycle,
            log2_stride,
            log2_stride_count
        ).await?;
        
        self.commitments
            .entry(level)
            .or_insert_with(HashMap::new)
            .insert(base_cycle, commitment.clone());
        
        Ok(commitment)
    }
}

pub struct FakeMachineCommitmentBuilder {
    initial_hash: Hash,
    second_state: Option<Hash>,
}

impl FakeMachineCommitmentBuilder {
    pub fn new(initial_hash: Hash, second_state: Option<Hash>) -> Self {
        FakeMachineCommitmentBuilder {
            initial_hash,
            second_state,
        }
    }
}

#[async_trait]
impl MachineCommitmentBuilder for FakeMachineCommitmentBuilder {
    async fn build_commitment(
        &mut self,
        base_cycle: u64,
        level: u64,
    ) -> Result<MachineCommitment, Box<dyn Error>> {
        let mut merkle_builder = MerkleBuilder::new();
        let level = constants::LEVELS - level + 1;
        if constants::LOG2_STEP[level as usize] == 0 && self.second_state.is_some() {
            merkle_builder.add(self.second_state.clone().unwrap(), None);
            merkle_builder.add(
                Hash::default(),
                Some((1 << constants::HEIGHTS[level as usize]) - 1),
            );
        } else {
            merkle_builder.add(
                Hash::default(),
                Some(1 << constants::HEIGHTS[level as usize]),
            );
        }

        let merkle = merkle_builder.build();

        Ok(MachineCommitment{
            implicit_hash: self.initial_hash,
            merkle: Arc::new(merkle),
        })
    }
}
