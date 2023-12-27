use strum_macros::Display;

pub mod mutation;
pub mod query;

#[derive(Debug, Display)]
pub enum RegistrationType {
    Email,
    Google,
}
