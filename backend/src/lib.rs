use api_error_derive::ApiErrorData;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use state::ServerState;
use tracing::error;
use uuid::Uuid;

pub mod auth;
pub mod cookies;
pub mod environment;
pub mod redis;
pub mod session;
pub mod state;
pub mod validator;

pub const INTERNAL_SERVER_ERROR_STR: &str = "InternalServerError";

pub fn routes() -> Router<ServerState> {
    Router::new().nest("/auth", auth::routes())
    // .layer(middleware::from_fn_with_state(state.clone(), session::mw_session_context_resolver))
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    error: ErrorResponseData,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponseData {
    kind: String,
    uuid: Uuid,
}

pub fn api_error_to_response(error: ApiErrorData) -> Response {
    let uuid = Uuid::new_v4();

    let response = ErrorResponse {
        error: ErrorResponseData {
            kind: error.client_description,
            uuid,
        },
    };

    error!(status_code = error.status_code.as_u16(), description = error.description, %uuid);
    (error.status_code, Json(response)).into_response()
}

pub async fn mw_main_response_mapper(mut response: Response) -> Response {
    if let Some(error_data) = response.extensions_mut().remove::<ApiErrorData>() {
        return api_error_to_response(error_data);
    }

    response
}
