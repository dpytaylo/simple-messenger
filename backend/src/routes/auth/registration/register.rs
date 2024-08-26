use std::sync::Arc;

use anyhow::Context;
use axum::extract::State;
use common::{
    entity::{
        registration_kind::RegistrationKind,
        user::{Email, Name},
    },
    routes::auth::registration::register::{RegisterError, RegisterRequest, RegisterResponse},
};
use garde::Valid;
use http::StatusCode;
use rand_chacha::rand_core::OsRng;
use rpc::server::error::{IntoProcFailure, ProcedureError};
use scrypt::{
    password_hash::{PasswordHasher, SaltString},
    Scrypt,
};
use service::{
    mutation::{CreateUserData, Mutation},
    query::Query,
};
use thiserror::Error;
use tracing::instrument;

use crate::{routes::auth::generate_jwt_token, state::ServerState, utils::dto_entity::ToEntity};

impl ToEntity<service::RegistrationKind> for RegistrationKind {
    fn to_entity(&self) -> service::RegistrationKind {
        match self {
            Self::Email => service::RegistrationKind::Email,
            Self::Discord => service::RegistrationKind::Discord,
            Self::Google => service::RegistrationKind::Google,
        }
    }
}

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
    request: Valid<RegisterRequest>,
) -> Result<RegisterResponse, RegisterServerError> {
    let RegisterRequest {
        email: Email(email),
        password,
        name: Name(name),
        avatar,
    } = request.into_inner();

    if Query::find_user_by_email(&state.db, &email)
        .await
        .context("failed to find user by email")?
        .is_some()
    {
        return Err(RegisterServerError::AccountWithSameEmailAlreadyExists);
    }

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Scrypt
        .hash_password(password.0.as_bytes(), &salt)
        .context("failed to hash password")?
        .to_string();

    let mut user = Mutation::create_user(
        &state.db,
        CreateUserData {
            kind: RegistrationKind::Email.to_entity(),
            email,
            password: Some(password_hash),
            name,
            avatar,
        },
    )
    .await
    .context("failed to create user")?;

    let token = generate_jwt_token(&state, user.id.take().unwrap().into())?;
    Ok(RegisterResponse { token })
}
