use std::fmt;

use garde::Validate;
use serde::{Deserialize, Deserializer, Serialize};

use crate::parser::ParseError;

pub const MAX_PASSWORD_SIZE: usize = 100;

#[derive(Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct Password(#[serde(deserialize_with = "parse")] String);

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Password").finish()
    }
}

#[derive(Validate)]
#[garde(transparent)]
struct Validator<'a>(#[garde(length(min = 1, max = MAX_PASSWORD_SIZE))] &'a str);

impl Password {
    pub fn new(value: String) -> Result<Self, ParseError> {
        Validator(&value).validate()?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn into_raw(self) -> String {
        self.0
    }
}

fn parse<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;

    Validator(&value)
        .validate()
        .map_err(serde::de::Error::custom)?;

    Ok(value)
}

impl From<Password> for String {
    fn from(password: Password) -> Self {
        password.0
    }
}
