use axum::{middleware, response::Response, Router};
use error::{api_error_to_response, ApiError};
use state::ServerStateWrapper;
use tower::ServiceBuilder;

pub mod authorization;
pub mod cookies;
pub mod environment;
pub mod error;
pub mod extractors;
pub mod routes;
pub mod session;
pub mod state;
pub mod utils;

pub const INTERNAL_SERVER_ERROR_STR: &str = "InternalServerError";

pub fn app(state: ServerStateWrapper) -> Router<ServerStateWrapper> {
    Router::new()
        .nest("/", routes::routes(state))
        .layer(ServiceBuilder::new().layer(middleware::map_response(mw_main_response_mapper)))
}

async fn mw_main_response_mapper(mut response: Response) -> Response {
    if let Some(error_data) = response.extensions_mut().remove::<ApiError>() {
        return api_error_to_response(error_data);
    }

    response
}
