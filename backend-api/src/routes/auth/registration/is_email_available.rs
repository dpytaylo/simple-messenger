use rpc::{dto, procedure, request};

use crate::entities::email::Email;

procedure! {
    name: "is_email_available",
    request: IsEmailAvailableRequest,
    response: IsEmailAvailableResponse,
    error: IsEmailAvailableError,
}

#[request]
pub struct IsEmailAvailableRequest {
    #[garde(dive)]
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
