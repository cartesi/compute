use crate::hash::Hash;
use crate::merkle_builder::Leaf;
#[derive(Debug)]
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

    pub fn iterated_merkle(&self, level: u32) -> Hash {
        self.root_hash.iterated_merkle(level)
    }

    pub fn last(&self) -> (Hash, Vec<Hash>) {
        (Hash::default(), Vec::<Hash>::default())
    }
    
    pub fn children(&self, node: Hash) -> (Option<String>, Option<String>) {
        self.leafs.iter().find_map(|leaf| {
            if leaf.hash == node {
                Some(leaf.hash.children())
            } else {
                None
            }
        }).unwrap_or((None, None))
    }

    pub fn prove_leaf(&self, leaf_position: u128) -> (Hash, Vec<Hash>) {
        (Hash::default(), Vec::<Hash>::default())
    }
}
