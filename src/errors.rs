use std::convert::Infallible;

use log::debug;
use serde::Serialize;
use thiserror::Error;
use warp::{
    hyper::StatusCode,
    reject::{InvalidHeader, MissingHeader, Reject},
    Rejection, Reply,
};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("illegal header value: {0}")]
    InvalidHeader(String),
    #[error("missing signing key")]
    MissingSigningKey,
    #[error("key signature was invalid")]
    InvalidSignature,
}

impl Reject for ServerError {}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(ServerError::InvalidSignature) = err.find() {
        code = StatusCode::UNAUTHORIZED;
        message = "Error 401: Unauthorized.";
        debug!("Invalid signature received on token.");
    } else if err.find::<InvalidHeader>().is_some() || err.find::<MissingHeader>().is_some() {
        code = StatusCode::BAD_REQUEST;
        message = "Error 401: Bad Request.";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Error 500: Internal Server Error.";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
