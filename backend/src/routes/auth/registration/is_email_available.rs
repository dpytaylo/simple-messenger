use std::{ops::Not, sync::Arc};

use anyhow::Context;
use api::routes::auth::registration::is_email_available::{
    IsEmailAvailableError, IsEmailAvailableRequest, IsEmailAvailableResponse,
};
use axum::extract::State;
use rpc::server::error::{IntoProcFailure, ProcedureError};
use thiserror::Error;
use tracing::instrument;

use crate::state::ServerState;

#[derive(Debug, Error)]
pub enum IsEmailAvailableSErr {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<IsEmailAvailableError> for IsEmailAvailableSErr {
    fn into_procedure_error(self) -> impl IntoProcFailure<IsEmailAvailableError> {
        match self {
            Self::Other(_) => IsEmailAvailableError::Other.into_proc_failure(),
        }
    }
}

#[instrument(skip(state), err)]
pub async fn is_email_available(
    State(state): State<Arc<ServerState>>,
    request: IsEmailAvailableRequest,
) -> Result<IsEmailAvailableResponse, IsEmailAvailableSErr> {
    let is_available = db::user::find_by_email(&state.db, &request.email)
        .await
        .context("failed to find user by email")?
        .is_some()
        .not();

    return Ok(IsEmailAvailableResponse { is_available });
}
