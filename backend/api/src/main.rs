use std::{collections::HashMap, time::Instant};

use blockchain::{chain::Chain, trial};

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
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use transaction::{pool::Pool, transaction::Transaction, wallet::Wallet};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    x();
    let c = Arc::new(Mutex::new(Chain::new()));

    let router = Router::new()
        .route("/api/chain", get(get_chain))
        .route("/api/mine", post(mine_block))
        .with_state(c);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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

async fn mine_block(State(c): State<Arc<Mutex<Chain>>>, Json(data): Json<Data>) -> Redirect {
    let mut c = c.lock().unwrap();
    c.add(data.data.as_str());
    Redirect::permanent("/api/chain")
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
