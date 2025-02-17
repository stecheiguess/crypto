use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub address: String,
    pub amount: f32,
}

impl Output {
    pub fn new(address: PublicKey, amount: f32) -> Self {
        Output {
            address: address.to_string(),
            amount,
        }
    }
}
