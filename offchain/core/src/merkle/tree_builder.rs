use std::collections::HashMap;

use sha3::{Digest, Keccak256};

use crate::{
    merkle::{ Hash, MerkleTreeNode, MerkleTreeLeaf, MerkleTree},
    utils::arithmetic,
};

#[derive(Debug)]
pub struct MerkleBuilder {
    leafs: Vec<MerkleTreeLeaf>,
    nodes: HashMap<Hash, MerkleTreeNode>,
    iterateds: HashMap<Hash, Vec<Hash>>,
}

impl MerkleBuilder {
    pub fn new() -> Self {
        MerkleBuilder { 
            leafs: Vec::new(),
            iterateds: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn add(&mut self, digest: Hash, rep: Option<u64>) {
        let rep = match rep {
            Some(r) => r,
            None => 1,
        };
        assert!(arithmetic::ult(0, rep));

        if !self.nodes.contains_key(&digest) {
            let node = MerkleTreeNode::new(digest);
            self.nodes.insert(node.digest, node.clone());
            self.iterateds.insert(node.digest, vec![node.digest]);
        }
        
        if let Some(last) = self.leafs.last() {
            assert!(last.accumulated_count != 0, "merkle builder is full");
            let accumulated_count = rep + last.accumulated_count;
            if !arithmetic::ult(rep, accumulated_count) {
                assert_eq!(accumulated_count, 0);
            }
            self.leafs.push(MerkleTreeLeaf {
                node: digest,
                accumulated_count,
                log2_size: None
            });
        } else {
            self.leafs.push(MerkleTreeLeaf {
                node: digest,
                accumulated_count: rep,
                log2_size: None
            });
        }
    }

    pub fn build(&mut self) -> MerkleTree {
        let last = self.leafs.last().expect("no leafs in merkle builder");
        let count = last.accumulated_count;
        let mut log2_size = 64;
        if count != 0 {
            assert!(arithmetic::is_pow2(count), "{}", count);
            log2_size = arithmetic::ctz(count)
        };
        let root = self.build_merkle(0, self.leafs.len() as u64, log2_size, 0);
        MerkleTree::new(log2_size, root.0, self.leafs.clone(), self.nodes.clone())
    }

    fn build_merkle(
        &mut self,
        first_leaf: u64,
        last_leaf: u64,
        log2_size: u64,
        stride: u64
    ) -> (Hash, u64, u64) {
        let leafs = &self.leafs.as_slice()[first_leaf as usize..last_leaf as usize];
        
        let first_time = stride * (1 << log2_size) + 1;
        let last_time = (stride + 1) * (1 << log2_size);

        let first_cell = find_cell_containing(leafs, first_time);
        let last_cell = find_cell_containing(leafs, last_time);
        if first_cell == last_cell {
            let node = self.leafs[first_cell as usize].node;
            let iterated = self.iterated_merkle(node, log2_size);
            return (iterated, first_time, last_time)
        }

        let left = self.build_merkle(first_cell, last_cell + 1, log2_size - 1, stride << 1);
        let right = self.build_merkle(first_cell, last_cell + 1, log2_size - 1, (stride << 1) + 1);
        
        let result = self.join_nodes(left.0, right.0);
        (result, first_time, last_time)
    }

    fn iterated_merkle(&mut self, node: Hash, level: u64) -> Hash {
        let iterated = self.iterateds.get(&node).unwrap();
        if let Some(node) = iterated.get(level as usize) {
            return *node;
        }
        self.build_iterated_merkle(node, level)
    }

    fn build_iterated_merkle(&mut self, node: Hash, level: u64) -> Hash {
        let iterated = self.iterateds.get(&node).unwrap();
        let mut i = iterated.len() - 1;
        let mut highest_level = *iterated.get(i).unwrap();
        while i < level as usize {
            highest_level = self.join_nodes(highest_level, highest_level);
            i += 1;
            self.iterateds.get_mut(&node).unwrap().push(highest_level);
        }
        highest_level
    }

    fn join_nodes(&mut self,
        left: Hash,
        right: Hash,
    ) -> Hash {
        let digest = join_merkle_tree_node_digests(left, right);
        
        let node = if let Some(node) = self.nodes.get_mut(&digest) {
            node
        } else {
            let node = MerkleTreeNode::new(digest);
            self.nodes.insert(node.digest, node.clone());
            self.iterateds.insert(node.digest, vec![node.digest]);
            self.nodes.get_mut(&digest).unwrap()
        };
        node.set_children(left, right);

        digest
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

fn find_cell_containing(leafs: &[MerkleTreeLeaf], elem: u64) -> u64 {
    let mut l = 1;
    let mut r = leafs.len() as u64;

    while arithmetic::ult(l, r) {
        let m = arithmetic::semi_sum(l, r);
        if arithmetic::ult(leafs[m as usize].accumulated_count - 1, elem - 1) {
            l = m + 1;
        } else {
            r = m;
        }
    }

    l
}