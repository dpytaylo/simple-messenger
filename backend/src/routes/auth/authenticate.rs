use std::sync::Arc;

use anyhow::Context;
use axum::extract::State;
use backend_api::{
    entities::user::{Email, Password},
    routes::auth::authenticate::{AuthenticateError, AuthenticateRequest, AuthenticateResponse},
};
use garde::Valid;
use rpc::server::error::{IntoProcFailure, ProcedureError};
use scrypt::{
    password_hash::{PasswordHash, PasswordVerifier},
    Scrypt,
};
use backend_db::query::Query;
use thiserror::Error;
use tracing::instrument;

use super::generate_jwt_token;
use crate::state::ServerState;

#[derive(Debug, Error)]
pub enum AuthenticateServerError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<AuthenticateError> for AuthenticateServerError {
    fn into_procedure_error(self) -> impl IntoProcFailure<AuthenticateError> {
        match self {
            AuthenticateServerError::InvalidCredentials => AuthenticateError::InvalidCredentials,
            AuthenticateServerError::Other(_) => AuthenticateError::Other,
        }
    }
}

#[instrument(skip(state), err)]
pub async fn authenticate(
    State(state): State<Arc<ServerState>>,
    payload: Valid<AuthenticateRequest>,
) -> Result<AuthenticateResponse, AuthenticateServerError> {
    let AuthenticateRequest {
        email: Email(email),
        password: Password(password),
    } = payload.into_inner();

    let Some(user) = Query::find_user_by_email(&state.db, &email)
        .await
        .context("failed to find user by email")?
    else {
        return Err(AuthenticateServerError::InvalidCredentials);
    };

    let Some(db_password) = user.password else {
        return Err(AuthenticateServerError::InvalidCredentials);
    };

    let parsed_hash = PasswordHash::new(&db_password).context("failed to hash password")?;

    if Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(AuthenticateServerError::InvalidCredentials);
    }

    let token = generate_jwt_token(&state, user.id.into())?;
    Ok(AuthenticateResponse { token })
}
