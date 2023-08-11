use crate::arena::{Hash, Proof};

pub struct ComputationCommitment {
    pub root: Hash,
}

impl ComputationCommitment {
    pub fn last(&self) -> (Hash, Proof) {
        (Hash::default(), Proof::default())
    }
    
    pub fn chidlren(&self, node: Hash) -> Option<(Hash, Hash)> {
        Some((Hash::default(), Hash::default()))
    }

    pub fn prove_leaf(&self, leaf_position: u64) -> (Hash, Proof) {
        (0, Proof::default())
    }
}