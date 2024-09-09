use std::sync::Arc;

use anyhow::bail;
use api::routes::auth::oauth::discord::authorized::OAuth2DiscordAuthorizedResponse;
use api::routes::auth::oauth::google::authorized::OAuth2GoogleAuthorizedRequest;
use api::routes::auth::oauth::{
    discord::authorized::{OAuth2DiscordAuthorizedRequest, Oauth2DiscordAuthorized},
    google::authorized::Oauth2GoogleAuthorized,
};
use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
    Router,
};
use leptos::{provide_context, view};
use leptos_axum::LeptosRoutes;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::{info, instrument, Level};

use self::environment::Environment;
use self::state::ServerStateWrapper;
use crate::server::state::ServerState;
use crate::App;

mod environment;
mod fileserv;
mod state;

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

    let environment = Environment::load()?;

    let conf = leptos::get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = leptos_axum::generate_route_list(|| view! { <App/> });

    let state = ServerStateWrapper::new(&environment, leptos_options.clone());

    let app = Router::new()
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
                .layer(CorsLayer::very_permissive()),
        )
        .with_state(state);

    let listener = TcpListener::bind(&addr).await?;

    info!("Listening on {}", &addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
