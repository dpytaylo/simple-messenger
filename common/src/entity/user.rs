use garde::Validate;
use serde::{Deserialize, Serialize};

pub const MAX_USER_EMAIL_SIZE: usize = 320; // RFC 5321, RFC 5322
pub const MAX_USER_NAME_SIZE: usize = 20;

// Not for using in a database because passwords will hashed
pub const MAX_USER_PASSWORD_SIZE: usize = 100;

pub const USER_AVATAR_SIZE: usize = 256;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[garde(transparent)]
pub struct Email(#[garde(email)] pub String);

#[derive(Serialize, Deserialize, Validate)]
#[garde(transparent)]
pub struct Password(#[garde(length(min = 1, max = MAX_USER_PASSWORD_SIZE))] pub String);

#[derive(Debug, Serialize, Deserialize, Validate)]
#[garde(transparent)]
pub struct Name(#[garde(length(min = 1, max = MAX_USER_NAME_SIZE))] pub String);
