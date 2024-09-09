use rpc::{dto, procedure};

use crate::entities::{email::Email, password::Password};

procedure! {
    name: "authenticate",
    request: AuthenticateRequest,
    response: AuthenticateResponse,
    error: AuthenticateError,
}

#[dto]
pub struct AuthenticateRequest {
    pub email: Email,
    pub password: Password,
}

#[dto]
pub struct AuthenticateResponse {
    pub token: String,
}

#[dto]
pub enum AuthenticateError {
    InvalidCredentials,
    Other,
}
