use std::collections::HashMap;

use crate::merkle::{Hash, MerkleTreeNode};

#[derive(Clone, Debug)]
pub struct MerkleTreeLeaf {
    pub node: Hash,
    pub accumulated_count: u64,
    pub log2_size: Option<u64>
}

pub type MerkleProof = Vec<Hash>;

struct ProofAccumulator {
    pub leaf: Hash,
    pub proof: MerkleProof,
}

#[derive(Debug)]
pub struct MerkleTree {
    log2_size: u64,
    root: Hash,
    leafs: Vec<MerkleTreeLeaf>,
    nodes: HashMap<Hash, MerkleTreeNode>,
}

impl MerkleTree {
    pub fn new(
        log2_size: u64,
        root: Hash,
        leafs: Vec<MerkleTreeLeaf>,
        nodes: HashMap<Hash, MerkleTreeNode>,
    ) -> Self {
        MerkleTree {
            log2_size: log2_size,
            root: root,
            leafs: leafs,
            nodes: nodes,
        }
    }

    pub fn root_hash(&self) -> Hash {
        self.root
    }

    pub fn root_children(&self) -> (Hash, Hash) {
        self.node_children(self.root).expect("root does not have children")
    }

    pub fn node_children(&self, digest: Hash) -> Option<(Hash, Hash)> {
        let node = self.nodes.get(&digest).expect("node does not exist");
        node.children()
    }

    pub fn prove_leaf(
        &self,
        index: u64
    ) -> (Hash, MerkleProof) {
        let mut height = 0u64;
        if let Some(leaf) = self.leafs.get(0) {
            if let Some(log2_size) = leaf.log2_size {
                height = log2_size + self.log2_size;
            }
        }
        assert!((index >> height) == 0);

        let mut proof_acc = ProofAccumulator {
            leaf: Hash::default(),
            proof: Vec::new(),
        };
        self.generate_proof(
            &mut proof_acc,
            self.nodes.get(&self.root).expect("root does not exist"),
            height,
            index
        );

        (proof_acc.leaf, proof_acc.proof)
    }

    fn generate_proof(
        &self, 
        proof_acc: &mut ProofAccumulator,
        root: &MerkleTreeNode,
        height: u64,
        include_index: u64
    ) {
        if height == 0 {
            proof_acc.leaf = root.digest;
            return;
        }

        let new_height = height - 1;
        let (left, right) = root.children().expect("root does not have children");
        let left = self.nodes.get(&left).expect("left child does not exist");
        let right = self.nodes.get(&right).expect("right child does not exist");

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
        let mut children: Option<(Hash, Hash)> = Some(self.root_children());
        let mut old_right = self.root;
        
        while let Some((left, right)) = children {
            proof.push(left);
            old_right = right;
            children = self.node_children(right);
        }

        proof.reverse();

        (old_right, proof)
    }
}
