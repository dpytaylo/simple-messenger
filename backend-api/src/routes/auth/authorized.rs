// use axum::response::{IntoResponse, Response};
// use http::StatusCode;
// use strum::IntoStaticStr;
// use thiserror::Error;

// use crate::error::wrap_error;

// #[derive(Debug, Error, IntoStaticStr)]
// pub enum AuthorizedError {
//     #[error(transparent)]
//     Other(#[from] anyhow::Error),
// }

// impl IntoResponse for AuthorizedError {
//     fn into_response(self) -> Response {
//         let code = match self {
//             AuthorizedError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
//         };

//         wrap_error(code, self)
//     }
// }

// pub enum AuthorizedClientError {
//     Other,
// }

// impl From<AuthorizedError> for AuthorizedClientError {
//     fn from(error: AuthorizedError) -> Self {
//         match error {
//             AuthorizedError::Other(_) => AuthorizedClientError::Other,
//         }
//     }
// }
