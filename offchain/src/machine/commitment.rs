use crate::arena::{Hash, CommitmentProof};

pub struct ComputationCommitment {
    pub root_hash: Hash,
    pub implicit_hash: Hash,
}

impl ComputationCommitment {
    pub fn last(&self) -> (Hash, CommitmentProof) {
        (Hash::default(), CommitmentProof::default())
    }
    
    pub fn chidlren(&self, node: Hash) -> Option<(Hash, Hash)> {
        Some((Hash::default(), Hash::default()))
    }

    pub fn prove_leaf(&self, leaf_position: u128) -> (Hash, CommitmentProof) {
        (Hash::default(), CommitmentProof::default())
    }
}