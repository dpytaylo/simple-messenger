use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::{
    extract::WithRejection,
    headers::{authorization::Bearer, Authorization},
    typed_header::TypedHeaderRejection,
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;
use thiserror::Error;

use crate::{error::wrap_error, state::ServerState};

#[derive(Clone)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[async_trait]
impl FromRequestParts<Arc<ServerState>> for Claims {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &Arc<ServerState>,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .ok_or(AuthError::NoAuth)
            .cloned()
    }
}

#[derive(Debug, Error, IntoStaticStr)]
pub enum AuthError {
    #[error("invalid token")]
    InvalidToken,

    #[error("no auth")]
    NoAuth,

    #[error(transparent)]
    MissedAuthorizationHeader(#[from] TypedHeaderRejection),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<axum::body::Body> {
        let code = match self {
            AuthError::InvalidToken
            | AuthError::NoAuth
            | AuthError::MissedAuthorizationHeader(_) => StatusCode::UNAUTHORIZED,
        };

        wrap_error(code, self)
    }
}

pub async fn mw_authorization(
    State(state): State<Arc<ServerState>>,
    WithRejection(TypedHeader(Authorization(bearer)), _): WithRejection<
        TypedHeader<Authorization<Bearer>>,
        AuthError,
    >,
    mut request: Request,
) -> Result<Request, AuthError> {
    let token_data: TokenData<Claims> =
        decode(bearer.token(), &state.keys.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

    request.extensions_mut().insert(token_data.claims);
    Ok(request)
}
