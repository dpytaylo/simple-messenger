use axum::{
    response::{IntoResponse, Response},
    Json,
};
use common::{
    entity::user::{Email, Password},
    routes::auth::{
        register::RegistrationKind,
        register_data::{RegisterDataClientError, RegisterDataRequest, RegisterDataResponse},
    },
};
use garde::Valid;
use http::StatusCode;
use strum::IntoStaticStr;
use thiserror::Error;
use tower_sessions::Session;

use crate::{
    error::wrap_error,
    extractors::jsonv::JsonV,
    session::{
        insert_session_key, REGISTRATION_EMAIL_KEY, REGISTRATION_KIND_KEY,
        REGISTRATION_PASSWORD_KEY,
    },
};

#[derive(Debug, Error, IntoStaticStr)]
pub enum RegisterDataError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for RegisterDataError {
    fn into_response(self) -> Response {
        wrap_error(StatusCode::INTERNAL_SERVER_ERROR, self)
    }
}

impl Into<RegisterDataClientError> for RegisterDataError {
    fn into(self) -> RegisterDataClientError {
        match self {
            Self::Other(_) => RegisterDataClientError::Other,
        }
    }
}

pub async fn register_data(
    session: Session,
    request: Valid<RegisterDataRequest>,
) -> Result<RegisterDataResponse, RegisterDataError> {
    let RegisterDataRequest {
        email: Email(email),
        password: Password(password),
    } = request.into_inner();

    insert_session_key(&session, REGISTRATION_KIND_KEY, RegistrationKind::Email).await?;
    insert_session_key(&session, REGISTRATION_EMAIL_KEY, email).await?;
    insert_session_key(&session, REGISTRATION_PASSWORD_KEY, password).await?;

    Ok(RegisterDataResponse {})
}

pub async fn register_data_route(
    session: Session,
    JsonV(request): JsonV<RegisterDataRequest>,
) -> Result<Json<RegisterDataResponse>, RegisterDataError> {
    register_data(session, request).await.map(|val| Json(val))
}
