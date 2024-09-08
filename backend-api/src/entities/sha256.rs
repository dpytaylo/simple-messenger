use garde::Validate;
use serde::{Deserialize, Deserializer, Serialize};

use crate::parser::ParseError;

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct Sha256(#[serde(deserialize_with = "parse")] Vec<u8>);

#[derive(Validate)]
#[garde(transparent)]
struct Validator<'a>(#[garde(length(min = 256, max = 256))] &'a [u8]);

impl Sha256 {
    pub fn new(value: Vec<u8>) -> Result<Self, ParseError> {
        Validator(&value).validate()?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &[u8] {
        &self.0
    }

    pub fn into_raw(self) -> Vec<u8> {
        self.0
    }
}

fn parse<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Vec<u8> = Deserialize::deserialize(deserializer)?;

    Validator(&value)
        .validate()
        .map_err(serde::de::Error::custom)?;

    Ok(value)
}

impl From<Sha256> for Vec<u8> {
    fn from(email: Sha256) -> Self {
        email.0
    }
}
