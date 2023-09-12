use crate::{
    merkle::node::MerkleTreeNode,
    merkle::tree::{MerkleTreeLeaf, MerkleTree},
    utils::arithmetic,
};

#[derive(Debug)]
struct Slice<'a> {
    arr: &'a Vec<MerkleTreeLeaf>,
    start_idx_inc: u64,
    end_idx_ex: u64,
}

impl<'a> Slice<'a> {
    fn new(arr: &'a Vec<MerkleTreeLeaf>, start_idx_inc: u64, end_idx_ex: u64) -> Self {
        let start_idx_inc = start_idx_inc;
        let end_idx_ex = end_idx_ex;
        assert!(arithmetic::ulte(start_idx_inc, end_idx_ex));
        assert!(end_idx_ex <= arr.len() as u64 + 1);
        Slice {
            arr,
            start_idx_inc,
            end_idx_ex,
        }
    }

    fn slice(&self, si: u64, ei: u64) -> Self {
        assert!(arithmetic::ulte(si, ei));
        let start_idx_inc = self.start_idx_inc + si - 1;
        let end_idx_ex = self.start_idx_inc + ei - 1;
        assert!(arithmetic::ulte(end_idx_ex, self.end_idx_ex));
        Slice::new(self.arr, start_idx_inc, end_idx_ex)
    }

    fn len(&self) -> u64 {
        self.end_idx_ex - self.start_idx_inc
    }

    fn get(&self, idx: u64) -> MerkleTreeLeaf {
        assert!(idx > 0);
        let i = self.start_idx_inc + idx - 1;
        assert!(i <= self.end_idx_ex);
        self.arr[i as usize].clone()
    }

    fn find_cell_containing(&self, elem: u64) -> u64 {
        let mut l = 1;
        let mut r = self.len() as u64;

        while arithmetic::ult(l, r) {
            let m = arithmetic::semi_sum(l, r);
            if arithmetic::ult(self.get(m).accumulated_count - 1, elem - 1) {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l
    }
}
#[derive(Clone, Debug)]
pub struct MerkleBuilder {
    pub leafs: Vec<MerkleTreeLeaf>,
}

impl MerkleBuilder {
    pub fn new() -> Self {
        MerkleBuilder { leafs: Vec::new() }
    }

    pub fn add(&mut self, hash: MerkleTreeNode, rep: Option<u64>) {
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

            self.leafs.push(MerkleTreeLeaf {
                hash,
                accumulated_count,
                log2_size: None
            });
        } else {
            self.leafs.push(MerkleTreeLeaf {
                hash,
                accumulated_count: rep,
                log2_size: None
            });
        }
    }

    pub fn build(&mut self, implicit_hash: Option<MerkleTreeNode>) -> MerkleTree {
        let last = self.leafs.last().expect("no leafs in merkle builder");
        let count = last.accumulated_count as u64;
        let mut log2size = 64;
        if count != 0 {
            assert!(arithmetic::is_pow2(count), "{}", count);
            log2size = arithmetic::ctz(count)
        };
        let root_hash = merkle(
            &Slice::new(&self.leafs, 0, (self.leafs.len()) as u64),
            log2size,
            0,
        );
        MerkleTree::new(self.leafs.clone(), root_hash.0, log2size, implicit_hash)
    }

    fn merkle(&mut self, log2size: u32, stride: u64) -> (MerkleTreeNode, u64, u64) {
        let shifting = (1 as u64).checked_shl(log2size);
        let (first_time, last_time) = match shifting {
            Some(sh) => {
                ((stride * sh + 1), (sh * (stride + 1) as u64))
            },
            None => {
                (0, 0)
            },
        };
        let first_cell = leafs.find_cell_containing(first_time as u64);
        let last_cell = leafs.find_cell_containing(last_time as u64);
        if first_cell == last_cell {
            return (leafs.get(first_cell).hash.iterated_merkle(log2size), first_time, last_time);
        }
        let slice: Slice<'_> = leafs.slice(first_cell, last_cell + 1);
        let hash_left = merkle(&slice, log2size - 1, stride << 1);
        let hash_right = merkle(&slice, log2size - 1, (stride << 1) + 1);
        let result = hash_left.0.join(&hash_right.0);
        (result, first_time, last_time)
    }

    fn iterated_merkle(&self, level: u32) -> MerkleTreeNode {

    }


}