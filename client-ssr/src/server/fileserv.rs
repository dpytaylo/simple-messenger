use axum::{
    body::Body,
    extract::{Request, State},
    response::{IntoResponse, Response},
};
use http::{StatusCode, Uri};
use leptos::*;
use tower::util::ServiceExt;
use tower_http::services::ServeDir;
use tracing::error;

use crate::App;

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
            let handler = leptos_axum::render_app_to_stream(options.clone().into(), App);

            handler(request).await
        }

        _ => {
            let handler = leptos_axum::render_app_to_stream(options.clone().into(), App);
            handler(request).await
        }
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
