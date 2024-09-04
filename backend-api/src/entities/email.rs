use anyhow::Context;
use garde::Validate;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Email(#[serde(deserialize_with = "parse")] String);

#[derive(Validate)]
#[garde(transparent)]
struct Validator<'a>(#[garde(email)] &'a str);

impl Email {
    pub fn new(value: String) -> anyhow::Result<Self> {
        Validator(&value).validate().context("Invalid email")?;
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
