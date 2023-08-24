use std::collections::HashMap;
use once_cell::sync::OnceCell;
use sha3::{Digest, Keccak256};

#[derive(Eq, Hash, PartialEq, Clone, Debug, Default)]
pub struct Hash {
    pub digest:Vec<u8>,
    left: Option<Box<Hash>>,
    right: Option<Box<Hash>>,
}

static INTERNALIZED_HASHES: OnceCell<HashMap<Vec<u8>, Hash>> = OnceCell::new();
static ITHERADETS: OnceCell<HashMap<Hash, Vec<Hash>>> = OnceCell::new();

impl Hash {
    pub fn from_digest(digest: Vec<u8>) -> Hash {
        let mut extended_itheradets = ITHERADETS.get_or_init(|| HashMap::new()).clone();

        let mut extended_internalized_hashes =
        INTERNALIZED_HASHES.get_or_init(|| HashMap::new()).clone();

        if let Some(x) = extended_internalized_hashes.get(&digest) {
            return x.clone();
        }

        let h = Hash {
            digest: digest.clone(),
            left: None,
            right: None,
        };
        extended_itheradets.insert(h.clone(), vec![h.clone()]);
        extended_internalized_hashes.insert(digest, h.clone());

        ITHERADETS.set(extended_itheradets);
        INTERNALIZED_HASHES.set(extended_internalized_hashes);
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
        ret.left = Some(Box::new(self.clone()));
        ret.right = Some(Box::new(other_hash.clone()));
        ret
    }

    pub fn children(&self) -> (Option<Box<Hash>>, Option<Box<Hash>>) {
        match (self.left.clone(), self.right.clone()) {
            (Some(left), Some(right)) => (Some(left), Some(right)),
            _ => (None, None),
        }
    }

    pub fn iterated_merkle(&self, level: u32) -> Hash {
        let mut iterated_hashes = ITHERADETS.get().unwrap().clone();
        if let Some(iterated) = iterated_hashes.get(self) {
            if let Some(hash) = iterated.get(level as usize) {
                return hash.clone();
            }
        } else {
            iterated_hashes.insert(self.clone(), Vec::new());
        }

        let mut highest_level = self.clone();
        let mut i = iterated_hashes[&highest_level].len();
        while i < level as usize {
            highest_level = highest_level.join(&highest_level);
            i += 1;
            iterated_hashes
                .get_mut(&self)
                .unwrap()
                .push(highest_level.clone());
        }
        ITHERADETS.set(iterated_hashes);
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
    hex::decode("0x0000000000000000000000000000000000000000000000000000000000000000".to_string()).unwrap()
}

pub fn zero_hash() -> Hash {
    Hash::from_digest(zero_bytes32())
}
