use std::sync::Arc;

use anyhow::Context;
use api::{
    entities::{email::Email, registration_kind::RegistrationKind},
    routes::auth::registration::register_oauth2::{
        RegisterOAuth2Error, RegisterOAuth2Request, RegisterOAuth2Response,
    },
};
use axum::extract::State;
use rpc::server::error::{IntoProcFailure, ProcedureError};
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
};

#[derive(Debug, Error)]
pub enum RegisterOAuth2SErr {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<RegisterOAuth2Error> for RegisterOAuth2SErr {
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
    request: RegisterOAuth2Request,
) -> Result<RegisterOAuth2Response, RegisterOAuth2SErr> {
    let kind: RegistrationKind = get_session_value(&session, REGISTRATION_KIND_KEY).await?;
    let email: Email = get_session_value(&session, REGISTRATION_EMAIL_KEY).await?;

    let avatar = request
        .avatar
        .or(get_session_value(&session, REGISTRATION_AVATAR_URI_KEY).await?);

    let user = db::user::create(
        &state.db,
        kind,
        &email,
        None,
        &request.name,
        avatar.as_ref(),
    )
    .await
    .context("failed to create oauth2 user")?;

    let token = generate_jwt_token(&state, user.id.to_string())?;
    Ok(RegisterOAuth2Response { token })
}
