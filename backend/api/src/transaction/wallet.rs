use hex::decode;
use secp256k1::{ecdsa::Signature, rand::rngs::OsRng, Message, PublicKey, Secp256k1, SecretKey};

use crate::utils::hash::Hash;

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
        tp: &mut Pool,
    ) -> Result<Transaction, String> {
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
}
