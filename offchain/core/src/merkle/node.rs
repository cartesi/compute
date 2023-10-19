use crate::merkle::Hash;

#[derive(Clone, Debug)]
pub struct MerkleTreeNode {
    pub digest: Hash,
    left: Option<Hash>,
    right: Option<Hash>,
}

impl MerkleTreeNode {
    pub fn new(digest: Hash) -> Self {
        MerkleTreeNode {
            digest: digest,
            left: None,
            right: None,
        }
    }

    pub fn set_children(&mut self, left: Hash, right: Hash) {
        self.left = Some(left);
        self.right = Some(right);
    }

    pub fn children(&self) -> Option<(Hash, Hash)> {
        if self.left.is_some() && self.right.is_some() {
            let left = self.left.as_ref().unwrap();
            let right = self.right.as_ref().unwrap(); 
            Some((left.clone(), right.clone()))
        } else {
            None
        }
    }
}