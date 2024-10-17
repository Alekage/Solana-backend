#![allow(unused)]

use std::net::SocketAddr;

use axum::{Router, routing::get, response::Html};
use log::info;

async fn hello_handler() -> &'static str {
    "Hello Solana World"
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new().route(
        "/hello", get(hello_handler));
        
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Listening on address: {:?}", addr);

    axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}