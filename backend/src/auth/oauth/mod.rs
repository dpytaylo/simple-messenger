use axum::Router;

use crate::state::ServerState;

pub mod google;

const OAUTH_STATE_EXPIRED: u64 = 600; // Seconds, 10 minutes

pub fn routes() -> Router<ServerState> {
    Router::new().nest("/google", google::routes())
}
