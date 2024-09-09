use rpc::{dto, procedure};

procedure! {
    name: "oauth2_google_authorized",
    request: OAuth2GoogleAuthorizedRequest,
    response: OAuth2GoogleAuthorizedResponse,
    error: OAuth2GoogleAuthorizedError,
}

#[dto]
pub struct OAuth2GoogleAuthorizedRequest {
    pub code: String,
    pub state: String,
}

#[dto]
pub enum OAuth2GoogleAuthorizedResponse {
    /// JWT token
    Authorized(String),

    /// Registration token
    RequiresRegistration(String),
}

#[dto]
pub enum OAuth2GoogleAuthorizedError {
    Other,
}
