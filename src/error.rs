use axum::response::{Response, IntoResponse};
use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
   LoginFail 
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

// Never pass your server error to the client!
// It is a big security exposure