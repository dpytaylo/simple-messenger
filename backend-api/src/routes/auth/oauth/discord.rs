use rpc::{dto, procedure};

pub mod authorized;

procedure! {
    name: "oauth2_discord",
    request: OAuth2DiscordRequest,
    response: OAuth2DiscordResponse,
    error: OAuth2DiscordError,
}

#[dto]
pub struct OAuth2DiscordRequest {}

#[dto]
pub struct OAuth2DiscordResponse {
    pub uri: String,
}

#[dto]
pub enum OAuth2DiscordError {
    Other,
}
