use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Output {
    pub address: PublicKey,
    pub amount: f64,
}

impl Output {
    pub fn new(address: &PublicKey, amount: f64) -> Self {
        Output {
            address: address.to_owned(),
            amount,
        }
    }
}
