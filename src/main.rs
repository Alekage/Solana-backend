#![allow(unused)]

use std::net::SocketAddr;
use axum::{extract::{Query,Path }, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;
use log::info;

#[derive(Debug, Deserialize)]
struct HelloParams {
    chain_name: Option<String>
}

async fn hello_handler(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello, {}", name))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new().route(
        "/hello/:name", get(hello_handler));
        
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Listening on address: {:?}", addr);
        // alternative:
        // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        // axum::serve(listener, app).await.unwrap();
    axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}