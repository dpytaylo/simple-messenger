use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Request, State},
    response::{IntoResponse, Response},
};
use backend::error::wrap_error;
use backend::state::ServerState;
use http::{StatusCode, Uri};
use leptos::*;
use strum::IntoStaticStr;
use thiserror::Error;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tracing::error;

use crate::App;

#[derive(Debug, Error, IntoStaticStr)]
enum FileAndErrorHandlerError {
    #[error("not found")]
    NotFound,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for FileAndErrorHandlerError {
    fn into_response(self) -> Response {
        let code = match self {
            FileAndErrorHandlerError::NotFound => StatusCode::NOT_FOUND,
            FileAndErrorHandlerError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        wrap_error(code, self)
    }
}

#[axum::debug_handler]
pub async fn file_and_error_handler(
    uri: Uri,
    State(state): State<Arc<ServerState>>,
    request: Request<Body>,
) -> Response {
    let root = state.leptos_options.site_root.clone();
    let response = get_static_file(uri.clone(), &root).await;

    match response.status() {
        StatusCode::OK => response,

        StatusCode::NOT_FOUND => {
            if uri.path().starts_with("/api/") {
                return FileAndErrorHandlerError::NotFound.into_response();
            }

            let handler =
                leptos_axum::render_app_to_stream(state.leptos_options.clone().into(), App);
            handler(request).await
        }

        code => FileAndErrorHandlerError::Other(anyhow!(
            "Invalid 'get_static_file()' response answer status code ({code})"
        ))
        .into_response(),
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
    ServeDir::new(root)
        .oneshot(request)
        .await
        .unwrap()
        .into_response()
}
