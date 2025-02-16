use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/block", get(get_block));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Blockchain API running at {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_block() -> &'static str {
    "Returning blockchain block data..."
}
