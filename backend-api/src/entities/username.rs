use garde::Validate;
use serde::{Deserialize, Deserializer, Serialize};

pub const MAX_USERNAME_SIZE: usize = 20;

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct Username(#[serde(deserialize_with = "parse")] String);

#[derive(Validate)]
#[garde(transparent)]
struct Validator<'a>(#[garde(length(min = 1, max = MAX_USERNAME_SIZE))] &'a str);

impl Username {
    pub fn new(value: String) -> anyhow::Result<Self> {
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

impl From<Username> for String {
    fn from(username: Username) -> Self {
        username.0
    }
}
