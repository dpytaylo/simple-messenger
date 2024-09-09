use api::routes::auth::oauth::OAuthClientError;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Deserialize;
use strum::IntoStaticStr;
use thiserror::Error;

use crate::error::wrap_error;

pub mod discord;
pub mod google;

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Error, IntoStaticStr)]
pub enum OAuthError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for OAuthError {
    fn into_response(self) -> Response {
        let code = match self {
            OAuthError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        wrap_error(code, self)
    }
}

impl Into<OAuthClientError> for OAuthError {
    fn into(self) -> OAuthClientError {
        match self {
            OAuthError::Other(_) => OAuthClientError::Other,
        }
    }
}
