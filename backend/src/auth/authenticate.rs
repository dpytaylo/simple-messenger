use api_error_derive::ApiError;
use axum::{extract::State, Json};
use redis::RedisError;
use scrypt::{
    password_hash::{PasswordHash, PasswordVerifier},
    Scrypt,
};
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use service::query::Query;
use thiserror::Error;
use tower_cookies::Cookies;

use crate::state::ServerState;

#[derive(Deserialize, Serialize)]
pub struct AuthorizatePayload {
    pub email: String,
    pub password: String,
}

#[derive(ApiError, Debug, Error)]
pub enum AuthorizateError {
    #[error("the account does not exist")]
    #[status_code(BAD_REQUEST)]
    #[custom("InvalidEmailOrPassword")]
    AccountNotExists,

    #[error("invalid password")]
    #[status_code(BAD_REQUEST)]
    #[custom("InvalidEmailOrPassword")]
    InvalidPassword,

    #[error("db error ({0})")]
    Db(#[from] DbErr),

    #[error("password error ({0})")]
    PasswordHashError(#[from] scrypt::password_hash::Error),

    #[error("redis error ({0})")]
    RedisError(#[from] RedisError),
}

pub async fn authenticate(
    mut state: ServerState,
    cookies: Cookies,
    payload: AuthorizatePayload,
) -> Result<(), AuthorizateError> {
    let Some(user) = Query::find_user_by_email(&state.db, &payload.email).await? else {
        return Err(AuthorizateError::AccountNotExists);
    };

    let parsed_hash = PasswordHash::new(&user.password)?;

    if Scrypt
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(AuthorizateError::InvalidPassword);
    }

    super::set_session_token(&mut state.random, &user.id, &state.redis, cookies).await?;
    Ok(())
}

pub async fn authenticate_route(
    State(state): State<ServerState>,
    cookies: Cookies,
    Json(payload): Json<AuthorizatePayload>,
) -> Result<(), AuthorizateError> {
    authenticate(state, cookies, payload).await
}
