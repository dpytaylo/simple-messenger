use std::mem;

use axum::{routing::post, Router};
use rand_chacha::{rand_core::RngCore, ChaCha8Rng};
use redis::RedisResult;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    cookies::{self, SESSION_TOKEN},
    redis::session,
    state::ServerState,
};

pub mod authenticate;
pub mod oauth;
pub mod register;

const SESSION_TOKEN_EXPIRED: u64 = 10800; // In seconds, 3 hours

pub fn routes() -> Router<ServerState> {
    Router::new()
        .nest("/oauth", oauth::routes())
        .route("/authenticate", post(authenticate::authenticate_route))
        .route("/register", post(register::register_route))
}

async fn set_session_token(
    random: &mut ChaCha8Rng,
    client_id: &Uuid,
    redis_client: &redis::Client,
    cookies: Cookies,
) -> RedisResult<()> {
    let mut pool = [0u8; mem::size_of::<u128>()];
    random.fill_bytes(&mut pool);

    // Endian doesn't matter here
    let token = u128::from_le_bytes(pool);
    session::insert_token(
        redis_client,
        token.to_string(),
        client_id.to_string(),
        SESSION_TOKEN_EXPIRED,
    )
    .await?;

    cookies.add(cookies::create_secure_cookie(
        SESSION_TOKEN,
        token.to_string(),
    ));

    Ok(())
}
