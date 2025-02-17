use secp256k1::{ecdsa::Signature, PublicKey};
use uuid::Uuid;
use wallet::Wallet;

mod input;
mod output;
pub mod wallet;

#[derive(Copy, Clone)]
pub struct Transaction {
    id: Uuid,
    sender: PublicKey,
    amount: f32,
    receiver: PublicKey,
}

impl Transaction {
    pub fn new(sender: &Wallet, receiver: &Wallet, amount: f32) -> Result<Transaction, String> {
        if amount > sender.balance {
            return Err("amount greater than balance.".to_owned());
        }

        Ok(Transaction {
            id: Uuid::new_v4(),
            sender: sender.public,
            amount,
            receiver: receiver.public,
        })
    }
}
