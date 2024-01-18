use api_error_derive::ApiErrorData;
use axum::{body::Body, response::Response};
use backend::api_error_to_server_fn_error;
use http::StatusCode;
use leptos::ServerFnError;

pub fn extraction_error(err: Response<Body>) -> ServerFnError {
    api_error_to_server_fn_error(ApiErrorData::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("{err:?}"),
        "InternalServerError".into(),
    ))
}
