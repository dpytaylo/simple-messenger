use std::sync::Arc;

use anyhow::Context;
use axum::extract::State;
use common::{
    entity::user::Name,
    routes::auth::registration::is_name_available::{
        IsNameAvailableError, IsNameAvailableRequest, IsNameAvailableResponse,
    },
};
use garde::Valid;
use rpc::server::error::{IntoProcFailure, ProcedureError};
use service::query::Query;
use thiserror::Error;
use tracing::instrument;

use crate::state::ServerState;

#[derive(Debug, Error)]
pub enum IsNameAvailableServerError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<IsNameAvailableError> for IsNameAvailableServerError {
    fn into_procedure_error(self) -> impl IntoProcFailure<IsNameAvailableError> {
        match self {
            Self::Other(_) => IsNameAvailableError::Other.into_proc_failure(),
        }
    }
}

#[instrument(skip(state), err)]
pub async fn is_name_available(
    State(state): State<Arc<ServerState>>,
    request: Valid<IsNameAvailableRequest>,
) -> Result<IsNameAvailableResponse, IsNameAvailableServerError> {
    let IsNameAvailableRequest { name: Name(name) } = request.into_inner();

    let is_available = Query::is_name_available(&state.db, &name)
        .await
        .context("failed to check if name is available")?;

    return Ok(IsNameAvailableResponse { is_available });
}
