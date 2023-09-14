use std::{
    sync::Arc,
    collections::HashMap,
};

use crate::{
    merkle::{ Hash, MerkleTreeNode, MerkleTreeLeaf, MerkleTree},
    utils::arithmetic,
};

#[derive(Debug)]
pub struct MerkleBuilder {
    leafs: Vec<Arc<MerkleTreeLeaf>>,
    nodes: HashMap<Hash, Arc<MerkleTreeNode>>,
    iterateds: HashMap<Hash, Vec<Arc<MerkleTreeNode>>>,
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

        if let Some(last) = self.leafs.last() {
            assert!(last.accumulated_count != 0, "merkle builder is full");
            let accumulated_count = rep + last.accumulated_count;
            if !arithmetic::ult(rep, accumulated_count) {
                assert_eq!(accumulated_count, 0);
            }
            self.leafs.push(Arc::new(MerkleTreeLeaf {
                node: self.create_node(digest),
                accumulated_count,
                log2_size: None
            }));
        } else {
            self.leafs.push(Arc::new(MerkleTreeLeaf {
                node: self.create_node(digest),
                accumulated_count: rep,
                log2_size: None
            }));
        }
    }

    fn create_node(&mut self, digest: Hash) -> Arc<MerkleTreeNode> {
        if let Some(node) = self.nodes.get(&digest) {
            return node.clone()
        }

        let node = Arc::new(MerkleTreeNode::new(digest));
        self.nodes.insert(node.digest, node.clone());
        self.iterateds.insert(node.digest, vec![node.clone()]);

        node
    }

    pub fn build(&mut self) -> MerkleTree {
        let last = self.leafs.last().expect("no leafs in merkle builder");
        let count = last.accumulated_count as u64;
        let mut log2size = 64;
        if count != 0 {
            assert!(arithmetic::is_pow2(count), "{}", count);
            log2size = arithmetic::ctz(count)
        };
        let root = self.build_merkle(self.leafs.as_slice(), log2size, 0);
        MerkleTree::new(log2size, root.0, self.leafs, self.nodes)
    }

    fn build_merkle(
        &mut self,
        leafs: &[Arc<MerkleTreeLeaf>],
        log2size: u32,
        stride: u64
    ) -> (Arc<MerkleTreeNode>, u64, u64) {
        let shifting = (1 as u64).checked_shl(log2size);
        let (first_time, last_time) = match shifting {
            Some(sh) => {
                ((stride * sh + 1), (sh * (stride + 1) as u64))
            },
            None => {
                (0, 0)
            },
        };

        let first_cell = find_cell_containing(leafs, first_time as u64);
        let last_cell = find_cell_containing(leafs, last_time as u64);
        if first_cell == last_cell {
            let iterated = self.iterated_merkle(self.leafs[first_cell as usize].node, log2size);
            return (iterated, first_time, last_time)
        }

        let leafs = &self.leafs.as_slice()[first_cell as usize..(last_cell + 1) as usize];
        let left = self.build_merkle(leafs, log2size - 1, stride << 1);
        let right = self.build_merkle(leafs, log2size - 1, (stride << 1) + 1);
        
        let result = left.0.join(right.0);
        (result, first_time, last_time)
    }

    fn iterated_merkle(&mut self, node: Arc<MerkleTreeNode>, level: u32) -> Arc<MerkleTreeNode> {
        let iterated = self.iterateds.get(&node.digest).unwrap();
        if let Some(node) = iterated.get(level as usize).clone() {
            return node.clone();
        }
        
        let mut i = iterated.len() - 1;
        let mut highest_level = iterated.last().unwrap().clone();
        while i < level as usize {
            highest_level = highest_level.join(highest_level);
            self.iterateds.get_mut(&node.digest).unwrap().push(highest_level);
            i += 1;
        }
        highest_level
    }

    pub fn node(&self, node_digest: Hash) -> Option<Arc<MerkleTreeNode>> {
        if let Some(node) = self.nodes.get(&node_digest) {
            Some(node.clone())
        } else {
            None
        }
    }
}

fn find_cell_containing(leafs: &[Arc<MerkleTreeLeaf>], elem: u64) -> u64 {
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