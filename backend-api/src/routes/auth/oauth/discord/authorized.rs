use rpc::{dto, procedure};

procedure! {
    name: "oauth2_discord_authorized",
    request: OAuth2DiscordAuthorizedRequest,
    response: OAuth2DiscordAuthorizedResponse,
    error: OAuth2DiscordAuthorizedError,
}

#[dto]
pub struct OAuth2DiscordAuthorizedRequest {
    pub code: String,
    pub state: String,
}

#[dto]
pub enum OAuth2DiscordAuthorizedResponse {
    /// JWT token
    Authorized(String),

    /// Registration token
    RequiresRegistration(String),
}

#[dto]
pub enum OAuth2DiscordAuthorizedError {
    Other,
}
