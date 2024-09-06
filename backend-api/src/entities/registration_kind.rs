use std::{fmt, str::FromStr};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::parser::ParseError;

#[derive(Clone, Debug, PartialEq, Hash, Deserialize, Serialize)]
pub enum RegistrationKind {
    Email,
    Discord,
    Google,
}

impl fmt::Display for RegistrationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RegistrationKind::Email => "email",
                RegistrationKind::Discord => "discord",
                RegistrationKind::Google => "google",
            }
        )
    }
}

impl FromStr for RegistrationKind {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "email" => Ok(RegistrationKind::Email),
            "discord" => Ok(RegistrationKind::Discord),
            "google" => Ok(RegistrationKind::Google),
            _ => Err(ParseError::Other(anyhow!(
                "invalid registration kind: {}",
                s
            ))),
        }
    }
}
