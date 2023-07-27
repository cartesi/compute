use std::collections::HashMap;

use tiny_keccak::{Hasher, Sha3};
use lazy_static::lazy_static;


fn hex_from_bin(bin: &[u8]) -> String {
    assert_eq!(bin.len(), 32);
    let hex_chars: Vec<String> = bin.iter().map(|c| format!("{:02x}", c)).collect();
    format!("0x{}", hex_chars.join(""))
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Hash {
    pub digest_hex: String,
    left: Option<String>,
    right: Option<String>,
}
lazy_static! {
    static ref internalized_hashes: std::sync::Mutex<HashMap<String, Hash>> = std::sync::Mutex::new(HashMap::new());
    static ref iterateds: std::sync::Mutex<HashMap<Hash, Vec<Hash>>> = std::sync::Mutex::new(HashMap::new());
}

impl Hash {
    pub fn from_digest(digest_hex: &str) -> Hash {
        assert!(digest_hex.len() == 66);
        let mut iterated_hashes = &mut iterateds.lock().unwrap();
        let mut internalized_hashes_locked = &mut internalized_hashes.lock().unwrap();

        if let Some(x) = internalized_hashes_locked.get(digest_hex) {
            return x.clone();
        }

        let h = Hash {
            digest_hex: digest_hex.to_string(),
            left: None,
            right: None,
        };

        iterated_hashes.insert(h.clone(), vec![h.clone()]);
        internalized_hashes_locked.insert(digest_hex.to_string(), h.clone());
        //println!("---------------digest_hex {:?}, from_digest {:?}", digest_hex, h);
        h
    }

    pub fn from_digest_bin(digest_bin: &[u8]) -> Hash {
        let digest_hex = hex_from_bin(digest_bin);
        Hash::from_digest(&digest_hex)
    }

    fn from_data(data: &[u8]) -> Hash {
        let mut hasher = Sha3::v256();
        hasher.update(data);
        let mut digest = [0u8; 32];
        hasher.finalize(&mut digest);
        let digest_hex = hex_from_bin(&digest);
        Hash::from_digest(&digest_hex)
    }

    pub fn join(&self, other_hash: &Hash) -> Hash {
        let mut hasher = Sha3::v256();
        hasher.update(self.digest_hex[2..].as_bytes());
        hasher.update(other_hash.digest_hex[2..].as_bytes());
        let mut digest = [0u8; 32];
        hasher.finalize(&mut digest);
        let digest_hex = hex_from_bin(&digest);
        let mut ret = Hash::from_digest(&digest_hex);
        ret.left = Some(self.digest_hex.clone());
        ret.right = Some(other_hash.digest_hex.clone());
        ret
    }

    pub fn children(&self) -> (bool, Option<String>, Option<String>) {
        match (self.left.clone(), self.right.clone()) {
            (Some(left), Some(right)) => (true, Some(left), Some(right)),
            _ => (false, None, None),
        }
    }

    pub fn iterated_merkle(&self, level: u32) -> Hash {
        let iterated_hashes = &mut iterateds.lock().unwrap();

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
            iterated_hashes.get_mut(&self).unwrap().push(highest_level.clone());
        }

        highest_level
    }

    fn is_zero(&self) -> bool {
        self == &zero_hash()
    }

    fn is_of_type_hash(&self, x: &Hash) -> bool {
        self.eq(x)
    }
}

impl ToString for Hash {
    fn to_string(&self) -> String {
        self.digest_hex.clone()
    }
}

fn zero_bytes32() -> String {
    "0x0000000000000000000000000000000000000000000000000000000000000000".to_string()
}

fn zero_hash() -> Hash {
    Hash::from_digest(&zero_bytes32())
}
