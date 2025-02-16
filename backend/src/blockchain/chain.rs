use crate::blockchain::block::Block;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct Chain {
    difficulty: usize,
    pub chain: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        let mut chain = Vec::new();
        chain.push(Block::genesis());
        Chain {
            difficulty: 4,
            chain,
        }
    }

    pub fn add(&mut self, data: &str) {
        let b = Block::new(data, self.chain.len());
        self.mine(b);
    }

    fn push(&mut self, block: Block) {
        self.chain.push(block);
    }

    fn pop(&mut self) {
        self.chain.pop();
    }

    fn mine(&mut self, mut block: Block) {
        match self.chain.last() {
            Some(b) => {
                block.prev = b.hash();
            }
            None => (),
        }

        while block.hash().0[..self.difficulty] != "0".repeat(self.difficulty) {
            block.nonce += 1;
        }

        self.push(block);
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

    pub fn replace(&mut self, new_chain: Vec<Block>) {
        if new_chain.len() <= self.chain.len() {
            return;
        }

        match Chain::check(&new_chain) {
            Ok(_) => self.chain = new_chain,
            Err(_) => println!("new chain is not valid."),
        }
    }
}
