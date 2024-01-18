use strum_macros::{Display, EnumString};

pub mod mutation;
pub mod query;

#[derive(Debug, Display, EnumString)]
pub enum RegistrationType {
    Email,

    Discord,
    Google,
}
