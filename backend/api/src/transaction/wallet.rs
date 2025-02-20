use hex::decode;
use secp256k1::{ecdsa::Signature, rand::rngs::OsRng, Message, PublicKey, Secp256k1, SecretKey};
use serde_json::from_str;

use crate::{blockchain::chain::Chain, utils::hash::Hash};

use super::{pool::Pool, transaction::Transaction};

#[derive(Copy, Clone)]
pub struct Wallet {
    pub balance: f64,
    secret: SecretKey,
    pub public: PublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret, public) = secp.generate_keypair(&mut OsRng);

        Wallet {
            balance: 50.,
            secret,
            public,
        }
    }

    pub fn sign(&self, hash: Hash) -> Signature {
        let secp = Secp256k1::new();
        let message = Message::from_digest(decode(hash.0).unwrap().try_into().unwrap());
        return secp.sign_ecdsa(&message, &self.secret);
    }

    pub fn send(
        &mut self,
        receiver: &PublicKey,
        amount: f64,
        c: &Chain,
        tp: &mut Pool,
    ) -> Result<Transaction, String> {
        self.balance = self.calculate_balance(c);
        if amount > self.balance {
            return Err("amount greater than balance.".to_string());
        }

        match tp.check(self.public) {
            Some(t) => {
                t.update(&self, receiver, amount);
                return Ok(t.clone());
            }
            None => {
                let t = Transaction::new(&self, receiver, amount).unwrap();
                tp.update(t.clone());
                return Ok(t);
            }
        }
    }

    pub fn calculate_balance(&mut self, c: &Chain) -> f64 {
        let mut transactions: Vec<Transaction> = Vec::new();

        // 🔹 **Extract Transactions from Blocks**
        for block in c.chain.iter() {
            // Try to parse the block data into a vector of transactions
            if let Ok(block_transactions) = from_str::<Vec<Transaction>>(&block.data) {
                transactions.extend(block_transactions);
            }
        }

        // 🔹 **Find Transactions Sent by This Wallet**
        let wallet_input_txs: Vec<&Transaction> = transactions
            .iter()
            .filter(|tx| {
                if let Some(input) = tx.input {
                    input.address == self.public
                } else {
                    false
                }
            })
            .collect();

        let mut start_time: u64 = 0;

        // 🔹 **Find the Most Recent Transaction Output Affecting This Wallet**
        if !wallet_input_txs.is_empty() {
            if let Some(recent_input_tx) = wallet_input_txs
                .iter()
                .max_by_key(|tx| tx.input.unwrap().timestamp)
            {
                // Find the most recent balance update
                if let Some(output) = recent_input_tx
                    .outputs
                    .iter()
                    .find(|o| o.address == self.public)
                {
                    self.balance = output.amount;
                    start_time = recent_input_tx.input.unwrap().timestamp;
                }
            }
        }

        // 🔹 **Apply Transactions After the Last Known Balance Update**
        for transaction in transactions.iter() {
            if let Some(input) = transaction.input {
                if input.timestamp > start_time {
                    for output in transaction.outputs.iter() {
                        if output.address == self.public {
                            self.balance += output.amount;
                        }
                    }
                }
            }
        }

        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn balance_check() {
        let mut w = Wallet::new();
        let mut w2 = Wallet::new();
        let mut c = Chain::new();
        let mut p = Pool::new();
        let add = 10.;
        for i in 0..3 {
            w.send(&w2.public, add, &c, &mut p);
        }

        c.add(json!(p.transactions).to_string().as_str());

        assert_eq!(w2.calculate_balance(&c), 80.);
        assert_eq!(w.calculate_balance(&c), 20.);
    }

    #[test]
    fn invalid_transaction() {
        let mut w = Wallet::new();
        let mut w2 = Wallet::new();
        let mut c = Chain::new();
        let mut p = Pool::new();

        let b = w.calculate_balance(&c);

        w2.send(&w.public, 60., &c, &mut p);

        c.add(json!(p.transactions).to_string().as_str());
        p.clear();

        w.send(&w2.public, 10., &c, &mut p);

        c.add(json!(p.transactions).to_string().as_str());
        p.clear();

        assert_eq!(w2.calculate_balance(&c), b + 10.);
        assert_eq!(w.calculate_balance(&c), b - 10.);
    }
}
