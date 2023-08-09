use crate::arena::{Hash, Proof};

pub struct ComputationCommitment {
    pub root: Hash,
}

impl ComputationCommitment {
    pub fn chidlren(node: Hash) -> Option<(Hash, Hash)> {
        return Some((Hash::default(), Hash::default()))
    }

    pub fn prove_leaf(leaf: Hash) -> (Hash, Proof) {
        return (Hash::default(), Proof::default())
    }
}