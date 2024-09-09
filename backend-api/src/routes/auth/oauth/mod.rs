use serde::{Deserialize, Serialize};

pub mod discord;
pub mod google;
pub mod token;

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub enum OAuthClientError {
    Other,
}
