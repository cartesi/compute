use std::error::Error;

use crate::commitment::ComputationCommitment;

pub struct Machine {
}

impl Machine {
    pub async fn build_commitment() -> Result<ComputationCommitment, Box<dyn Error>> {
        Ok(ComputationCommitment{})
    }
}