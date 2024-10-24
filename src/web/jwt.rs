use hyper::StatusCode;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct LoginPayload {
    pub wallet: String,
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

pub fn generate_jwt(payload: &LoginPayload) -> Result<String, StatusCode> {
    let secret = "randString".to_string();
    let claim = Claims {
        sub: payload.username.clone(),
        iat: chrono::Utc::now().timestamp() as usize,
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref())
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR )
}