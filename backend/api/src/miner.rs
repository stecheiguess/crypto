use serde_json::json;

use crate::{
    blockchain::{block::Block, chain::Chain},
    transaction::{pool::Pool, transaction::Transaction, wallet::Wallet},
};

pub struct Miner {
    chain: Chain,
    pool: Pool,
    wallet: Wallet,
}

impl Miner {
    pub fn new() -> Self {
        Miner {
            chain: Chain::new(),
            pool: Pool::new(),
            wallet: Wallet::new(),
        }
    }

    pub fn mine(&mut self) -> Chain {
        let mut transactions = self.pool.valid();

        let reward = Transaction::reward(&self.wallet.public);
        transactions.push(&reward);

        self.chain.add(json!(transactions).as_str().unwrap());
    }
}
