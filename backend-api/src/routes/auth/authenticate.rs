use rpc::{dto, procedure, request};

use crate::entities::user::{Email, Password};

procedure! {
    name: "authenticate",
    request: AuthenticateRequest,
    response: AuthenticateResponse,
    error: AuthenticateError,
}

#[request]
pub struct AuthenticateRequest {
    #[garde(dive)]
    pub email: Email,

    #[garde(dive)]
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
