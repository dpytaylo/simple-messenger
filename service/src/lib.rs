use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

pub mod mutation;
pub mod query;

#[derive(Debug, Display, EnumString, Serialize, Deserialize)]
pub enum RegistrationType {
    Email,

    Discord,
    Google,
}
