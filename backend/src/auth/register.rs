use api_error_derive::ApiError;
use axum::extract::State;
use common::{MAX_USER_NAME_SIZE, MAX_USER_PASSWORD_SIZE};
use rand_chacha::rand_core::OsRng;
use redis::RedisError;
use scrypt::{
    password_hash::{PasswordHasher, SaltString},
    Scrypt,
};
use sea_orm::DbErr;
use serde::Deserialize;
use service::{
    mutation::{CreateUserData, Mutation},
    query::Query,
};
use thiserror::Error;
use tower_cookies::Cookies;
use validator::Validate;

use crate::{state::ServerState, validator::ValidatedJson};

#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, max = "MAX_USER_PASSWORD_SIZE"))]
    pub password: String,

    #[validate(length(min = 1, max = "MAX_USER_NAME_SIZE"))]
    pub name: String,
}

#[derive(ApiError, Debug, Error)]
pub enum RegisterError {
    #[error("account with the same email already exists")]
    #[status_code(BAD_REQUEST)]
    AccountWithSameEmailAlreadyExists,

    #[error("db error ({0})")]
    Db(#[from] DbErr),

    #[error("password error ({0})")]
    PasswordHash(#[from] scrypt::password_hash::Error),

    #[error("redis error ({0})")]
    Redis(#[from] RedisError),
}

pub async fn register(
    mut state: ServerState,
    cookies: Cookies,
    payload: RegisterPayload,
) -> Result<(), RegisterError> {
    if Query::find_user_by_email(&state.db, &payload.email)
        .await?
        .is_some()
    {
        return Err(RegisterError::AccountWithSameEmailAlreadyExists);
    }

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Scrypt
        .hash_password(payload.password.as_bytes(), &salt)?
        .to_string();

    let mut user = Mutation::create_user(
        &state.db,
        CreateUserData {
            email: payload.email,
            password: password_hash,
            name: payload.name,
        },
    )
    .await?;

    super::set_session_token(
        &mut state.random,
        &user.id.take().unwrap(),
        &state.redis,
        cookies,
    )
    .await?;
    Ok(())
}

pub async fn register_route(
    State(state): State<ServerState>,
    cookies: Cookies,
    ValidatedJson(payload): ValidatedJson<RegisterPayload>,
) -> Result<(), RegisterError> {
    register(state, cookies, payload).await
}
