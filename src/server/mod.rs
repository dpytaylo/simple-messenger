use anyhow::bail;
use axum::Router;
use backend::environment::Environment;
use backend::state::ServerStateWrapper;
use leptos::{provide_context, view};
use leptos_axum::LeptosRoutes;
use time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tracing::{info, Level};

use crate::App;

mod fileserv;

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

    let state = ServerStateWrapper::new(&environment, leptos_options.clone()).await?;

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_same_site(tower_cookies::cookie::SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));

    let app = Router::new()
        .nest("/api", backend::routes(state.clone()))
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let state = state.clone();
                move || provide_context(state.inner.clone())
            },
            App,
        )
        .fallback(fileserv::file_and_error_handler)
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(Level::INFO)),
                )
                .layer(CorsLayer::very_permissive())
                .layer(session_layer),
        )
        .with_state(state);

    let listener = TcpListener::bind(&addr).await?;

    info!("Listening on {}", &addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
