use std::fmt;

use hex;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Hash {
    data: [u8; 32],
}

impl Hash {
    pub fn new(data: [u8; 32]) -> Self {
        return Hash{
            data: data,
        }
    }

    pub fn from_data(digest_data: Vec<u8>) -> Hash {
        let data: [u8; 32] = digest_data.try_into().unwrap();
        Hash::new(data)
    }

    pub fn from_hex(digest_hex: &str) -> Hash {
        let mut data = [0u8; 32];
        hex::decode_to_slice(&digest_hex, &mut data as &mut [u8]).unwrap();
        Hash::new(data)
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.data)
    }

    pub fn is_zero(&self) -> bool {
        self.data == zero_bytes32()
    }
}

impl Default for Hash {
    fn default() -> Self {
        Hash::new(zero_bytes32())
    }
}

impl From<[u8; 32]> for Hash {
    fn from(data: [u8; 32]) -> Self {
        Hash { data: data }
    }
}

impl From<Hash> for [u8; 32] {
    fn from (hash: Hash) -> Self {
        hash.data
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
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