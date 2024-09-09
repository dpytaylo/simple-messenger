use rpc::{dto, procedure};

pub mod authorized;

procedure! {
    name: "oauth2_google",
    request: OAuth2GoogleRequest,
    response: OAuth2GoogleResponse,
    error: OAuth2GoogleError,
}

#[dto]
pub struct OAuth2GoogleRequest {}

#[dto]
pub struct OAuth2GoogleResponse {
    pub uri: String,
}

#[dto]
pub enum OAuth2GoogleError {
    Other,
}
