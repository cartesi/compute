use std::sync::Arc;

use crate::merkle::{Hash, MerkleTreeNode};

#[derive(Debug)]
pub struct MerkleTreeLeaf {
    pub node: Arc<MerkleTreeNode>,
    pub accumulated_count: u64,
    pub log2_size: Option<u32>
}

pub struct MerkleTreeProof {
    pub leaf: Hash,
    pub data: Vec<Hash>,
}

#[derive(Debug)]
pub struct MerkleTree {
    root: Arc<MerkleTreeNode>,
    leafs: Vec<Arc<MerkleTreeLeaf>>,
}

impl MerkleTree {
    pub fn new(root: Arc<MerkleTreeNode>, leafs: Vec<Arc<MerkleTreeLeaf>>) -> Self {
        MerkleTree {
            root: root,
            leafs: leafs,
        }
    }

    pub fn root_hash(&self) -> Hash {
        self.root.digest
    }

    pub fn children(&self) -> (Option<Arc<MerkleTreeNode>>, Option<Arc<MerkleTreeNode>>) {
        self.root.clone().children()
    }

    pub fn join(&self, other_hash: Arc<MerkleTreeNode>) -> Arc<MerkleTreeNode> {
        self.root.clone().join(other_hash)
    }

    pub fn prove_leaf(
        &self,
        log2_size: u32,
        index: u64
    ) -> MerkleTreeProof {
        let mut height = log2_size;
        if let Some(leaf) = self.leafs.get(0) {
            if let Some(log2size) = leaf.log2_size {
                height = log2size + log2_size;
            }
        }
        assert!((index >> height) == 0);

        let mut proof = MerkleTreeProof {
            leaf: Hash::default(),
            data: Vec::new(),
        };
        self.generate_proof(&mut proof, self.root, height, index);

        proof
    }

    fn generate_proof(
        &self, 
        proof: &mut MerkleTreeProof,
        root: Arc<MerkleTreeNode>,
        height: u32,
        include_index: u64
    ) {    
        if height == 0 {
            proof.leaf = root.digest;
            return;
        }

        let new_height = height - 1;
        let (left, right) = root.children();
        assert!(left.is_some() && right.is_some());

        if (include_index >> new_height) & 1 == 0 {
            let left = left.unwrap();
            self.generate_proof(
                proof,
                left,
                new_height,
                include_index,
            );
            proof.data.push(left.digest);
        } else {
            let right = right.unwrap();
            self.generate_proof(
                proof,
                right,
                new_height,
                include_index,
            );
            proof.data.push(right.digest);
        }
    }

    pub fn last(&self) -> MerkleTreeProof {
        let mut proof = Vec::new();
        let (mut left, mut right) = self.root.children();
        let mut old_right = self.root.digest;
        
        while left.is_some() && right.is_some() {
            proof.push(left.unwrap().digest);
            old_right = right.unwrap().digest;
            (left, right) = right.unwrap().children();
        }

        proof.reverse();

        MerkleTreeProof{
            leaf: old_right,
            data: proof,
        }
    }
}
