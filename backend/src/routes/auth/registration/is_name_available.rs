use std::sync::Arc;

use anyhow::Context;
use api::routes::auth::registration::is_name_available::{
    IsUsernameAvailableError, IsUsernameAvailableRequest, IsUsernameAvailableResponse,
};
use axum::extract::State;
use rpc::server::error::{IntoProcFailure, ProcedureError};
use thiserror::Error;
use tracing::instrument;

use crate::state::ServerState;

#[derive(Debug, Error)]
pub enum IsUsernameAvailableServerError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<IsUsernameAvailableError> for IsUsernameAvailableServerError {
    fn into_procedure_error(self) -> impl IntoProcFailure<IsUsernameAvailableError> {
        match self {
            Self::Other(_) => IsUsernameAvailableError::Other.into_proc_failure(),
        }
    }
}

#[instrument(skip(state), err)]
pub async fn is_username_available(
    State(state): State<Arc<ServerState>>,
    request: IsUsernameAvailableRequest,
) -> Result<IsUsernameAvailableResponse, IsUsernameAvailableServerError> {
    let is_available = db::user::find_by_name(&state.db, &request.name)
        .await
        .context("failed to check if name is available")?
        .is_none();

    return Ok(IsUsernameAvailableResponse { is_available });
}
