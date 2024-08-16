use anyhow::Context;
use axum::{routing::post, Router};
use chrono::{Duration, Utc};
use common::routes::auth::authenticate::Authenticate;
use jsonwebtoken::{encode, Header};
use rpc::server::RouterExt;

use crate::{
    authorization::Claims,
    state::{ServerState, ServerStateWrapper},
};

pub mod authenticate;
pub mod oauth;
pub mod register;
pub mod register_data;

pub fn routes() -> Router<ServerStateWrapper> {
    Router::new()
        .nest("/oauth", oauth::routes())
        .route("/register", post(register::register_route))
}

pub fn generate_jwt_token(state: &ServerState, id: String) -> anyhow::Result<String> {
    let claims = Claims {
        sub: id,
        exp: (Utc::now() + Duration::hours(2)).timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &state.keys.encoding)
        .context("failed to encode JWT token")?;

    Ok(token)
}
