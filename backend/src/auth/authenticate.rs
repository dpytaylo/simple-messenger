use axum::{extract::State, Json};
use axum_garde::WithValidation;
use common::{
    entity::user::{Email, Password},
    error::auth::AuthenticateError,
};
use garde::Validate;
use scrypt::{
    password_hash::{PasswordHash, PasswordVerifier},
    Scrypt,
};
use serde::{Deserialize, Serialize};
use service::query::Query;
use tower_cookies::Cookies;

use crate::state::ServerState;

#[derive(Deserialize, Serialize, Validate)]
pub struct AuthorizatePayload {
    #[garde(dive)]
    pub email: Email,

    #[garde(dive)]
    pub password: Password,
}

pub async fn authenticate(
    mut state: ServerState,
    cookies: Cookies,
    payload: AuthorizatePayload,
) -> Result<(), AuthenticateError> {
    let AuthorizatePayload {
        email: Email(email),
        password: Password(password),
    } = payload;

    let Some(user) = Query::find_user_by_email(&state.db, &email).await? else {
        return Err(AuthenticateError::AccountNotExists);
    };

    let Some(db_password) = user.password else {
        return Err(AuthenticateError::NotEmailRegistrationType);
    };

    let parsed_hash = PasswordHash::new(&db_password)?;

    if Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(AuthenticateError::InvalidPassword);
    }

    super::set_session_token(&mut state.random, &user.id, &state.redis, cookies).await?;
    Ok(())
}

pub async fn authenticate_route(
    State(state): State<ServerState>,
    cookies: Cookies,
    WithValidation(payload): WithValidation<Json<AuthorizatePayload>>,
) -> Result<(), AuthenticateError> {
    authenticate(state, cookies, payload.into_inner()).await
}
