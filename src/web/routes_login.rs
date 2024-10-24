use crate::{LoginError, Result};
use crate::web::jwt::generate_jwt;
use crate::web::jwt::{LoginPayload};
use axum::{Router, routing::post, Json};
use tower_cookies::{Cookies, Cookie};
use serde::Deserialize;
use serde_json::{Value, json};
use log::info;
use crate::web::AUTH_TOKEN;



pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookie: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // TODO: Implement real db/auth login
    if payload.wallet.is_empty() || payload.username.is_empty() || payload.password.is_empty() {
        return Err(LoginError::LoginFail)
    };

    // Create cookie 
    let jwt = match generate_jwt(&payload) {
        Ok(jwt) => jwt,
        Err(e) => return Err(LoginError::LoginFail)
    };

    info!("JWT Generated!");

    // FIX: Implement real auth-token generation/signature
    cookie.add(Cookie::new(AUTH_TOKEN, jwt.clone()));    

    let body = Json(json!({
        "result": {
            "success": true,
            "loginResponse": jwt
        }
    }));    

    info!("User {} has logged in", payload.username);

    Ok(body)

}