use cryptography::{hash::Hash, merkle_builder::MerkleBuilder, merkle_tree::MerkleTree};
use crate::constants;

struct CommitmentBuilder {
    initial_hash: Hash,
    pub second_state: Option<Hash>,
}

impl CommitmentBuilder {
    fn new(&self, initial_hash: Hash, second_state: Option<Hash>) -> CommitmentBuilder {
        CommitmentBuilder { initial_hash, second_state }
    }

    fn build(&self, level: usize) -> MerkleTree {
        let mut builder = MerkleBuilder::new();
        if constants::LOG2STEP[constants::LEVELS - level + 1] == 0 && self.second_state.is_some() {
        builder.add(self.second_state.clone().unwrap(), None);
        builder.add(cryptography::hash::zero_hash(), Some((1 << constants::HEIGHTS[constants::LEVELS - level + 1]) - 1));
    }
    else{
        builder.add(cryptography::hash::zero_hash(), Some(1 << constants::HEIGHTS[constants::LEVELS - level + 1]));
    }
        builder.build(Some(self.initial_hash.clone()))
    }
}