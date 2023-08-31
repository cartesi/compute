use lazy_static::lazy_static;
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
#[derive(Eq, Hash, PartialEq, Clone, Debug, Default)]
pub struct Hash {
    pub digest: Vec<u8>,
    left: Option<Arc<Hash>>,
    right: Option<Arc<Hash>>,
}

lazy_static! {
    static ref ITHERADETS: Mutex<HashMap<Hash, Vec<Hash>>> = Mutex::new(HashMap::new());
    static ref INTERNALIZED_HASHES: Mutex<HashMap<Vec<u8>, Hash>> = Mutex::new(HashMap::new());
}

impl Hash {
    pub fn from_digest(digest: Vec<u8>) -> Hash {
        match INTERNALIZED_HASHES.lock().unwrap().get(&digest) {
            Some(hash) => {
                return hash.clone();
            }
            None => {}
        }
        let h = Hash {
            digest: digest.clone(),
            left: None,
            right: None,
        };
        ITHERADETS
            .lock()
            .unwrap()
            .insert(h.clone(), vec![h.clone()]);
        INTERNALIZED_HASHES
            .lock()
            .unwrap()
            .insert(digest, h.clone());
        h
    }

    pub fn from_digest_hex(digest_hex: &str) -> Hash {
        assert!(digest_hex.len() == 66);
        let digest = hex::decode(&digest_hex).unwrap();
        Hash::from_digest(digest)
    }

    fn from_data(data: &[u8; 32]) -> Hash {
        let mut keccak = Keccak256::new();
        keccak.update(&hex::decode(&data).unwrap());
        let digest = keccak.finalize();
        Hash::from_digest(digest.to_vec())
    }

    pub fn join(&self, other_hash: &Hash) -> Hash {
        let mut keccak = Keccak256::new();
        keccak.update(&self.digest);
        keccak.update(&other_hash.digest);
        let digest = keccak.finalize();
        let mut ret = Hash::from_digest(digest.to_vec());
        ret.left = Some(Arc::new(self.clone()));
        ret.right = Some(Arc::new(other_hash.clone()));
        ret
    }

    pub fn children(&self) -> (Option<Arc<Hash>>, Option<Arc<Hash>>) {
        match (self.left.clone(), self.right.clone()) {
            (Some(left), Some(right)) => (Some(left), Some(right)),
            _ => (None, None),
        }
    }

    pub fn iterated_merkle(&self, level: u32) -> Hash {
        let iterated = ITHERADETS.lock().unwrap().get(self).unwrap().clone();
        if let Some(hash) = iterated.get(level as usize).clone() {
            return hash.clone();
        }
        let mut i = iterated.len() - 1;
        let mut highest_level = iterated.last().unwrap().clone();
        while i < level as usize {
            highest_level = highest_level.clone().join(&highest_level.clone());
            i += 1;
            ITHERADETS
                .lock()
                .unwrap()
                .get_mut(self)
                .unwrap()
                .push(highest_level.clone());
        }
        highest_level
    }

    fn is_zero(&self) -> bool {
        self == &zero_hash()
    }

    fn is_of_type_hash(&self, x: &Hash) -> bool {
        self.eq(x)
    }

    pub fn hex_string(&self) -> String {
        hex::encode(self.digest.clone())
    }
}

impl ToString for Hash {
    fn to_string(&self) -> String {
        self.hex_string()
    }
}

fn zero_bytes32() -> Vec<u8> {
    hex::decode("0000000000000000000000000000000000000000000000000000000000000000".to_string())
        .unwrap()
}

pub fn zero_hash() -> Hash {
    Hash::from_digest(zero_bytes32())
}
