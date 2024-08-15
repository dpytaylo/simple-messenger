use axum::{
    response::{IntoResponse, Response},
    Json,
};
use common::routes::auth::oauth::token::{TokenClientError, TokenResponse};
use http::StatusCode;
use strum::IntoStaticStr;
use thiserror::Error;
use tower_sessions::Session;

use crate::{
    error::wrap_error,
    session::{get_session_value, JWT_TOKEN_KEY},
};

#[derive(Debug, Error, IntoStaticStr)]
pub enum TokenError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        wrap_error(code, self)
    }
}

impl Into<TokenClientError> for TokenError {
    fn into(self) -> TokenClientError {
        match self {
            TokenError::Other(_) => TokenClientError::Other,
        }
    }
}

async fn token(session: Session) -> Result<TokenResponse, TokenError> {
    get_session_value(&session, JWT_TOKEN_KEY)
        .await
        .map_err(Into::into)
}

pub async fn token_route(session: Session) -> Result<Json<TokenResponse>, TokenError> {
    token(session).await.map(|val| Json(val))
}
