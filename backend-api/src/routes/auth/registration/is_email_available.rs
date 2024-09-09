use rpc::{dto, procedure};

use crate::entities::email::Email;

procedure! {
    name: "is_email_available",
    request: IsEmailAvailableRequest,
    response: IsEmailAvailableResponse,
    error: IsEmailAvailableError,
}

#[dto]
pub struct IsEmailAvailableRequest {
    pub email: Email,
}

#[dto]
pub struct IsEmailAvailableResponse {
    pub is_available: bool,
}

#[dto]
pub enum IsEmailAvailableError {
    Other,
}
