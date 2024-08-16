use axum::{middleware, routing::get, Router};
use common::routes::auth::authenticate::Authenticate;
use rpc::server::RouterExt;

use self::auth::authenticate;
use crate::{authorization::mw_authorization, state::ServerStateWrapper};

pub mod auth;
pub mod health;

pub fn routes(state: ServerStateWrapper) -> Router<ServerStateWrapper> {
    Router::new()
        // TODO add private routes here
        .layer(middleware::map_request_with_state(state, mw_authorization))
        .nest("/auth", auth::routes())
        .route("/health", get(health::health))
        .nest("/rpc", rpc_routes())
}

fn rpc_routes() -> Router<ServerStateWrapper> {
    Router::new().route_rpc(Authenticate, authenticate::authenticate)
}
