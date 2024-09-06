use rpc::{dto, procedure};

use crate::entities::{avatar_uri::AvatarUri, username::Username};

procedure! {
    name: "register_oauth2",
    request: RegisterOAuth2Request,
    response: RegisterOAuth2Response,
    error: RegisterOAuth2Error,
}

#[dto]
pub struct RegisterOAuth2Request {
    pub name: Username,
    pub avatar: Option<AvatarUri>,
}

#[dto]
pub struct RegisterOAuth2Response {
    pub token: String,
}

#[dto]
pub enum RegisterOAuth2Error {
    Other,
}
