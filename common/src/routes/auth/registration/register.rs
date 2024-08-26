use rpc::{dto, procedure, request};

use crate::entity::user::{Email, Name, Password};

procedure! {
    name: "register",
    request: RegisterRequest,
    response: RegisterResponse,
    error: RegisterError,
}

#[request]
pub struct RegisterRequest {
    #[garde(dive)]
    pub email: Email,

    #[garde(dive)]
    pub password: Password,

    #[garde(dive)]
    pub name: Name,

    #[garde(ascii, length(max = 512))]
    pub avatar: Option<String>,
}

#[dto]
pub struct RegisterResponse {
    pub token: String,
}

#[dto]
pub enum RegisterError {
    AccountWithSameEmailAlreadyExists,
    NoPassword,
    Other,
}
