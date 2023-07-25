use crate::hash::Hash;
use crate::merkle_builder::Leaf;
pub struct MerkleTree {
    leafs: Vec<Leaf>,
    pub root_hash: Hash,
    digest_hex: String,
    log2size: u32,
}

impl MerkleTree {
    pub fn new(leafs: Vec<Leaf>, root_hash: Hash, log2size: u32) -> Self {
        MerkleTree {
            leafs,
            root_hash: root_hash.clone(),
            digest_hex: root_hash.digest_hex.clone(),
            log2size,
        }
    }

    pub fn join(&self, other_hash: Hash) -> Hash {
        self.root_hash.join(&other_hash)
    }

    pub fn children(&self) -> (bool, Option<String>, Option<String>) {
        self.root_hash.children()
    }

    pub fn iterated_merkle(&self, level: u32) -> Hash {
        self.root_hash.iterated_merkle(level)
    }
}
