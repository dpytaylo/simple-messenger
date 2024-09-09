use garde::Validate;
use serde::{Deserialize, Deserializer, Serialize};

use crate::parser::ParseError;

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct Email(#[serde(deserialize_with = "parse")] String);

#[derive(Validate)]
#[garde(transparent)]
struct Validator<'a>(#[garde(email)] &'a str);

impl Email {
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

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}
