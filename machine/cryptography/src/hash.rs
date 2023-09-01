use lazy_static::lazy_static;
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
#[derive(Eq, Hash, PartialEq, Clone, Debug, Default)]
pub struct Hash {
    pub digest: [u8; 32],
    left: Option<Arc<Hash>>,
    right: Option<Arc<Hash>>,
}

lazy_static! {
    static ref ITHERADETS: Mutex<HashMap<Hash, HashMap<usize, Hash>>> = Mutex::new(HashMap::new());
    static ref INTERNALIZED_HASHES: Mutex<HashMap<[u8; 32], Hash>> = Mutex::new(HashMap::new());
}

impl Hash {
    pub fn new(data: [u8; 32]) -> Hash {
        match INTERNALIZED_HASHES.lock().unwrap().get(&data) {
            Some(hash) => {
                return hash.clone();
            }
            None => {}
        }
        let h = Hash {
            digest: data.clone(),
            left: None,
            right: None,
        };
        let mut initial_hash_map = HashMap::new();
        initial_hash_map.insert(0, h.clone());
        ITHERADETS
            .lock()
            .unwrap()
            .insert(h.clone(), initial_hash_map);
        INTERNALIZED_HASHES
            .lock()
            .unwrap()
            .insert(data, h.clone());
        h
    }

    pub fn from_digest_hex(digest_hex: &str) -> Hash {
        assert!(digest_hex.len() == 66);
        let mut data = [0u8; 32];
        hex::decode_to_slice(&digest_hex, &mut data as &mut [u8]).unwrap();
        Hash::new(data)
    }

    fn from_data(data:Vec<u8>) -> Hash {
        let mut keccak = Keccak256::new();
        keccak.update(&data);
        let digest: [u8; 32] = keccak.finalize().into();
        Hash::new(digest)
    }

    pub fn join(&self, other_hash: &Hash) -> Hash {
        let mut keccak = Keccak256::new();
        keccak.update(&self.digest);
        keccak.update(&other_hash.digest);
        let digest: [u8; 32] = keccak.finalize().into();
        let mut ret = Hash::new(digest);
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
        if let Some(hash) = iterated.get(&(level as usize)).clone() {
            return hash.clone();
        }
        let mut i = iterated.len() - 1;
        let mut highest_level = iterated.get(&i).unwrap().clone();
        while i < level as usize {
            highest_level = highest_level.clone().join(&highest_level.clone());
            i += 1;
            ITHERADETS
                .lock()
                .unwrap()
                .get_mut(self)
                .unwrap()
                .insert(i, highest_level.clone());
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

fn zero_bytes32() -> [u8; 32] {
    let mut data = [0u8; 32];
    hex::decode_to_slice(
        "0000000000000000000000000000000000000000000000000000000000000000",
        &mut data as &mut [u8],
    )
    .unwrap();
    data
}

pub fn zero_hash() -> Hash {
    Hash::new(zero_bytes32())
}
