use rpc::{dto, procedure, request};
use serde::{Deserialize, Serialize};

use crate::entity::user::{Email, Name, Password};

procedure! {
    name: "register",
    request: RegisterRequest,
    response: RegisterResponse,
    error: RegisterError,
}

#[derive(Clone, Debug, PartialEq, Hash, Deserialize, Serialize)]
pub enum RegistrationKind {
    Email,
    Discord,
    Google,
}

#[request]
pub struct RegisterRequest {
    #[garde(skip)]
    pub kind: RegistrationKind,

    #[garde(dive)]
    pub email: Email,

    #[garde(custom(is_email_kind(&self.kind)), dive)]
    pub password: Option<Password>,

    #[garde(dive)]
    pub name: Name,

    #[garde(ascii, length(max = 512))]
    pub avatar: Option<String>,
}

fn is_email_kind(
    kind: &RegistrationKind,
) -> impl FnOnce(&Option<Password>, &()) -> garde::Result + '_ {
    move |password, _| {
        if password.is_some() && matches!(kind, RegistrationKind::Email) || password.is_none() {
            Ok(())
        } else {
            Err(garde::Error::new(
                "password is required for email registration",
            ))
        }
    }
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
