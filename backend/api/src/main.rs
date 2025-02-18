use std::{collections::HashMap, env, time::Instant};

use blockchain::{block::Block, chain::Chain, trial};

mod blockchain;
//mod server;
mod transaction;
mod utils;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use hex::decode;
use reqwest::Client;
use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use transaction::{pool::Pool, transaction::Transaction, wallet::Wallet};
use uuid::Uuid;

#[derive(Clone)]
struct TransactionState {
    p: Arc<Mutex<Pool>>,
    w: Arc<Mutex<Wallet>>,
}

#[tokio::main]
async fn main() {
    //x();
    let c = Arc::new(Mutex::new(Chain::new()));
    let p = Arc::new(Mutex::new(Pool::new()));
    let w = Arc::new(Mutex::new(Wallet::new()));

    let t = TransactionState { p, w };

    let port: u16 = env::var("API_PORT")
        .unwrap_or_else(|_| "3001".to_string()) // Default to 4000
        .parse()
        .expect("Invalid PORT number");

    let router = Router::new()
        .route("/api/chain", get(get_chain))
        .route("/api/mine", post(mine_block))
        .route("/api/replace", post(replace_chain))
        .with_state(c)
        .route("/api/transactions", get(get_pool))
        .route("/api/transact", post(create_transaction))
        .with_state(t);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Listening at {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn get_chain(State(c): State<Arc<Mutex<Chain>>>) -> Json<Value> {
    let c = c.lock().unwrap();
    match c.validate() {
        Ok(_) => {
            let chain = json!(&c.chain);
            Json(chain)
        }
        Err(_) => Json(json!("{}")),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BlockData {
    data: String,
}

async fn mine_block(State(c): State<Arc<Mutex<Chain>>>, Json(data): Json<BlockData>) {
    let mut c = c.lock().unwrap();
    c.add(data.data.as_str());

    tokio::spawn(notify_p2p_server(c.chain.clone()));

    //Redirect::permanent("/api/chain")
}

async fn replace_chain(State(c): State<Arc<Mutex<Chain>>>, Json(chain): Json<Vec<Block>>) {
    let mut c = c.lock().unwrap();

    match c.replace(chain) {
        Some(ch) => {
            tokio::spawn(notify_p2p_server(ch));
        }
        None => (),
    };

    //Redirect::permanent("/api/chain")
}

async fn get_pool(State(t): State<TransactionState>) -> Json<Value> {
    let p = t.p.lock().unwrap();

    let chain = json!(&p.transactions);

    Json(chain)
}

#[derive(Debug, Serialize, Deserialize, Clone)]

struct TransactionData {
    receiver: PublicKey,
    amount: f32,
}

async fn create_transaction(State(t): State<TransactionState>, Json(data): Json<TransactionData>) {
    let mut p = t.p.lock().unwrap();
    let mut w = t.w.lock().unwrap();
    w.send(&data.receiver, data.amount, &mut p);
}

fn x() {
    //trial();
    let mut tp = Pool::new();
    let mut w1 = Wallet::new();
    let w2 = Wallet::new();

    // w1.send(&w2, 5., &mut tp);

    println!("{:?}", tp);

    //w1.send(&w2, 10., &mut tp);

    /*match t.verify() {
        Ok(_) => println!("Success"),
        Err(_) => println!("Fail"),
    }*/

    println!("{:?}", tp);

    let w3 = Wallet::new();

    //w1.send(&w3, 5., &mut tp);

    println!("{:?}", tp);
}

#[derive(serde::Serialize)]
struct BlockchainMessage<T> {
    r#type: String,
    data: T,
}

pub async fn notify_p2p_server(chain: Vec<Block>) {
    let client = Client::new();

    let port: u16 = env::var("API_PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .expect("Invalid API_PORT number");

    let p2p_url = format!("http://127.0.0.1:{}/broadcast", port + 3000); // Target P2P API

    // Create the BlockchainMessage object
    let message = BlockchainMessage {
        r#type: "CHAIN".to_string(),
        data: chain,
    };

    // Send request to P2P server
    match client.post(p2p_url).json(&message).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!("✅ Successfully notified P2P server.");
            } else {
                eprintln!(
                    "❌ P2P server responded with an error: {}",
                    response.status()
                );
            }
        }
        Err(err) => {
            eprintln!("❌ Failed to notify P2P server: {}", err);
        }
    }
}
