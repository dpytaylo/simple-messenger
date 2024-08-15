use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::entity::user::{Email, Password};

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Validate)]
pub struct RegisterDataRequest {
    #[garde(dive)]
    pub email: Email,

    #[garde(dive)]
    pub password: Password,
}

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct RegisterDataResponse {}

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub enum RegisterDataClientError {
    Other,
}
