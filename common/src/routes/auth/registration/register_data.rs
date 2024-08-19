use rpc::{dto, procedure, request};

use crate::entity::user::{Email, Password};

procedure! {
    name: "register_data",
    request: RegisterDataRequest,
    response: RegisterDataResponse,
    error: RegisterDataError,
}

#[request]
pub struct RegisterDataRequest {
    #[garde(dive)]
    pub email: Email,

    #[garde(dive)]
    pub password: Password,
}

#[dto]
pub struct RegisterDataResponse {}

#[dto]
pub enum RegisterDataError {
    Other,
}
