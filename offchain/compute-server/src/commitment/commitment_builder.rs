use std::{
    error::Error,
    sync::Arc,
    collections::HashMap,
};

use tokio::sync::Mutex;
use async_trait::async_trait;

use crate::{
    merkle::{Hash, MerkleBuilder},
    commitment::{
        constants,
        RemoteMachine,
        MachineCommitment,
        build_commitment,
    }
};

#[async_trait]
pub trait CommitmentBuilder {
    async fn build_commitment(
        &mut self,
        base_cycle: u64,
        level: usize,
    ) -> Result<MachineCommitment, Box<dyn Error>>;
}

pub struct CachingCommitmentBuilder {
    machine: Arc<Mutex<RemoteMachine>>,
    commitments: HashMap<usize, HashMap<usize, MachineCommitment>>,
}

impl CachingCommitmentBuilder {
    pub fn new(machine: Arc<Mutex<RemoteMachine>>) -> Self {
        CachingCommitmentBuilder { 
            machine: machine,
            commitments: HashMap::new(),
        }
    }
}

#[async_trait]
impl CommitmentBuilder for CachingCommitmentBuilder {
    async fn build_commitment(
        &mut self,
        base_cycle: u64,
        level: usize,
    ) -> Result<MachineCommitment, Box<dyn Error>> {
        assert!(level <= constants::LEVELS);
        
        if !self.commitments.contains_key(&level) {
            self.commitments.insert(level, HashMap::new());
        } else if self.commitments[&level].contains_key(&(base_cycle as usize)) {
            return Ok(self.commitments[&level][&(base_cycle as usize)]);
        }

        let l = (constants::LEVELS - level + 1) as usize;
        let log2_stride = constants::LOG2STEP[l];
        let log2_stride_count = constants::HEIGHTS[l];        
        let commitment = build_commitment(self.machine, base_cycle, log2_stride, log2_stride_count).await?;
        
        self.commitments
            .entry(level)
            .or_insert_with(HashMap::new)
            .insert(base_cycle as usize, commitment);
        
        Ok(commitment)
    }
}

pub struct FakeCommitmentBuilder {
    initial_hash: Hash,
    second_state: Option<Hash>,
}

impl FakeCommitmentBuilder {
    pub fn new(initial_hash: Hash, second_state: Option<Hash>) -> Self {
        FakeCommitmentBuilder {
            initial_hash,
            second_state,
        }
    }
}

#[async_trait]
impl CommitmentBuilder for FakeCommitmentBuilder {
    async fn build_commitment(
        &mut self,
        base_cycle: u64,
        level: usize,
    ) -> Result<MachineCommitment, Box<dyn Error>> {
        let mut merkle_builder = MerkleBuilder::new();
        if constants::LOG2STEP[constants::LEVELS - level + 1] == 0 && self.second_state.is_some() {
            merkle_builder.add(self.second_state.clone().unwrap(), None);
            merkle_builder.add(
                Hash::default(),
                Some((1 << constants::HEIGHTS[constants::LEVELS - level + 1]) - 1),
            );
        } else {
            merkle_builder.add(
                Hash::default(),
                Some(1 << constants::HEIGHTS[constants::LEVELS - level + 1]),
            );
        }

        let merkle = merkle_builder.build();

        Ok(MachineCommitment{
            initial_hash: self.initial_hash,
            merkle: Arc::new(merkle),
        })
    }
}
