use anyhow::bail;
use axum::Router;
use leptos::view;
use leptos_axum::LeptosRoutes;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
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

    let conf = leptos::get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = leptos_axum::generate_route_list(|| view! { <App/> });

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(fileserv::file_and_error_handler)
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(Level::INFO)),
                )
                .layer(CorsLayer::very_permissive()),
        )
        .with_state(leptos_options);

    let listener = TcpListener::bind(&addr).await?;

    info!("Listening on {}", &addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
