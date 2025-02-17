use secp256k1::PublicKey;

use super::transaction::Transaction;

#[derive(Debug)]
pub struct Pool {
    transactions: Vec<Transaction>,
}

impl Pool {
    pub fn new() -> Self {
        Pool {
            transactions: Vec::new(),
        }
    }

    pub fn update(&mut self, transaction: Transaction) {
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

    pub fn check(&mut self, address: PublicKey) -> Option<&mut Transaction> {
        self.transactions
            .iter_mut()
            .find(|t| t.input.address == address)
    }
}
