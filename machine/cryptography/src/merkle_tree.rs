use crate::hash::Hash;
use crate::merkle_builder::Leaf;
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

    pub fn children(&self, node: Vec<u8>) -> (Option<Vec<u8>>, Option<Vec<u8>>) {
        self.leafs
            .iter()
            .find_map(|leaf| {
                if leaf.hash.digest == node {
                    Some(leaf.hash.children())
                } else {
                    None
                }
            })
            .unwrap_or((None, None))
    }

    fn prove_leaf(&self, index: u64) -> (Option<Vec<u8>>, Proof) {
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
        self.generate_proof(&mut proof, self.root_hash.digest.clone(), height, index);

        (proof.leaf.clone(), proof)
    }

    fn last(&self) -> (Vec<u8>, Vec<Vec<u8>>) {
        let mut proof = Vec::new();
        let mut old_right = self.root_hash.digest.clone();
        let (mut children_left, mut children_right) = self.root_hash.children();
        let mut ok = children_left.is_some() || children_right.is_some();

        while ok {
            proof.push(children_left.as_ref().unwrap().clone());
            old_right = children_right.as_ref().unwrap().clone();
            (children_left, children_right) = self.root_hash.children();
            ok = children_left.is_some() || children_right.is_some();
        }

        proof.reverse();

        (old_right, proof)
    }
    pub fn hex_string(&self) -> String {
        hex::encode(self.root_hash.hex_string())
    }

    fn generate_proof(&self, proof: &mut Proof, root: Vec<u8>, height: u32, include_index: u64) {
        if height == 0 {
            proof.leaf = Some(root);
            return;
        }

        let new_height = height - 1;
        let (left, right) = &self.children(root);
        assert!(left.is_some() && right.is_some());

        if (include_index >> new_height) & 1 == 0 {
            self.generate_proof(
                proof,
                left.as_ref().unwrap().clone(),
                new_height,
                include_index,
            );
            proof.data.push(right.as_ref().unwrap().clone());
        } else {
            self.generate_proof(
                proof,
                right.as_ref().unwrap().clone(),
                new_height,
                include_index,
            );
            proof.data.push(left.as_ref().unwrap().clone());
        }
    }
}

impl ToString for MerkleTree {
    fn to_string(&self) -> String {
        self.hex_string()
    }
}

pub struct Proof {
    pub leaf: Option<Vec<u8>>,
    pub data: Vec<Vec<u8>>,
}
