use cryptography::hash::Hash;
use crate::constants;

struct CommitmentBuilder;

impl CommitmentBuilder {
    fn build(&self, level: usize) -> Hash {
        let commitment = cryptography::hash::zero_hash().iterated_merkle(constants::HEIGHTS[level] as u32);
        commitment
    }
}