use std::sync::Arc;

use anyhow::Context;
use axum::extract::FromRef;
use mem::MemoryDbService;
use reqwest::Client as ReqwestClient;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{
    authorization::Keys,
    environment::Environment,
    routes::auth::oauth::{self, discord::DiscordClient, google::GoogleClient},
};

#[derive(Clone, FromRef)]
pub struct ServerStateWrapper {
    pub inner: Arc<ServerState>,
}

pub struct ServerState {
    pub reqwest: ReqwestClient,
    pub discord: DiscordClient,
    pub google: GoogleClient,
    pub db: PgPool,
    pub keys: Keys,
    pub mem: MemoryDbService,
}

impl ServerStateWrapper {
    pub async fn new(environment: &Environment) -> anyhow::Result<ServerStateWrapper> {
        let reqwest = ReqwestClient::builder()
            .brotli(true)
            .build()
            .context("Failed to initialize reqwest::Client")?;

        let discord = oauth::discord::create_client(environment)?;
        let google = oauth::google::create_client(environment)?;

        let db = PgPoolOptions::new()
            .max_connections(20)
            .connect(&environment.database_url)
            .await
            .context("failed to connect to the database")?;

        let keys = Keys::new(environment.jwt_secret.as_bytes());
        let mem = MemoryDbService::new();

        let this = ServerState {
            reqwest,
            discord,
            google,
            db,
            keys,
            mem,
        };

        Ok(Self {
            inner: Arc::new(this),
        })
    }
}
