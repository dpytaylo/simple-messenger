use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header};

use crate::{authorization::Claims, state::ServerState};

pub mod authenticate;
pub mod oauth;
pub mod registration;

pub fn generate_jwt_token(state: &ServerState, id: String) -> anyhow::Result<String> {
    let claims = Claims {
        sub: id,
        exp: (Utc::now() + Duration::hours(2)).timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &state.keys.encoding)
        .context("failed to encode JWT token")?;

    Ok(token)
}
