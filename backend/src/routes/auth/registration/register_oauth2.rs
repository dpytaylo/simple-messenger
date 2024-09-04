use std::sync::Arc;

use anyhow::Context;
use axum::extract::State;
use backend_api::{
    entities::{
        registration_kind::RegistrationKind,
        user::{Email, Name},
    },
    routes::auth::registration::register_oauth2::{
        RegisterOAuth2Error, RegisterOAuth2Request, RegisterOAuth2Response,
    },
};
use garde::{Unvalidated, Valid};
use rpc::server::error::{IntoProcFailure, ProcedureError};
use backend_db::mutation::{CreateUserData, Mutation};
use thiserror::Error;
use tower_sessions::Session;
use tracing::instrument;

use crate::{
    routes::auth::generate_jwt_token,
    session::{
        get_session_value, REGISTRATION_AVATAR_URI_KEY, REGISTRATION_EMAIL_KEY,
        REGISTRATION_KIND_KEY,
    },
    state::ServerState,
    utils::dto_entity::ToEntity,
};

#[derive(Debug, Error)]
pub enum RegisterOAuth2SError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<RegisterOAuth2Error> for RegisterOAuth2SError {
    fn into_procedure_error(self) -> impl IntoProcFailure<RegisterOAuth2Error> {
        match self {
            Self::Other(_) => RegisterOAuth2Error::Other.into_proc_failure(),
        }
    }
}

#[instrument(skip(state), err)]
pub async fn register_oauth2(
    State(state): State<Arc<ServerState>>,
    session: Session,
    request: Valid<RegisterOAuth2Request>,
) -> Result<RegisterOAuth2Response, RegisterOAuth2SError> {
    let RegisterOAuth2Request {
        name: Name(name),
        avatar,
    } = request.into_inner();

    let kind: RegistrationKind = get_session_value(&session, REGISTRATION_KIND_KEY).await?;
    let email: String = get_session_value(&session, REGISTRATION_EMAIL_KEY).await?;

    let avatar = avatar.or(get_session_value(&session, REGISTRATION_AVATAR_URI_KEY).await?);

    let email = Unvalidated::new(Email(email))
        .validate()
        .context("failed to validate email")?;

    // TODO avatar validation

    let mut user = Mutation::create_user(
        &state.db,
        CreateUserData {
            kind: kind.to_entity(),
            email: email.into_inner().0,
            password: None,
            name,
            avatar,
        },
    )
    .await
    .context("failed to create oauth2 user")?;

    let token = generate_jwt_token(&state, user.id.take().unwrap().into())?;
    Ok(RegisterOAuth2Response { token })
}
