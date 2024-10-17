#![allow(unused)]

use std::os::unix::net::SocketAddr;

use axum::{Router, routing::get, response::Html};

async fn hello() -> &'static str {
    "Hello Solana World"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/hello", get(hello));
        
    let addr = "0.0.0.0:3000";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}