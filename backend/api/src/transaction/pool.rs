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
            .find(|t| t.input.unwrap().address == address)
    }

    /// 🔹 Filter valid transactions from the pool
    pub fn valid(&self) -> Vec<Transaction> {
        self.transactions
            .iter()
            .filter(|transaction| {
                let output_total: f64 = transaction.outputs.iter().map(|o| o.amount).sum();

                // 🔥 Check if input amount matches output total
                if transaction.input.unwrap().amount != output_total {
                    println!(
                        "❌ Invalid transaction from {}",
                        transaction.input.unwrap().address
                    );
                    return false;
                }

                // 🔥 Verify transaction signature
                match transaction.verify() {
                    Ok(_) => true,
                    Err(_) => false,
                }
            })
            .cloned()
            .collect()
    }

    pub fn clear(&mut self) {
        self.transactions = Vec::new();
    }
}
