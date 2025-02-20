use std::time::Instant;

use crate::{blockchain::block::Block, utils::time};
use serde::{Deserialize, Serialize};
use to_binary::BinaryString;

const MINE_RATE: u64 = 1;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Chain {
    pub chain: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        let mut chain = Vec::new();
        chain.push(Block::genesis());
        Chain { chain }
    }

    pub fn add(&mut self, data: &str) -> Block {
        let b = self.mine(data);
        self.push(b.clone());
        b
    }

    fn push(&mut self, block: Block) {
        self.chain.push(block);
    }

    fn pop(&mut self) {
        self.chain.pop();
    }

    fn mine(&mut self, data: &str) -> Block {
        let b = self.chain.last().unwrap();

        let mut block = Block::new(b.clone(), data);

        while hex_to_binary(block.hash().0.as_str())[..block.difficulty]
            != "0".repeat(block.difficulty)
        {
            block.timestamp = time();

            block.nonce += 1;

            block.difficulty = {
                let diff = b.difficulty;
                if diff < 1 {
                    1
                } else if ((block.timestamp - b.timestamp) > MINE_RATE) {
                    diff - 1
                } else {
                    diff + 1
                }
            };
        }

        block
    }

    fn check(chain: &Vec<Block>) -> Result<&str, &str> {
        for (i, block) in chain.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if block.prev != chain[i - 1].hash() {
                return Err("Blockchain is not valid.");
            }
        }

        return Ok("Blockchain is valid.");
    }

    pub fn validate(&self) -> Result<&str, &str> {
        Chain::check(&self.chain)
    }

    pub fn replace(&mut self, new_chain: Vec<Block>) -> Option<Vec<Block>> {
        match Chain::check(&new_chain) {
            Ok(_) => {
                if new_chain.len() < self.chain.len() {
                    return None;
                }

                if new_chain.len() == self.chain.len()
                    && new_chain.last().unwrap().nonce <= self.chain.last().unwrap().nonce
                {
                    return None;
                }

                self.chain = new_chain;
                Some(self.chain.clone())
            }
            Err(_) => {
                println!("new chain is not valid.");
                None
            }
        }
    }
}

fn hex_to_binary(hex: &str) -> String {
    match hex::decode(hex) {
        Ok(bytes) => bytes
            .iter()
            .map(|byte| format!("{:08b}", byte)) // Convert each byte to 8-bit binary
            .collect::<Vec<String>>()
            .join(""),
        Err(e) => format!("Error decoding hex: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &str = "foo";
    static DATA2: &str = "goo";

    fn create() -> (Chain, Chain) {
        (Chain::new(), Chain::new())
    }

    // ✅ Test: Blockchain starts with the Genesis block
    #[test]
    fn genesis() {
        assert_eq!(create().0.chain[0], Block::genesis());
    }

    // ✅ Test: Adding a new block with data
    #[test]
    fn add() {
        let mut c = create().0;
        c.add(DATA);

        assert_eq!(c.chain.last().unwrap().data, DATA);
    }

    // ✅ Test: Validates a valid chain
    #[test]
    fn valid() {
        let mut c: Chain = create().0;
        c.add(DATA);

        assert_eq!(c.validate().unwrap(), "Blockchain is valid.");
    }

    #[test]
    fn verify_nonce() {
        let mut c: Chain = create().0;
        c.add(DATA);

        let b = c.chain.last().unwrap();

        assert_eq!(
            hex_to_binary(b.hash().0.as_str())[..b.difficulty],
            "0".repeat(b.difficulty)
        );
    }
    // ✅ Test: Invalidates a chain with a corrupt Genesis block
    #[test]
    fn invalid_genesis() {
        let mut c: Chain = create().0;

        c.chain[0].data = "Bad data".to_string(); // ❌ Corrupt Genesis Block

        assert_ne!(c.chain[0].hash(), Block::genesis().hash());
    }

    // ✅ Test: Invalidates a chain with corrupted data
    #[test]
    fn invalid_chain() {
        let (c1, mut c2) = create();

        c2.add(DATA);
        c2.chain[0].data = "Not foo".to_string(); // ❌ Corrupt Data in Block

        assert_ne!(c1.validate(), c2.validate());
        assert_eq!(Chain::check(&c2.chain), Err("Blockchain is not valid."));
    }

    // ✅ Test: Replaces the chain with a valid chain
    #[test]
    fn replace_chain() {
        let (mut c1, mut c2) = create();

        c2.add(DATA2);
        assert!(c1.replace(c2.chain.clone()).is_some()); // ✅ Replacement should succeed
        assert_eq!(c1.chain.len(), 2); // ✅ New chain has length 2
    }

    // ✅ Test: Does not replace chain if it's shorter or equal
    #[test]
    fn replace_chain_invalid_length() {
        let (mut c1, mut c2) = create();

        c1.add(DATA); // ✅ Original chain has length 2
        assert!(c1.replace(c2.chain.clone()).is_none()); // ❌ Replacement should fail
        assert_eq!(c1.chain.len(), 2); // ✅ Chain length should remain unchanged
    }
}
