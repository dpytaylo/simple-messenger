use common::{
    entity::user::{Email, Password},
    routes::auth::registration::{
        register::RegistrationKind,
        register_data::{RegisterDataError, RegisterDataRequest, RegisterDataResponse},
    },
};
use garde::Valid;
use rpc::server::error::{IntoProcFailure, ProcedureError};
use thiserror::Error;
use tower_sessions::Session;
use tracing::instrument;

use crate::session::{
    insert_session_key, REGISTRATION_EMAIL_KEY, REGISTRATION_KIND_KEY, REGISTRATION_PASSWORD_KEY,
};

#[derive(Debug, Error)]
pub enum RegisterDataServerError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<RegisterDataError> for RegisterDataServerError {
    fn into_procedure_error(self) -> impl IntoProcFailure<RegisterDataError> {
        match self {
            RegisterDataServerError::Other(_) => RegisterDataError::Other,
        }
    }
}

#[instrument(skip(session), err)]
pub async fn register_data(
    session: Session,
    request: Valid<RegisterDataRequest>,
) -> Result<RegisterDataResponse, RegisterDataServerError> {
    let RegisterDataRequest {
        email: Email(email),
        password: Password(password),
    } = request.into_inner();

    insert_session_key(&session, REGISTRATION_KIND_KEY, RegistrationKind::Email).await?;
    insert_session_key(&session, REGISTRATION_EMAIL_KEY, email).await?;
    insert_session_key(&session, REGISTRATION_PASSWORD_KEY, password).await?;

    Ok(RegisterDataResponse {})
}
