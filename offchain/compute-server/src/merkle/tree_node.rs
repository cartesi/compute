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

    pub fn children(self: Arc<Self>) -> (Option<Arc<MerkleTreeNode>>, Option<Arc<MerkleTreeNode>>) {
        match (self.left, self.right) {
            (Some(left), Some(right)) => (Some(left.clone()), Some(right.clone())),
            _ => (None, None),
        }
    }

    pub fn join(self: Arc<Self>, other_node: Arc<MerkleTreeNode>) -> Arc<MerkleTreeNode> {
        let mut keccak = Keccak256::new();
        let digest: [u8; 32] = self.digest.into();
        keccak.update(digest);
        let other_digest: [u8; 32] = other_node.digest.into();
        keccak.update(other_digest);
        
        let new_digest: [u8; 32] = keccak.finalize().into();
        Arc::new(MerkleTreeNode {
            digest: Hash::from(new_digest),
            left: Some(self.clone()),
            right: Some(other_node.clone()),
        })
    }
}

impl ToString for MerkleTreeNode {
    fn to_string(&self) -> String {
        self.digest.to_hex()
    }
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
