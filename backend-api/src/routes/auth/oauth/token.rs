use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct TokenRequest;

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub enum TokenClientError {
    Other,
}
