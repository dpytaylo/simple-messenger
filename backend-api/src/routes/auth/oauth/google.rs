use rpc::{dto, procedure};

use crate::entities::sha256::Sha256;

procedure! {
    name: "oauth2_google",
    request: OAuth2GoogleRequest,
    response: OAuth2GoogleResponse,
    error: OAuth2GoogleError,
}

#[dto]
pub struct OAuth2GoogleRequest {
    pub pkce_code_challenge: Sha256,
}

#[dto]
pub struct OAuth2GoogleResponse {
    pub uri: String,
}

#[dto]
pub enum OAuth2GoogleError {
    Other,
}
