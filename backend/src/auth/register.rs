use axum::{extract::State, Json};
use axum_garde::{IntoInner, WithValidation};
use common::{
    entity::user::{Email, Name, Password},
    error::auth::RegisterError,
};
use garde::Validate;
use rand_chacha::rand_core::OsRng;
use scrypt::{
    password_hash::{PasswordHasher, SaltString},
    Scrypt,
};
use serde::{Deserialize, Serialize};
use service::{
    mutation::{CreateUserData, Mutation},
    query::Query,
    RegistrationType,
};
use tower_cookies::Cookies;

use crate::state::ServerState;

#[derive(Serialize, Deserialize, Validate)]
pub struct RegisterPayload {
    #[garde(skip)]
    pub kind: RegistrationType,

    #[garde(dive)]
    pub email: Email,

    #[garde(dive)]
    pub password: Option<Password>,

    #[garde(dive)]
    pub name: Name,
}

pub async fn register(
    mut state: ServerState,
    cookies: Cookies,
    payload: RegisterPayload,
) -> Result<(), RegisterError> {
    let RegisterPayload {
        kind,
        email: Email(email),
        password,
        name: Name(name),
    } = payload;

    let password = password.map(|val| val.0);

    if Query::find_user_by_email(&state.db, &email)
        .await?
        .is_some()
    {
        return Err(RegisterError::AccountWithSameEmailAlreadyExists);
    }

    let mut user = match kind {
        kind @ RegistrationType::Email => {
            let Some(password) = password else {
                return Err(RegisterError::NoPassword)?;
            };

            let salt = SaltString::generate(&mut OsRng);

            let password_hash = Scrypt
                .hash_password(password.as_bytes(), &salt)?
                .to_string();

            Mutation::create_user(
                &state.db,
                CreateUserData {
                    kind,
                    email,
                    password: Some(password_hash),
                    name,
                },
            )
            .await?
        }
        kind @ (RegistrationType::Discord | RegistrationType::Google) => {
            Mutation::create_user(
                &state.db,
                CreateUserData {
                    kind,
                    email,
                    password: None,
                    name,
                },
            )
            .await?
        }
    };

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
