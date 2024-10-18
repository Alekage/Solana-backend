use crate::{LoginError, Result};
use axum::{Router, routing::post, Json};
use serde::Deserialize;
use serde_json::{Value, json};
use log::info;

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // TODO: Implement real db/auth login
    if payload.username != "solana" || payload.pwd != "memesolana" {
        return Err(LoginError::LoginFail)
    }

    // TODO: Set cookies    

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));    

    info!("User {} has logged in", payload.username);

    Ok(body)

}