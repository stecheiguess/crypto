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
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use transaction::{pool::Pool, transaction::Transaction, wallet::Wallet};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    //x();
    let c = Arc::new(Mutex::new(Chain::new()));

    let port: u16 = env::var("API_PORT")
        .unwrap_or_else(|_| "3001".to_string()) // Default to 4000
        .parse()
        .expect("Invalid PORT number");

    let router = Router::new()
        .route("/api/chain", get(get_chain))
        .route("/api/mine", post(mine_block))
        .route("/api/replace", post(replace_chain))
        .with_state(c);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Listening at {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn get_chain(State(c): State<Arc<Mutex<Chain>>>) -> Json<Value> {
    let c = c.lock().unwrap();

    let chain = json!(&c.chain);

    Json(chain)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Data {
    data: String,
}

async fn notify_p2p_server(chain: Vec<Block>) {
    let client = Client::new();

    let port: u16 = env::var("API_PORT")
        .unwrap_or_else(|_| "3001".to_string()) // Default to 4000
        .parse()
        .expect("Invalid PORT number");

    let p2p_url = format!("http://127.0.0.1:{}/broadcast", port + 3000); // P2P server API endpoint - 6001

    let response = client.post(p2p_url).json(&json!(chain)).send().await;

    if let Err(err) = response {
        eprintln!("Failed to notify P2P server: {}", err);
    }
}

async fn mine_block(State(c): State<Arc<Mutex<Chain>>>, Json(data): Json<Data>) {
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

fn x() {
    //trial();
    let mut tp = Pool::new();
    let mut w1 = Wallet::new();
    let w2 = Wallet::new();

    w1.send(&w2, 5., &mut tp);

    println!("{:?}", tp);

    w1.send(&w2, 10., &mut tp);

    /*match t.verify() {
        Ok(_) => println!("Success"),
        Err(_) => println!("Fail"),
    }*/

    println!("{:?}", tp);

    let w3 = Wallet::new();

    w1.send(&w3, 5., &mut tp);

    println!("{:?}", tp);
}
