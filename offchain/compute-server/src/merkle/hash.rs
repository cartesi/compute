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

    pub fn from_hex(digest_hex: &str) -> Hash {
        assert!(digest_hex.len() == 66);
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

impl ToString for Hash {
    fn to_string(&self) -> String {
        self.to_hex()
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

fn zero_bytes32() -> [u8; 32] {
    let mut data = [0u8; 32];
    hex::decode_to_slice(
        "0000000000000000000000000000000000000000000000000000000000000000",
        &mut data as &mut [u8],
    )
    .unwrap();
    data
}