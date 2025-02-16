use hex;
use sha2::{Digest, Sha256};

#[derive(Debug, PartialEq)]
pub struct Hash(pub String);

impl Hash {
    pub fn new(input: &str) -> Result<Hash, &str> {
        let mut h = Sha256::new();

        h.update(input);

        let out = hex::encode(h.finalize());

        if out.len() != 64 {
            return Err("Hash must be 64 characters long.");
        }
        if !out.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Hash must contain only hexadecimal characters.");
        }

        Ok(Hash(out))
    }

    pub fn blank() -> Self {
        Hash("0".repeat(64))
    }
}
