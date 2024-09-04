use anyhow::Context;
use garde::Validate;
use serde::{Deserialize, Deserializer, Serialize};

pub const MAX_AVATAR_URI_SIZE: usize = 256;

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct AvatarUri(#[serde(deserialize_with = "parse")] String);

#[derive(Validate)]
#[garde(transparent)]
struct Validator<'a>(#[garde(length(min = 1, max = MAX_AVATAR_URI_SIZE))] &'a str);

impl AvatarUri {
    pub fn new(value: String) -> anyhow::Result<Self> {
        Validator(&value).validate().context("Invalid avatar uri")?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
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
