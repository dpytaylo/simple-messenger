use api::routes::auth::oauth::OAuthClientError;
use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use http::StatusCode;
use serde::Deserialize;
use strum::IntoStaticStr;
use thiserror::Error;
use token::token_route;

use crate::{error::wrap_error, state::ServerStateWrapper};

pub mod discord;
pub mod google;
pub mod token;

pub const AUTH_SUCCESS_PAGE_URL: &str = "/authorization-success";
pub const SIGN_UP_OAUTH2_PAGE_URL: &str = "/sign-up-oauth2";

pub const CSRF_TOKEN_KEY: &str = "csrf-token";
pub const PKCE_VERIFIER_KEY: &str = "pkce-verifier";

pub fn routes() -> Router<ServerStateWrapper> {
    Router::new()
        .nest("/discord", discord::routes())
        .nest("/google", google::routes())
        .route("/token", get(token_route))
}

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
