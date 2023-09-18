use std::sync::Arc;

use sha3::{Digest, Keccak256};

use crate::merkle::Hash;

#[derive(Debug)]
pub struct MerkleTreeNode {
    pub digest: Hash,
    left: Option<Arc<MerkleTreeNode>>,
    right: Option<Arc<MerkleTreeNode>>,
}

impl MerkleTreeNode {
    pub fn new(digest: Hash) -> Self {
        MerkleTreeNode {
            digest: digest,
            left: None,
            right: None,
        }
    }

    fn from_data(data:Vec<u8>) -> MerkleTreeNode {
        let mut keccak = Keccak256::new();
        keccak.update(&data);
        let digest: [u8; 32] = keccak.finalize().into();
        MerkleTreeNode::new(Hash::from(digest))
    }

    pub fn from_digest_hex(digest_hex: &str) -> MerkleTreeNode {
        assert!(digest_hex.len() == 66);
        let mut data = [0u8; 32];
        hex::decode_to_slice(&digest_hex, &mut data as &mut [u8]).unwrap();
        MerkleTreeNode::new(Hash::from(data))
    }

    pub fn children(&self) -> Option<(Arc<MerkleTreeNode>, Arc<MerkleTreeNode>)> {
        if self.left.is_some() && self.right.is_some() {
            Some((self.left.unwrap().clone(), self.right.unwrap().clone()))
        } else {
            None
        }
    }

    pub fn join(self: Arc<Self>, other: Arc<MerkleTreeNode>) -> Arc<MerkleTreeNode> {
        Arc::new(MerkleTreeNode {
            digest: join_merkle_tree_node_digests(self.digest, other.digest),
            left: Some(self.clone()),
            right: Some(other.clone()),
        })
    }
}

impl ToString for MerkleTreeNode {
    fn to_string(&self) -> String {
        self.digest.to_hex()
    }
}

pub fn join_merkle_tree_node_digests(digest_1: Hash, digest_2: Hash) -> Hash {
    let mut keccak = Keccak256::new();
    
    let digest_1: [u8; 32] = digest_1.into();
    keccak.update(digest_1);
    
    let digest_2: [u8; 32] = digest_2.into();
    keccak.update(digest_2);

    let digest: [u8; 32] = keccak.finalize().into();
    Hash::from(digest)
}

fn zero_bytes32() -> [u8; 32] {
    let mut data = [0u8; 32];
    hex::decode_to_slice(
        "0000000000000000000000000000000000000000000000000000000000000000",
        &mut data as &mut [u8],
    )
    .unwrap();
    data
}
