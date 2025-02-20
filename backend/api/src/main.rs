use std::{collections::HashMap, env, time::Instant};

use blockchain::{block::Block, chain::Chain, trial};

mod blockchain;
//mod server;
//mod miner;
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
struct AppState {
    c: Arc<Mutex<Chain>>,
    p: Arc<Mutex<Pool>>,
    w: Arc<Mutex<Wallet>>,
}

#[tokio::main]
async fn main() {
    //x();
    let c = Arc::new(Mutex::new(Chain::new()));
    let p = Arc::new(Mutex::new(Pool::new()));
    let w = Arc::new(Mutex::new(Wallet::new()));

    let s = AppState { c, p, w };

    let port: u16 = env::var("API_PORT")
        .unwrap_or_else(|_| "3001".to_string()) // Default to 4000
        .parse()
        .expect("Invalid PORT number");

    let router = Router::new()
        .route("/api/chain/get", get(get_chain))
        .route("/api/chain/mine", post(mine_block))
        .route("/api/chain/replace", post(replace_chain))
        .route("/api/transaction/get", get(get_pool))
        .route("/api/transaction/create", post(create_transaction))
        .route("/api/transaction/update", post(update_transaction))
        .route("/api/public_key", get(get_public_key))
        .route("/api/mine", get(mine))
        .with_state(s);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Listening at {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn get_chain(State(s): State<AppState>) -> Json<Value> {
    let c = s.c.lock().unwrap();
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

async fn mine_block(State(s): State<AppState>, Json(data): Json<BlockData>) {
    let mut c = s.c.lock().unwrap();
    c.add(data.data.as_str());

    tokio::spawn(notify_p2p_server(c.chain.clone()));

    //Redirect::permanent("/api/chain")
}

async fn replace_chain(State(s): State<AppState>, Json(chain): Json<Vec<Block>>) {
    let mut c = s.c.lock().unwrap();

    match c.replace(chain) {
        Some(ch) => {
            //tokio::spawn(notify_p2p_server(ch));
        }
        None => (),
    };

    //Redirect::permanent("/api/chain")
}

async fn get_pool(State(s): State<AppState>) -> Json<Value> {
    let p = s.p.lock().unwrap();

    let chain = json!(&p.transactions);

    Json(chain)
}

#[derive(Debug, Serialize, Deserialize, Clone)]

struct TransactionData {
    receiver: PublicKey,
    amount: f64,
}

async fn create_transaction(State(s): State<AppState>, Json(data): Json<TransactionData>) {
    let c = s.c.lock().unwrap();
    let mut p = s.p.lock().unwrap();
    let mut w = s.w.lock().unwrap();

    let t = w.send(&data.receiver, data.amount, &c, &mut p).unwrap();
    tokio::spawn(notify_p2p_transaction(t));
}

async fn update_transaction(State(s): State<AppState>, Json(transaction): Json<Transaction>) {
    let mut p = s.p.lock().unwrap();

    p.update(transaction);

    //Redirect::permanent("/api/chain")
}

async fn get_public_key(State(s): State<AppState>) -> Json<Value> {
    let w = s.w.lock().unwrap();

    Json(json!(w.public))
}

async fn mine(State(s): State<AppState>) -> Json<Value> {
    let mut c = match s.c.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("⚠️ Warning: Blockchain mutex was poisoned! Recovering...");
            poisoned.into_inner() // Recover from the poisoned state
        }
    };

    let mut p = match s.p.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("⚠️ Warning: Transaction Pool mutex was poisoned! Recovering...");
            poisoned.into_inner()
        }
    };

    let w = match s.w.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("⚠️ Warning: Wallet mutex was poisoned! Recovering...");
            poisoned.into_inner()
        }
    };

    let mut transactions = p.valid();

    let reward = Transaction::reward(&w.public);
    transactions.push(reward);

    println!("{}", json!(transactions));

    let block = c.add(json!(transactions).to_string().as_str());

    p.clear();

    tokio::spawn(notify_p2p_server(c.chain.clone()));

    Json(json!(block))
}

// notify

pub async fn notify_p2p_server(chain: Vec<Block>) {
    let client = Client::new();

    let port: u16 = env::var("API_PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .expect("Invalid API_PORT number");

    let p2p_url = format!("http://127.0.0.1:{}/chain", port + 3000); // Target P2P API

    // Send request to P2P server
    match client.post(p2p_url).json(&chain).send().await {
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

pub async fn notify_p2p_transaction(transaction: Transaction) {
    let client = Client::new();

    let port: u16 = env::var("API_PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .expect("Invalid API_PORT number");

    let p2p_url = format!("http://127.0.0.1:{}/transaction", port + 3000); // Target P2P API

    // Send request to P2P server
    match client.post(p2p_url).json(&transaction).send().await {
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
