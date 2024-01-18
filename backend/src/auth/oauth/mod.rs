use api_error_derive::ApiError;
use axum::Router;
use redis::RedisError;
use thiserror::Error;

use crate::state::ServerState;

pub mod discord;
pub mod google;

const OAUTH_STATE_EXPIRED: u64 = 600; // Seconds, 10 minutes

pub fn routes() -> Router<ServerState> {
    Router::new()
        .nest("/discord", discord::routes())
        .nest("/google", google::routes())
}

#[derive(ApiError, Debug, Error)]
pub enum OAuthError {
    #[error("redis error ({0})")]
    RedisError(#[from] RedisError),
}
