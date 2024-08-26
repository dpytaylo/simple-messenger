use rpc::{dto, procedure, request};

use crate::entity::user::Name;

procedure! {
    name: "register_oauth2",
    request: RegisterOAuth2Request,
    response: RegisterOAuth2Response,
    error: RegisterOAuth2Error,
}

#[request]
pub struct RegisterOAuth2Request {
    #[garde(dive)]
    pub name: Name,

    #[garde(ascii, length(max = 512))]
    pub avatar: Option<String>,
}

#[dto]
pub struct RegisterOAuth2Response {
    pub token: String,
}

#[dto]
pub enum RegisterOAuth2Error {
    Other,
}
