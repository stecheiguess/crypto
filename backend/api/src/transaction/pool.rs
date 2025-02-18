use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

use super::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pool {
    pub transactions: Vec<Transaction>,
}

impl Pool {
    pub fn new() -> Self {
        Pool {
            transactions: Vec::new(),
        }
    }

    pub fn update(&mut self, transaction: Transaction) {
        match transaction.verify() {
            Ok(_) => {
                match self
                    .transactions
                    .iter()
                    .enumerate()
                    .find(|(i, t)| t.id == transaction.id)
                {
                    Some((i, _)) => {
                        self.transactions[i] = transaction;
                    }
                    None => self.transactions.push(transaction),
                }
            }
            Err(_) => (),
        }
    }

    pub fn check(&mut self, address: PublicKey) -> Option<&mut Transaction> {
        self.transactions
            .iter_mut()
            .find(|t| t.input.address == address)
    }
}
