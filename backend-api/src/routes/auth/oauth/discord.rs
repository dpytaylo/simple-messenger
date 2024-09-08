use rpc::{dto, procedure};

use crate::entities::sha256::Sha256;

procedure! {
    name: "oauth2_discord",
    request: OAuth2DiscordRequest,
    response: OAuth2DiscordResponse,
    error: OAuth2DiscordError,
}

#[dto]
pub struct OAuth2DiscordRequest {
    pub pkce_code_challenge: Sha256,
}

#[dto]
pub struct OAuth2DiscordResponse {
    pub uri: String,
}

#[dto]
pub enum OAuth2DiscordError {
    Other,
}
