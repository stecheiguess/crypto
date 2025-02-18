use std::str::FromStr;

use super::input::Input;
use super::output::Output;
use super::wallet::Wallet;
use bincode::serialize;
use hex::decode;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::hash::Hash;

/*const TOTAL_SUPPLY_CAP: f64 = 1_048_576.0; // 2^20
const INITIAL_REWARD: f64 = 50.0;
const HALVING_INTERVAL: u64 = 100; // Adjust for your chain

/// 🔹 Calculate mining reward based on block height
fn mining_reward(block_height: u64) -> f64 {
    let halvings = block_height / HALVING_INTERVAL;
    let reward = INITIAL_REWARD / 2.0_f64.powi(halvings as i32);
    reward.max(0.0) // Ensure it doesn't go negative
}*/

const REWARD: f64 = 50.;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub outputs: Vec<Output>,
    pub input: Option<Input>,
}

impl Transaction {
    pub fn new(sender: &Wallet, receiver: &PublicKey, amount: f64) -> Result<Transaction, String> {
        if amount > sender.balance {
            return Err("amount greater than balance.".to_string());
        }

        let mut outputs = Vec::new();
        outputs.push(Output::new(&sender.public, sender.balance - amount));
        outputs.push(Output::new(receiver, amount));

        let t = Transaction {
            id: Uuid::new_v4(),
            outputs: outputs.clone(),
            input: Some(Input::new(&sender, &outputs)),
        };

        match t.verify() {
            Ok(_) => Ok(t),
            Err(_) => Err("signature does not match.".to_string()),
        }
    }

    pub fn reward(miner: &PublicKey) -> Transaction {
        let mut outputs = Vec::new();
        outputs.push(Output::new(miner, REWARD));

        Transaction {
            id: Uuid::new_v4(),
            outputs: outputs.clone(),
            input: None,
        }
    }

    pub fn update(&mut self, sender: &Wallet, receiver: &PublicKey, amount: f64) {
        let x = self.clone();

        match self.outputs.iter_mut().find(|n| n.address == sender.public) {
            Some(sender_output) => {
                if amount > sender_output.amount {
                    return;
                }

                sender_output.amount -= amount;
                self.outputs.push(Output::new(receiver, amount));
                self.input = Some(Input::new(&sender, &self.outputs))
            }
            None => (),
        };

        match self.verify() {
            Ok(_) => return,
            Err(_) => {
                *self = x;
            }
        }
    }

    pub fn verify(&self) -> Result<(), ()> {
        let s = Secp256k1::new();

        let hash = Hash::new(serialize(&self.outputs).unwrap()).unwrap();

        match s.verify_ecdsa(
            &Message::from_digest(decode(hash.0).unwrap().try_into().unwrap()),
            &self.input.unwrap().signature,
            &self.input.unwrap().address,
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
