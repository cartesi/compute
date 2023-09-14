use std::{sync::Arc, collections::HashMap};

use crate::merkle::{Hash, MerkleTreeNode};

#[derive(Debug)]
pub struct MerkleTreeLeaf {
    pub node: Arc<MerkleTreeNode>,
    pub accumulated_count: u64,
    pub log2_size: Option<u32>
}

pub type MerkleProof = Vec<Hash>;

struct ProofAccumulator {
    pub leaf: Hash,
    pub proof: MerkleProof,
}

#[derive(Debug)]
pub struct MerkleTree {
    log2_size: u32,
    root: Arc<MerkleTreeNode>,
    leafs: Vec<Arc<MerkleTreeLeaf>>,
    nodes: HashMap<Hash, Arc<MerkleTreeNode>>,
}

impl MerkleTree {
    pub fn new(
        log2_size: u32,
        root: Arc<MerkleTreeNode>,
        leafs: Vec<Arc<MerkleTreeLeaf>>,
        nodes: HashMap<Hash, Arc<MerkleTreeNode>>,
    ) -> Self {
        MerkleTree {
            log2_size: log2_size,
            root: root,
            leafs: leafs,
            nodes: nodes,
        }
    }

    pub fn node(&self, digest: Hash) -> Option<Arc<MerkleTreeNode>> {
        if let Some(node) = self.nodes.get(&digest) {
            Some(node.clone())
        } else {
            None
        }
    }

    pub fn root_hash(&self) -> Hash {
        self.root.digest
    }

    pub fn root_children(&self) -> Option<(Arc<MerkleTreeNode>, Arc<MerkleTreeNode>)> {
        self.root.children()
    }

    pub fn prove_leaf(
        &self,
        index: u64
    ) -> (Hash, MerkleProof) {
        let mut height = 0u32;
        if let Some(leaf) = self.leafs.get(0) {
            if let Some(log2size) = leaf.log2_size {
                height = log2size + self.log2_size;
            }
        }
        assert!((index >> height) == 0);

        let mut proof_acc = ProofAccumulator {
            leaf: Hash::default(),
            proof: Vec::new(),
        };
        self.generate_proof(&mut proof_acc, self.root, height, index);

        (proof_acc.leaf, proof_acc.proof)
    }

    fn generate_proof(
        &self, 
        proof_acc: &mut ProofAccumulator,
        root: Arc<MerkleTreeNode>,
        height: u32,
        include_index: u64
    ) {    
        if height == 0 {
            proof_acc.leaf = root.digest;
            return;
        }

        let new_height = height - 1;
        let (left, right) = root.children().expect("root does not have children");

        if (include_index >> new_height) & 1 == 0 {
            let left = left;
            self.generate_proof(
                proof_acc,
                left,
                new_height,
                include_index,
            );
            proof_acc.proof.push(left.digest);
        } else {
            let right = right;
            self.generate_proof(
                proof_acc,
                right,
                new_height,
                include_index,
            );
            proof_acc.proof.push(right.digest);
        }
    }

    pub fn last(&self) -> (Hash, MerkleProof) {
        let mut proof = Vec::new();
        let mut children = self.root.children();
        let mut old_right = self.root.digest;
        
        while let Some((left, right)) = children {
            proof.push(left.digest);
            old_right = right.digest;
            children = right.children();
        }

        proof.reverse();

        (old_right, proof)
    }
}
