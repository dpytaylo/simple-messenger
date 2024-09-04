use std::{ops::Not, sync::Arc};

use anyhow::Context;
use axum::extract::State;
use backend_api::{
    entities::user::Email,
    routes::auth::registration::is_email_available::{
        IsEmailAvailableError, IsEmailAvailableRequest, IsEmailAvailableResponse,
    },
};
use garde::Valid;
use rpc::server::error::{IntoProcFailure, ProcedureError};
use backend_db::query::Query;
use thiserror::Error;
use tracing::instrument;

use crate::state::ServerState;

#[derive(Debug, Error)]
pub enum IsEmailAvailableServerError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<IsEmailAvailableError> for IsEmailAvailableServerError {
    fn into_procedure_error(self) -> impl IntoProcFailure<IsEmailAvailableError> {
        match self {
            Self::Other(_) => IsEmailAvailableError::Other.into_proc_failure(),
        }
    }
}

#[instrument(skip(state), err)]
pub async fn is_email_available(
    State(state): State<Arc<ServerState>>,
    request: Valid<IsEmailAvailableRequest>,
) -> Result<IsEmailAvailableResponse, IsEmailAvailableServerError> {
    let IsEmailAvailableRequest {
        email: Email(email),
    } = request.into_inner();

    let is_available = Query::find_user_by_email(&state.db, &email)
        .await
        .context("failed to find user by email")?
        .is_some()
        .not();

    return Ok(IsEmailAvailableResponse { is_available });
}
