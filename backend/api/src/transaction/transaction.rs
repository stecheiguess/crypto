use super::input::Input;
use super::output::Output;
use super::wallet::Wallet;
use bincode::serialize;
use hex::decode;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};
use uuid::Uuid;

use crate::utils::hash::Hash;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub outputs: Vec<Output>,
    pub input: Input,
}

impl Transaction {
    pub fn new(sender: &Wallet, receiver: &Wallet, amount: f32) -> Result<Transaction, String> {
        if amount > sender.balance {
            return Err("amount greater than balance.".to_owned());
        }

        let mut outputs = Vec::new();
        outputs.push(Output::new(sender.public, sender.balance - amount));
        outputs.push(Output::new(receiver.public, amount));

        Ok(Transaction {
            id: Uuid::new_v4(),
            outputs: outputs.clone(),
            input: Input::new(&sender, &outputs),
        })
    }

    pub fn update(&mut self, sender: &Wallet, receiver: &Wallet, amount: f32) {
        match self
            .outputs
            .iter_mut()
            .find(|n| n.address == sender.public.to_string())
        {
            Some(sender_output) => {
                if amount > sender_output.amount {
                    return;
                }

                sender_output.amount -= amount;
                self.outputs.push(Output::new(receiver.public, amount));
                self.input = Input::new(&sender, &self.outputs)
            }
            None => (),
        };
    }

    pub fn verify(&self) -> Result<(), ()> {
        let s = Secp256k1::new();

        let hash = Hash::new(serialize(&self.outputs).unwrap()).unwrap();

        match s.verify_ecdsa(
            &Message::from_digest(decode(hash.0).unwrap().try_into().unwrap()),
            &self.input.signature,
            &self.input.address,
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
