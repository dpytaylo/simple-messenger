use garde::Validate;
use serde::{Deserialize, Deserializer, Serialize};

use crate::parser::ParseError;

pub const AVATAR_SIZE: usize = 256;
pub const MAX_AVATAR_URI_SIZE: usize = 2048;

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct AvatarUri(#[serde(deserialize_with = "parse")] String);

#[derive(Validate)]
#[garde(transparent)]
struct Validator<'a>(#[garde(length(min = 1, max = MAX_AVATAR_URI_SIZE))] &'a str);

impl AvatarUri {
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

impl From<AvatarUri> for String {
    fn from(avatar_uri: AvatarUri) -> Self {
        avatar_uri.0
    }
}
