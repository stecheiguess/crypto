use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils::{hash::Hash, time};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Block {
    pub timestamp: u64,
    pub nonce: usize,
    pub prev: Hash,
    pub height: usize,
    pub data: String,
    pub difficulty: usize,
}

impl Block {
    pub fn new(prev: Block, data: &str) -> Self {
        Block {
            timestamp: time(),
            nonce: 0,
            prev: prev.hash(),
            height: prev.height + 1,
            data: data.to_string(),
            difficulty: prev.difficulty,
        }
    }

    pub fn genesis() -> Self {
        Block {
            timestamp: 0,
            nonce: 0,
            prev: Hash::blank(),
            height: 0,
            data: "".to_string(),
            difficulty: 5,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::new(
            format!(
                "{}{}{}{}{}{}",
                self.timestamp, self.prev.0, self.nonce, self.data, self.height, self.difficulty
            )
            .as_str(),
        )
        .unwrap()
    }
}
