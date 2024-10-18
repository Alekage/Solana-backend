use crate::{LoginError, Result};
use axum::{Router, routing::post, Json};
use tower_cookies::{Cookies, Cookie};
use serde::Deserialize;
use serde_json::{Value, json};
use log::info;
use crate::web::AUTH_TOKEN;

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookie: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // TODO: Implement real db/auth login
    if payload.username != "solana" || payload.pwd != "memesolana" {
        return Err(LoginError::LoginFail)
    }

    // FIX: Implement real auth-token generation/signature
    cookie.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));    

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));    

    info!("User {} has logged in", payload.username);

    Ok(body)

}