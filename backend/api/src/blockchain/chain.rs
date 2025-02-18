use std::time::Instant;

use crate::{blockchain::block::Block, utils::time};
use serde::{Deserialize, Serialize};
use to_binary::BinaryString;

#[derive(Serialize, Deserialize, Debug)]

pub struct Chain {
    pub chain: Vec<Block>,
}

const MINE_RATE: u64 = 1;

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
        let prev = b.hash();
        let difficulty = b.difficulty;

        let mut block = Block::new(prev, self.chain.len(), data, difficulty);

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
