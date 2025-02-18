use bincode::serialize;
use secp256k1::{ecdsa::Signature, PublicKey};
use serde::{Deserialize, Serialize};

use crate::utils::hash::Hash;
use crate::utils::time;

use super::wallet::Wallet;

use super::output::Output;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Input {
    pub timestamp: u64,
    pub amount: f32,
    pub address: PublicKey,
    pub signature: Signature,
}

impl Input {
    pub fn new(sender: &Wallet, outputs: &Vec<Output>) -> Self {
        Input {
            timestamp: time(),
            amount: sender.balance,
            address: sender.public,
            signature: sender.sign(Hash::new(serialize(outputs).unwrap()).unwrap()),
        }
    }
}
