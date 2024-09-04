use std::fmt;

use anyhow::Context;
use garde::Validate;
use serde::{Deserialize, Deserializer, Serialize};

pub const MAX_PASSWORD_SIZE: usize = 100;

#[derive(Clone, PartialEq, Hash, Serialize, Deserialize)]
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
    pub fn new(value: String) -> anyhow::Result<Self> {
        Validator(&value).validate().context("Invalid password")?;
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
