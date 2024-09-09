use rpc::{dto, procedure};

use crate::entities::username::Username;

procedure! {
    name: "is_username_available",
    request: IsUsernameAvailableRequest,
    response: IsUsernameAvailableResponse,
    error: IsUsernameAvailableError,
}

#[dto]
pub struct IsUsernameAvailableRequest {
    pub name: Username,
}

#[dto]
pub struct IsUsernameAvailableResponse {
    pub is_available: bool,
}

#[dto]
pub enum IsUsernameAvailableError {
    Other,
}
