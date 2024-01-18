use api_error_derive::ApiError;
use axum::{extract::State, Json};
use axum_garde::{IntoInner, WithValidation};
use common::entity::user::{Email, Name, Password};
use garde::Validate;
use rand_chacha::rand_core::OsRng;
use redis::RedisError;
use scrypt::{
    password_hash::{PasswordHasher, SaltString},
    Scrypt,
};
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use service::{
    mutation::{CreateUserData, Mutation},
    query::Query,
};
use thiserror::Error;
use tower_cookies::Cookies;

use crate::state::ServerState;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterPayload {
    #[garde(dive)]
    pub email: Email,

    #[garde(dive)]
    pub password: Password,

    #[garde(dive)]
    pub name: Name,
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
    RegisterPayload {
        email: Email(email),
        password: Password(password),
        name: Name(name),
    }: RegisterPayload,
) -> Result<(), RegisterError> {
    if Query::find_user_by_email(&state.db, &email)
        .await?
        .is_some()
    {
        return Err(RegisterError::AccountWithSameEmailAlreadyExists);
    }

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Scrypt
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    let mut user = Mutation::create_user(
        &state.db,
        CreateUserData {
            email,
            password: password_hash,
            name,
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
    WithValidation(payload): WithValidation<Json<RegisterPayload>>,
) -> Result<(), RegisterError> {
    register(state, cookies, payload.into_inner()).await
}

pub async fn register_oauth(_state: ServerState, _cookies: Cookies, _email: Email, _name: Name) {}
