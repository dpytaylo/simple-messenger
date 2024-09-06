use anyhow::{bail, Context};
use axum::Router;
use backend::environment::Environment;
use backend::state::ServerStateWrapper;
use time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tracing::{info, Level};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
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
    let state = ServerStateWrapper::new(&environment).await?;

    db::migrate(&state.inner.db)
        .await
        .context("failed to migrate the database")?;

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_same_site(tower_cookies::cookie::SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));

    let app = Router::new()
        .nest("/api", backend::app(state.clone()))
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

    let listener = TcpListener::bind(&environment.addr).await?;

    info!("Listening on {}", &environment.addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
