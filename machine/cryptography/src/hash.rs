use std::collections::HashMap;
use once_cell::sync::OnceCell;
use sha3::{Digest, Keccak256};

fn hex_from_bin(bin: &[u8]) -> String {
    assert_eq!(bin.len(), 32);
    let hex_chars: Vec<String> = bin.iter().map(|c| format!("{:02x}", c)).collect();
    format!("0x{}", hex_chars.join(""))
}
#[derive(Eq, Hash, PartialEq, Clone, Debug, Default)]
pub struct Hash {
    pub digest_hex: String,
    left: Option<String>,
    right: Option<String>,
}

static INTERNALIZED_HASHES: OnceCell<HashMap<String, Hash>> = OnceCell::new();
static ITHERADETS: OnceCell<HashMap<Hash, Vec<Hash>>> = OnceCell::new();

impl Hash {
    pub fn from_digest(digest_hex: &str) -> Hash {
        assert!(digest_hex.len() == 66);

        if let Some(x) = INTERNALIZED_HASHES
            .get_or_init(|| HashMap::new())
            .get(digest_hex)
            .clone()
        {
            return x.clone();
        }

        let h = Hash {
            digest_hex: digest_hex.to_string(),
            left: None,
            right: None,
        };
        let mut extended_itheradets = ITHERADETS.get_or_init(|| HashMap::new()).clone();
        let mut extended_internalized_hashes =
            INTERNALIZED_HASHES.get_or_init(|| HashMap::new()).clone();

        extended_itheradets.insert(h.clone(), vec![h.clone()]);
        extended_internalized_hashes.insert(digest_hex.to_string(), h.clone());

        ITHERADETS.set(extended_itheradets);
        INTERNALIZED_HASHES.set(extended_internalized_hashes);
        h
    }

    pub fn from_digest_bin(digest_bin: &[u8]) -> Hash {
        let digest_hex = hex_from_bin(digest_bin);
        Hash::from_digest(&digest_hex)
    }

    fn from_data(data: &[u8]) -> Hash {
        let mut keccak = Keccak256::new();
        keccak.update(&hex::decode(&data).unwrap());
        let digest = keccak.finalize();
        let digest_hex = hex_from_bin(&digest);
        Hash::from_digest(&digest_hex)
    }

    pub fn join(&self, other_hash: &Hash) -> Hash {
        let mut keccak = Keccak256::new();
        keccak.update(&hex::decode(&self.digest_hex[2..]).unwrap());
        keccak.update(&hex::decode(&other_hash.digest_hex[2..]).unwrap());
        let digest = keccak.finalize();
        let digest_hex = hex_from_bin(&digest);
        let mut ret = Hash::from_digest(&digest_hex);
        ret.left = Some(self.digest_hex.clone());
        ret.right = Some(other_hash.digest_hex.clone());
        ret
    }

    pub fn children(&self) -> (Option<String>, Option<String>) {
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
}

impl ToString for Hash {
    fn to_string(&self) -> String {
        self.digest_hex.clone()
    }
}

fn zero_bytes32() -> String {
    "0x0000000000000000000000000000000000000000000000000000000000000000".to_string()
}

pub fn zero_hash() -> Hash {
    Hash::from_digest(&zero_bytes32())
}
