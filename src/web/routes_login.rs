use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    pwd: String
}