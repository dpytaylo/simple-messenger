use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use common::{
    entity::user::{Email, Name},
    routes::auth::register::{
        RegisterClientError, RegisterRequest, RegisterResponse, RegistrationKind,
    },
};
use garde::Valid;
use http::StatusCode;
use rand_chacha::rand_core::OsRng;
use scrypt::{
    password_hash::{PasswordHasher, SaltString},
    Scrypt,
};
use service::{
    mutation::{CreateUserData, Mutation},
    query::Query,
};
use strum::IntoStaticStr;
use thiserror::Error;

use super::generate_jwt_token;
use crate::{
    error::wrap_error, extractors::jsonv::JsonV, state::ServerState, utils::dto_entity::ToEntity,
};

impl ToEntity<service::RegistrationKind> for RegistrationKind {
    fn to_entity(&self) -> service::RegistrationKind {
        match self {
            Self::Email => service::RegistrationKind::Email,
            Self::Discord => service::RegistrationKind::Discord,
            Self::Google => service::RegistrationKind::Google,
        }
    }
}

#[derive(Debug, Error, IntoStaticStr)]
pub enum RegisterError {
    #[error("account with the same email already exists")]
    AccountWithSameEmailAlreadyExists,

    #[error("no password provided")]
    NoPassword,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::NoPassword => StatusCode::BAD_REQUEST,
            Self::AccountWithSameEmailAlreadyExists | RegisterError::Other(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        wrap_error(code, self)
    }
}

impl Into<RegisterClientError> for RegisterError {
    fn into(self) -> RegisterClientError {
        match self {
            Self::AccountWithSameEmailAlreadyExists => {
                RegisterClientError::AccountWithSameEmailAlreadyExists
            }
            Self::NoPassword => RegisterClientError::NoPassword,
            Self::Other(_) => RegisterClientError::Other,
        }
    }
}

pub async fn register(
    state: Arc<ServerState>,
    request: Valid<RegisterRequest>,
) -> Result<RegisterResponse, RegisterError> {
    let RegisterRequest {
        kind,
        email: Email(email),
        password,
        name: Name(name),
        avatar,
    } = request.into_inner();

    let password = password.map(|val| val.0);

    if Query::find_user_by_email(&state.db, &email)
        .await
        .context("failed to find user by email")?
        .is_some()
    {
        return Err(RegisterError::AccountWithSameEmailAlreadyExists);
    }

    let mut user = match kind {
        RegistrationKind::Email => {
            let Some(password) = password else {
                return Err(RegisterError::NoPassword)?;
            };

            let salt = SaltString::generate(&mut OsRng);

            let password_hash = Scrypt
                .hash_password(password.as_bytes(), &salt)
                .context("failed to hash password")?
                .to_string();

            Mutation::create_user(
                &state.db,
                CreateUserData {
                    kind: kind.to_entity(),
                    email,
                    password: Some(password_hash),
                    name,
                    avatar,
                },
            )
            .await
            .context("failed to create user")?
        }
        RegistrationKind::Discord | RegistrationKind::Google => Mutation::create_user(
            &state.db,
            CreateUserData {
                kind: kind.to_entity(),
                email,
                password: None,
                name,
                avatar,
            },
        )
        .await
        .context("failed to create user")?,
    };

    let token = generate_jwt_token(&state, user.id.take().unwrap().into())?;
    Ok(RegisterResponse { token })
}

pub async fn register_route(
    State(state): State<Arc<ServerState>>,
    JsonV(request): JsonV<RegisterRequest>,
) -> Result<Json<RegisterResponse>, RegisterError> {
    register(state, request).await.map(|val| Json(val))
}
