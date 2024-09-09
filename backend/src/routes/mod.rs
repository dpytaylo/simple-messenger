use api::routes::auth::{
    authenticate::Authenticate,
    oauth::{discord::Oauth2Discord, google::Oauth2Google},
    registration::{
        is_email_available::IsEmailAvailable, is_name_available::IsUsernameAvailable,
        register::Register, register_oauth2::RegisterOauth2,
    },
};
use axum::{middleware, routing::get, Router};
use rpc::server::RouterExt;

use self::auth::authenticate;
use self::auth::registration::register;
use self::auth::registration::{is_email_available, is_name_available, register_oauth2};
use crate::{authorization::mw_authorization, state::ServerStateWrapper};

pub mod auth;
pub mod health;

pub fn routes(state: ServerStateWrapper) -> Router<ServerStateWrapper> {
    Router::new()
        // TODO add private routes here
        .layer(middleware::map_request_with_state(state, mw_authorization))
        .route("/health", get(health::health))
        .nest("/rpc", rpc_routes())
}

fn rpc_routes() -> Router<ServerStateWrapper> {
    Router::new()
        .route_rpc(IsEmailAvailable, is_email_available::is_email_available)
        .route_rpc(
            IsUsernameAvailable,
            is_name_available::is_username_available,
        )
        .route_rpc(Register, register::register)
        .route_rpc(RegisterOauth2, register_oauth2::register_oauth2)
        .route_rpc(Authenticate, authenticate::authenticate)
        .route_rpc(Oauth2Discord, auth::oauth::discord::oauth2_discord)
        .route_rpc(Oauth2Google, auth::oauth::google::oauth2_google)
}
