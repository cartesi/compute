use std::sync::Arc;

use crate::merkle::node::Hash;

#[derive(Clone, Debug)]
pub struct Leaf {
    pub hash: Hash,
    pub accumulated_count: u64,
    pub log2size: Option<u32>
}

#[derive(Debug)]
pub struct MerkleTree {
    leafs: Vec<Leaf>,
    pub root_hash: Hash,
    digest_hex: String,
    pub log2size: u32,
    implicit_hash: Option<Hash>,
}

impl MerkleTree {
    pub fn new(
        leafs: Vec<Leaf>,
        root_hash: Hash,
        log2size: u32,
        implicit_hash: Option<Hash>,
    ) -> Self {
        MerkleTree {
            leafs,
            root_hash: root_hash.clone(),
            digest_hex: hex::encode(&root_hash.digest.clone()),
            log2size,
            implicit_hash,
        }
    }

    pub fn join(&self, other_hash: Hash) -> Hash {
        self.root_hash.join(&other_hash)
    }

    pub fn iterated_merkle(&self, level: u32) -> Hash {
        self.root_hash.iterated_merkle(level)
    }

    pub fn children(&self) -> (Option<Arc<Hash>>, Option<Arc<Hash>>) {
        self.root_hash.children()
    }

    pub fn prove_leaf(&self, index: u64) -> (Option<Hash>, Proof) {
        let mut height = self.log2size;
        if let Some(leaf) = self.leafs.get(0) {
            if let Some(log2size) = leaf.log2size {
                height = log2size + self.log2size;
            }
        }

        println!("{:?} {:?} P", index, height);

        assert!((index >> height) == 0);

        let mut proof = Proof {
            leaf: None,
            data: Vec::new(),
        };
        self.generate_proof(&mut proof, self.root_hash.clone(), height, index);

        (proof.leaf.clone(), proof)
    }

    pub fn last(&self) -> (Hash, Vec<Hash>) {
        let mut proof = Vec::new();
        let mut old_right = self.root_hash.clone();
        let (mut children_left, mut children_right) = self.root_hash.children();
        let mut ok = children_left.is_some() && children_right.is_some();
        while ok {
            proof.push((*children_left.as_ref().unwrap().clone()).clone());
            old_right = (*children_right.as_ref().unwrap().clone()).clone();
            (children_left, children_right) = children_right.as_ref().unwrap().children();
            ok = children_left.is_some() && children_right.is_some();
        }

        proof.reverse();

        (old_right, proof)
    }
    pub fn hex_string(&self) -> String {
        hex::encode(self.root_hash.hex_string())
    }

    fn generate_proof(&self, proof: &mut Proof, root: Hash, height: u32, include_index: u64) {
        if height == 0 {
            proof.leaf = Some(root);
            return;
        }

        let new_height = height - 1;
        let (left, right) = root.children();
        assert!(left.is_some() && right.is_some());

        if (include_index >> new_height) & 1 == 0 {
            self.generate_proof(
                proof,
                (*Arc::clone(left.as_ref().unwrap())).clone(),
                new_height,
                include_index,
            );
            proof.data.push((*Arc::clone(right.as_ref().unwrap())).clone());
        } else {
            self.generate_proof(
                proof,
                (*Arc::clone(right.as_ref().unwrap())).clone(),
                new_height,
                include_index,
            );
            proof.data.push((*Arc::clone(left.as_ref().unwrap())).clone());
        }
    }
}

impl ToString for MerkleTree {
    fn to_string(&self) -> String {
        self.hex_string()
    }
}
#[derive(Debug)]
pub struct Proof {
    pub leaf: Option<Hash>,
    pub data: Vec<Hash>,
}
