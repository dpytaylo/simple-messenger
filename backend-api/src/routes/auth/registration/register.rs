use rpc::{dto, procedure};

use crate::entities::{
    avatar_uri::AvatarUri, email::Email, password::Password, username::Username,
};

procedure! {
    name: "register",
    request: RegisterRequest,
    response: RegisterResponse,
    error: RegisterError,
}

#[dto]
pub struct RegisterRequest {
    pub email: Email,
    pub password: Password,
    pub name: Username,
    pub avatar: Option<AvatarUri>,
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
