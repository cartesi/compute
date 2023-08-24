use crate::hash::Hash;
use crate::merkle_builder::Leaf;
#[derive(Debug)]
pub struct MerkleTree {
    leafs: Vec<Leaf>,
    pub root_hash: Hash,
    digest_hex: String,
    pub log2size: u32,
    implicit_hash: Option<Hash>
}

impl MerkleTree {
    pub fn new(leafs: Vec<Leaf>, root_hash: Hash, log2size: u32, implicit_hash: Option<Hash>) -> Self {
        MerkleTree {
            leafs,
            root_hash: root_hash.clone(),
            digest_hex: hex::encode(&root_hash.digest.clone()),
            log2size,
            implicit_hash
        }
    }

    pub fn join(&self, other_hash: Hash) -> Hash {
        self.root_hash.join(&other_hash)
    }

    pub fn iterated_merkle(&self, level: u32) -> Hash {
        self.root_hash.iterated_merkle(level)
    }
    
    pub fn children(&self, node: Hash) -> (Option<Box<Hash>>, Option<Box<Hash>>) {
        self.leafs.iter().find_map(|leaf| {
            if leaf.hash == node {
                Some(leaf.hash.children())
            } else {
                None
            }
        }).unwrap_or((None, None))
    }

    fn prove_leaf(&self, index: u64) -> (Option<Hash>, Proof) {
        let mut height = self.log2size;
        if let Some(leaf) = self.leafs.get(0) {
            if let Some(log2size) = leaf.log2size {
                height = log2size + self.log2size;
            }
        }

        println!("{:?} {:?} P", index, height);

        assert!((index >> height) == 0);

        let mut proof = Proof { leaf: None, data: Vec::new()};
        generate_proof(&mut proof, Box::new(self.root_hash.clone()), height, index);

        (proof.leaf.clone(), proof)
    }

    fn last(&self) -> (Box<Hash>, Vec<Box<Hash>>) {
        let mut proof = Vec::new();
        let mut ok = true;
        let mut left: Box<Hash>; 
        let mut right: Box<Hash>; 
        let mut old_right = Box::new(self.root_hash.clone());

        while ok {
            let (children_left, children_right) = self.root_hash.children();
            
            if let (Some(children_left), Some(children_right)) = (children_left, children_right) {
                left = children_left;
                right = children_right;
                proof.push(left);
                old_right = right;
            } else {
                ok = false;
            }
        }
        
        proof.reverse();

        (old_right, proof)
    }
    pub fn hex_string(&self) -> String {
        hex::encode(self.root_hash.hex_string())
    }
}

impl ToString for MerkleTree {
    fn to_string(&self) -> String {
        self.hex_string()
    }
}

fn generate_proof(proof: &mut Proof, root: Box<Hash>, height: u32, include_index: u64) {
    if height == 0 {
        proof.leaf = Some(*root);
        return;
    }

    let new_height = height - 1;
    let (left, right) = root.children();
    assert!(left.is_some() && right.is_some());

    if (include_index >> new_height) & 1 == 0 {
        generate_proof(proof, left.unwrap(), new_height, include_index);
        proof.data.push(right.unwrap());
    } else {
        generate_proof(proof, right.unwrap(), new_height, include_index);
        proof.data.push(left.unwrap());
    }
}

pub struct Proof {
    pub leaf: Option<Hash>,
    pub data: Vec<Box<Hash>>,
}