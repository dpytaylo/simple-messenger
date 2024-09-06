use std::sync::Arc;

use anyhow::Context;
use axum::extract::FromRef;
use rand_chacha::{
    rand_core::{OsRng, RngCore, SeedableRng},
    ChaCha8Rng,
};
use reqwest::Client as ReqwestClient;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{
    authorization::Keys,
    environment::Environment,
    routes::auth::oauth::{self, discord::DiscordClient, google::GoogleClient},
    utils::memory_storage::MemoryStorage,
};

#[derive(Clone, FromRef)]
pub struct ServerStateWrapper {
    pub inner: Arc<ServerState>,
}

pub struct ServerState {
    pub random: ChaCha8Rng,
    pub reqwest: ReqwestClient,
    pub discord: DiscordClient,
    pub google: GoogleClient,
    pub db: PgPool,
    pub keys: Keys,
    pub mem: MemoryStorage,
}

impl ServerStateWrapper {
    pub async fn new(environment: &Environment) -> anyhow::Result<ServerStateWrapper> {
        let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
        let reqwest = ReqwestClient::builder()
            .brotli(true)
            .build()
            .context("Failed to initialize reqwest::Client")?;

        let discord = oauth::discord::create_basic_client(environment);
        let google = oauth::google::create_basic_client(environment);

        let db = PgPoolOptions::new()
            .max_connections(20)
            .connect(&environment.database_url)
            .await
            .context("failed to connect to the database")?;

        let keys = Keys::new(environment.jwt_secret.as_bytes());
        let mem = MemoryStorage::new();

        let this = ServerState {
            random,
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
