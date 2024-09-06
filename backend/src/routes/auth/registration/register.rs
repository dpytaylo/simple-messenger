use std::sync::Arc;

use anyhow::Context;
use api::{
    entities::registration_kind::RegistrationKind,
    routes::auth::registration::register::{RegisterError, RegisterRequest, RegisterResponse},
};
use axum::extract::State;
use http::StatusCode;
use rand_chacha::rand_core::OsRng;
use rpc::server::error::{IntoProcFailure, ProcedureError};
use scrypt::{
    password_hash::{PasswordHasher, SaltString},
    Scrypt,
};
use thiserror::Error;
use tracing::instrument;

use crate::{routes::auth::generate_jwt_token, state::ServerState};

#[derive(Debug, Error)]
pub enum RegisterServerError {
    #[error("account with the same email already exists")]
    AccountWithSameEmailAlreadyExists,

    #[error("no password provided")]
    NoPassword,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<RegisterError> for RegisterServerError {
    fn into_procedure_error(self) -> impl IntoProcFailure<RegisterError> {
        match self {
            Self::AccountWithSameEmailAlreadyExists => {
                RegisterError::AccountWithSameEmailAlreadyExists.into_proc_failure()
            }
            Self::NoPassword => {
                (StatusCode::BAD_REQUEST, RegisterError::NoPassword).into_proc_failure()
            }
            Self::Other(_) => RegisterError::Other.into_proc_failure(),
        }
    }
}

#[instrument(skip(state), err)]
pub async fn register(
    State(state): State<Arc<ServerState>>,
    request: RegisterRequest,
) -> Result<RegisterResponse, RegisterServerError> {
    if db::user::find_by_email(&state.db, &request.email)
        .await
        .context("failed to find user by email")?
        .is_some()
    {
        return Err(RegisterServerError::AccountWithSameEmailAlreadyExists);
    }

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Scrypt
        .hash_password(request.password.value().as_bytes(), &salt)
        .context("failed to hash password")?
        .to_string();

    let user = db::user::create(
        &state.db,
        RegistrationKind::Email,
        &request.email,
        Some(&password_hash),
        &request.name,
        request.avatar.as_ref(),
    )
    .await
    .context("failed to create user")?;

    let token = generate_jwt_token(&state, user.id.to_string())?;
    Ok(RegisterResponse { token })
}
