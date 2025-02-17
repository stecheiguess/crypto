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

type PeerMap = Arc<Mutex<HashMap<Uuid, mpsc::UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() {
    x();
    let c = Arc::new(Mutex::new(Chain::new()));
    let peer_map = Arc::new(Mutex::new(HashMap::new()));

    let router = Router::new()
        .route("/api/chain", get(get_chain))
        .route("/api/mine", post(mine_block))
        .with_state(c)
        .route("/ws", get(handle_ws))
        .with_state(peer_map);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Blockchain API running at {}", addr);

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

async fn handle_ws(ws: WebSocketUpgrade, State(peer_map): State<PeerMap>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, peer_map))
}

async fn handle_socket(socket: WebSocket, peer_map: PeerMap) {
    let (mut sender, mut receiver) = socket.split();

    let peer_id = Uuid::new_v4();
    println!("New peer connected: {}", peer_id);

    let (tx, rx) = mpsc::unbounded_channel();
    peer_map.lock().unwrap().insert(peer_id, tx);

    let mut rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    let peer_map_clone = peer_map.clone();
    let peer_id_clone = peer_id;

    // Spawn a task to listen for incoming messages
    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                println!("Received: {} from {}", text, peer_id);

                // Broadcast the message to all peers
                let peers = peer_map_clone.lock().unwrap();
                for (&id, tx) in peers.iter() {
                    if id != peer_id_clone {
                        let _ = tx.send(Message::Text(text.clone()));
                    }
                }
            }
        }

        peer_map_clone.lock().unwrap().remove(&peer_id_clone);
        println!("Peer {} disconnected", peer_id_clone);
    });

    // Spawn a task to send messages to this peer
    tokio::spawn(async move {
        while let Some(msg) = rx.next().await {
            let _ = sender.send(msg).await;
        }
    });
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
