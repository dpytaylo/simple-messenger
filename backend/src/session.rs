use std::str::FromStr;

use api_error_derive::ApiError;
use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Request, State},
    middleware::Next,
    response::Response,
};
use http::request::Parts;
use redis::AsyncCommands;
use thiserror::Error;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use super::state::ServerState;
use crate::cookies::SESSION_TOKEN;

#[derive(Clone)]
pub struct SessionContext {
    pub user_id: Uuid,
}

#[derive(ApiError, Clone, Debug, Error)]
pub enum SessionContextError {
    #[error("request should have the session token cookie")]
    AuthFailNoSessionToken,

    #[error("invalid/expired session token")]
    AuthFailInvalidSessionToken,

    #[error("uuid error ({0})")]
    UuidError(#[from] uuid::Error),

    #[error("redis error")]
    RedisError,
}

pub async fn mw_session_context_resolver(
    State(state): State<ServerState>,
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> Response {
    // Cache the session context
    request
        .extensions_mut()
        .insert(resolve_session(state, cookies).await);

    next.run(request).await
}

async fn resolve_session(
    state: ServerState,
    cookies: Cookies,
) -> Result<SessionContext, SessionContextError> {
    let Some(session_cookie) = cookies.get(SESSION_TOKEN) else {
        return Err(SessionContextError::AuthFailInvalidSessionToken);
    };

    let Ok(mut connection) = state.redis.get_async_connection().await else {
        return Err(SessionContextError::RedisError);
    };

    let user_id: String = match connection.get(session_cookie.value()).await {
        Ok(val) => val,
        Err(_) => {
            // TODO
            cookies.remove(Cookie::from(SESSION_TOKEN));
            return Err(SessionContextError::RedisError);
        }
    };

    Ok(SessionContext {
        user_id: Uuid::from_str(&user_id)?,
    })
}

#[async_trait]
impl FromRequestParts<ServerState> for SessionContext {
    type Rejection = SessionContextError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &ServerState,
    ) -> Result<Self, SessionContextError> {
        parts
            .extensions
            .get::<SessionContext>()
            .ok_or(SessionContextError::AuthFailNoSessionToken)
            .cloned()
    }
}
