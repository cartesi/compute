use std::error::Error;

use async_trait::async_trait;

use crate::{
    merkle::{Hash, MerkleTree},
};

pub trait MachineManager {
    
}

#[async_trait]
pub trait CommitmentBuilder <MM: MachineManager> {
    async fn build_commitment(
        &mut self,
        base_cycle: u64,
        log2_stride: u32,
        log2_stride_count: u8,
    ) -> Result<(Hash, MerkleTree), Box<dyn Error>>;
}