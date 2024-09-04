use rpc::{dto, procedure, request};

use crate::entities::name::Name;

procedure! {
    name: "is_name_available",
    request: IsNameAvailableRequest,
    response: IsNameAvailableResponse,
    error: IsNameAvailableError,
}

#[request]
pub struct IsNameAvailableRequest {
    #[garde(dive)]
    pub name: Name,
}

#[dto]
pub struct IsNameAvailableResponse {
    pub is_available: bool,
}

#[dto]
pub enum IsNameAvailableError {
    Other,
}
