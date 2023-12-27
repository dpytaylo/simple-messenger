use api_error_derive::ApiError;
use axum::{
    body::Body,
    extract::{Request, State},
    response::{IntoResponse, Response},
};
use http::{StatusCode, Uri};
use leptos::*;
use thiserror::Error;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tracing::error;

use crate::App;

#[derive(ApiError, Debug, Error)]
enum FileAndErrorHandlerError {
    #[error("not found")]
    #[status_code(NOT_FOUND)]
    NotFound,

    #[error("internal server error")]
    InternalServerError,
}

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    request: Request<Body>,
) -> Response {
    let root = options.site_root.clone();
    let response = get_static_file(uri.clone(), &root).await;

    match response.status() {
        StatusCode::OK => response,

        StatusCode::NOT_FOUND => {
            if uri.path().starts_with("/api/") {
                return backend::api_error_to_response(FileAndErrorHandlerError::NotFound.into());
            }

            let handler = leptos_axum::render_app_to_stream(options.to_owned(), App);
            handler(request).await
        }

        _ => backend::api_error_to_response(FileAndErrorHandlerError::InternalServerError.into()),
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Response {
    let request = match Request::builder().uri(uri.clone()).body(Body::empty()) {
        Ok(val) => val,
        Err(err) => {
            error!("request builder error ({err})");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(request).await {
        Ok(val) => val.into_response(),
        Err(err) => {
            error!("serve dir error ({err})");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
