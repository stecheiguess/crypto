use hex::decode;
use secp256k1::{ecdsa::Signature, rand::rngs::OsRng, Message, PublicKey, Secp256k1, SecretKey};

use crate::hash::Hash;

#[derive(Copy, Clone)]
pub struct Wallet {
    pub balance: f32,
    secret: SecretKey,
    pub public: PublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret, public) = secp.generate_keypair(&mut OsRng);

        Wallet {
            balance: 0.,
            secret,
            public,
        }
    }

    pub fn sign(&self, hash: Hash) -> Signature {
        let secp = Secp256k1::new();
        let message = Message::from_digest(decode(hash.0).unwrap().try_into().unwrap());
        return secp.sign_ecdsa(&message, &self.secret);
    }
}
