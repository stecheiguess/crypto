use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils::{hash::Hash, time};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub timestamp: u64,
    pub nonce: usize,
    pub prev: Hash,
    pub index: usize,
    pub data: String,
    pub difficulty: usize,
}

impl Block {
    pub fn new(prev: Hash, index: usize, data: &str, difficulty: usize) -> Self {
        Block {
            timestamp: time(),
            nonce: 0,
            prev,
            index,
            data: data.to_string(),
            difficulty,
        }
    }

    pub fn genesis() -> Self {
        Block {
            timestamp: 0,
            nonce: 0,
            prev: Hash::blank(),
            index: 0,
            data: "".to_string(),
            difficulty: 3,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::new(
            format!(
                "{}{}{}{}{}{}",
                self.timestamp, self.prev.0, self.nonce, self.data, self.index, self.difficulty
            )
            .as_str(),
        )
        .unwrap()
    }
}

// TODO: dynamic difficulty.
