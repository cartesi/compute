use std::error::Error;

use async_trait::async_trait;

use crate::{
    arena::Hash,
    machine::commitment::ComputationCommitment,
};

#[async_trait]
pub trait Machine {
    async fn build_commitment(
        &self,
        log2_step: u64,
        height: u64,
        arg_1: bool, // TODO
        arg_2: bool, // TODO
    ) -> Result<ComputationCommitment, Box<dyn Error>>;

    async fn initial_hash(&self) -> Result<Hash, Box<dyn Error>>;
}

pub struct MachineJsonRpcClient {
}

#[async_trait]
impl Machine for MachineJsonRpcClient {
    async fn build_commitment(
        &self,
        log2_step: u64,
        height: u64,
        arg_1: bool, // TODO
        arg_2: bool, // TODO
    ) -> Result<ComputationCommitment, Box<dyn Error>> {
        Ok(ComputationCommitment{
            root: Hash::default(),
        })
    }

    async fn initial_hash(&self) -> Result<Hash, Box<dyn Error>> {
       Ok(Hash::default())
    }
}