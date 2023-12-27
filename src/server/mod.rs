use anyhow::bail;
use axum::body::Body;
use axum::extract::{Path, RawQuery, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{middleware, Router};
use backend::environment::Environment;
use backend::session;
use backend::state::ServerState;
use http::{HeaderMap, Request};
use leptos::{provide_context, view};
use leptos_axum::LeptosRoutes;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::App;

mod fileserv;

async fn server_fn_handler(
    State(server_state): State<ServerState>,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<Body>,
) -> impl IntoResponse {
    leptos_axum::handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move || {
            provide_context(server_state.clone());
        },
        request,
    )
    .await
}

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    match dotenvy::dotenv() {
        Ok(_) => info!("The environment(.env) file was succesfully loaded"),
        Err(err) => match err {
            dotenvy::Error::Io(_) => info!("The environment(.env) file not found"),
            _ => bail!("Failed to parse the environment(.env) file"),
        },
    }

    let environment = Environment::new()?;

    let conf = leptos::get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = leptos_axum::generate_route_list(|| view! { <App/> });

    let state = ServerState::new(&environment, leptos_options.clone()).await?;

    let app = Router::new()
        .nest("/api", backend::routes())
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes(&state, routes, App)
        .fallback(fileserv::file_and_error_handler)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::very_permissive())
                .layer(CookieManagerLayer::new())
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    session::mw_session_context_resolver,
                ))
                .layer(middleware::map_response(backend::mw_main_response_mapper)),
        )
        .with_state(state);

    let listener = TcpListener::bind(&addr).await?;

    info!("Listening on {}", &addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
