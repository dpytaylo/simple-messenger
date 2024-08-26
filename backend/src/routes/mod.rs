use auth::registration::register_oauth2;
use axum::{middleware, routing::get, Router};
use common::routes::auth::{
    authenticate::Authenticate,
    registration::{
        is_email_available::IsEmailAvailable, is_name_available::IsNameAvailable,
        register::Register, register_oauth2::RegisterOauth2,
    },
};
use rpc::server::RouterExt;

use self::auth::authenticate;
use self::auth::registration::register;
use self::auth::registration::{is_email_available, is_name_available};
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
    Router::new()
        .route_rpc(IsEmailAvailable, is_email_available::is_email_available)
        .route_rpc(IsNameAvailable, is_name_available::is_name_available)
        .route_rpc(Register, register::register)
        .route_rpc(RegisterOauth2, register_oauth2::register_oauth2)
        .route_rpc(Authenticate, authenticate::authenticate)
}
