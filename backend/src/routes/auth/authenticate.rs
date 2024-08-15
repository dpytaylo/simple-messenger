use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use common::{
    entity::user::{Email, Password},
    routes::auth::authenticate::{AuthenticateError, AuthenticateRequest, AuthenticateResponse},
};
use garde::Valid;
use http::StatusCode;
use rpc::server::error::{IntoProcFailure, ProcedureError};
use scrypt::{
    password_hash::{PasswordHash, PasswordVerifier},
    Scrypt,
};
use service::query::Query;
use strum::IntoStaticStr;
use thiserror::Error;

use super::generate_jwt_token;
use crate::{error::wrap_error, extractors::jsonv::JsonV, state::ServerState};

#[derive(Debug, Error, IntoStaticStr)]
pub enum AuthenticateServerError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<AuthenticateError> for AuthenticateServerError {
    fn into_procedure_error(self) -> impl IntoProcFailure<AuthenticateError> {
        error!(error = %error);

        match self {
            AuthenticateServerError::InvalidCredentials => AuthenticateError::InvalidCredentials,
            AuthenticateServerError::Other(_) => AuthenticateError::Other,
        }
    }
}

// impl IntoResponse for AuthenticateError {
//     fn into_response(self) -> Response {
//         let code = match self {
//             Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
//             Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
//         };

//         wrap_error(code, self)
//     }
// }

// impl Into<AuthenticateClientError> for AuthenticateError {
//     fn into(self) -> AuthenticateClientError {
//         match self {
//             Self::InvalidCredentials => AuthenticateClientError::InvalidCredentials,
//             Self::Other(_) => AuthenticateClientError::Other,
//         }
//     }
// }

pub async fn authenticate(
    state: Arc<ServerState>,
    payload: Valid<AuthenticateRequest>,
) -> Result<AuthenticateResponse, AuthenticateError> {
    let AuthenticateRequest {
        email: Email(email),
        password: Password(password),
    } = payload.into_inner();

    let Some(user) = Query::find_user_by_email(&state.db, &email)
        .await
        .context("failed to find user by email")?
    else {
        return Err(AuthenticateError::InvalidCredentials);
    };

    let Some(db_password) = user.password else {
        return Err(AuthenticateError::InvalidCredentials);
    };

    let parsed_hash = PasswordHash::new(&db_password).context("failed to hash password")?;

    if Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(AuthenticateError::InvalidCredentials);
    }

    let token = generate_jwt_token(&state, user.id.into())?;
    Ok(AuthenticateResponse { token })
}

pub async fn authenticate_route(
    State(state): State<Arc<ServerState>>,
    JsonV(payload): JsonV<AuthenticateRequest>,
) -> Result<Json<AuthenticateResponse>, AuthenticateError> {
    authenticate(state, payload).await.map(|val| Json(val))
}
