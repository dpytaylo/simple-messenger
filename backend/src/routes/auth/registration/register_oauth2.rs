use std::sync::Arc;

use anyhow::Context;
use api::{
    entities::{email::Email, registration_kind::RegistrationKind, username::Username},
    routes::auth::registration::register_oauth2::{
        RegisterOAuth2Error, RegisterOAuth2Request, RegisterOAuth2Response,
    },
};
use axum::extract::State;
use rpc::server::error::{IntoProcFailure, ProcedureError};
use thiserror::Error;
use tower_sessions::Session;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    routes::auth::generate_jwt_token,
    session::{
        get_session_value, REGISTRATION_AVATAR_URI_KEY, REGISTRATION_EMAIL_KEY,
        REGISTRATION_KIND_KEY,
    },
    state::ServerState,
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
    request: RegisterOAuth2Request,
) -> Result<RegisterOAuth2Response, RegisterOAuth2SError> {
    let kind: RegistrationKind = get_session_value(&session, REGISTRATION_KIND_KEY).await?;
    let email: Email = get_session_value(&session, REGISTRATION_EMAIL_KEY).await?;

    let avatar = request
        .avatar
        .or(get_session_value(&session, REGISTRATION_AVATAR_URI_KEY).await?);

    // Searches for an available username
    let username = loop {
        let full_uuid = Uuid::new_v4().to_string();

        // Takes the first 13 characters of the UUID
        // For example, if the UUID is 'cb6a7ea5-45fa-44d9-9c9d-1d7190076e63'
        // The part_uuid will be 'cb6a7ea5-45fa'
        let left_part_uuid = &full_uuid[..13];

        let username = Username::new(format!("user-{}", left_part_uuid))?; // 'user-cb6a7ea5-45fa'

        if db::user::find_by_name(&state.db, &username)
            .await
            .context("failed to check if name is available")?
            .is_none()
        {
            break username;
        }
    };

    let user = db::user::create(&state.db, kind, &email, None, &username, avatar.as_ref())
        .await
        .context("failed to create oauth2 user")?;

    let token = generate_jwt_token(&state, user.id.to_string())?;
    Ok(RegisterOAuth2Response { token })
}
