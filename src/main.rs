#![allow(unused)]

use axum::{Router, routing::get, response::Html};
use log::info;

async fn hello() -> &'static str {
    "Hello Solana World"
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new().route(
        "/hello", get(hello));
        
    let addr = "0.0.0.0:3000";

    info!("Listening on address: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap(); 

    axum::serve(listener, app).await.unwrap();
}