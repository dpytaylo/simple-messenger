use std::sync::Arc;

use anyhow::Context;
use api::routes::auth::authenticate::{
    AuthenticateError, AuthenticateRequest, AuthenticateResponse,
};
use axum::extract::State;
use rpc::server::error::{IntoProcFailure, ProcedureError};
use scrypt::{
    password_hash::{PasswordHash, PasswordVerifier},
    Scrypt,
};
use thiserror::Error;
use tracing::instrument;

use super::generate_jwt_token;
use crate::state::ServerState;

#[derive(Debug, Error)]
pub enum AuthenticateSErr {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<AuthenticateError> for AuthenticateSErr {
    fn into_procedure_error(self) -> impl IntoProcFailure<AuthenticateError> {
        match self {
            AuthenticateSErr::InvalidCredentials => AuthenticateError::InvalidCredentials,
            AuthenticateSErr::Other(_) => AuthenticateError::Other,
        }
    }
}

#[instrument(skip(state), err)]
pub async fn authenticate(
    State(state): State<Arc<ServerState>>,
    request: AuthenticateRequest,
) -> Result<AuthenticateResponse, AuthenticateSErr> {
    let Some(user) = db::user::find_by_email(&state.db, &request.email)
        .await
        .context("failed to find user by email")?
    else {
        return Err(AuthenticateSErr::InvalidCredentials);
    };

    let Some(db_password) = user.password else {
        return Err(AuthenticateSErr::InvalidCredentials);
    };

    let parsed_hash = PasswordHash::new(&db_password).context("failed to hash password")?;

    if Scrypt
        .verify_password(request.password.value().as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(AuthenticateSErr::InvalidCredentials);
    }

    let token = generate_jwt_token(&state, user.id.into())?;
    Ok(AuthenticateResponse { token })
}
