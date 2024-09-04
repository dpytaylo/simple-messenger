use anyhow::Context;
use garde::Validate;
use serde::{Deserialize, Deserializer, Serialize};

pub const MAX_USERNAME_SIZE: usize = 20;

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Username(#[serde(deserialize_with = "parse")] String);

#[derive(Validate)]
#[garde(transparent)]
struct Validator<'a>(#[garde(length(min = 1, max = MAX_USERNAME_SIZE))] &'a str);

impl Username {
    pub fn new(value: String) -> anyhow::Result<Self> {
        Validator(&value).validate().context("Invalid username")?;
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
