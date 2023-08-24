use std::error::Error;

use async_trait::async_trait;

use crate::{
    arena::{Hash, MachineProof},
    machine::commitment::ComputationCommitment,
};

#[async_trait]
pub trait Machine : Send + Sync {
    async fn build_commitment(
        &self,
        log2_step: u64,
        height: u64,
        arg_1: bool, // TODO
        arg_2: bool, // TODO
    ) -> Result<ComputationCommitment, Box<dyn Error>>;

    async fn get_logs(&self, cycle: u64, ucycle: u64) -> Result<MachineProof, Box<dyn Error>>;
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
            root_hash: Hash::default(),
            implicit_hash: Hash::default(),
        })
    }

    async fn get_logs(&self, cycle: u64, ucycle: u64) -> Result<MachineProof, Box<dyn Error>> {
        Ok(MachineProof::default())
    }
}