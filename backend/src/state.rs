use std::sync::Arc;

use anyhow::Context;
use axum::extract::FromRef;
use leptos::LeptosOptions;
use migration::{Migrator, MigratorTrait};
use rand_chacha::{
    rand_core::{OsRng, RngCore, SeedableRng},
    ChaCha8Rng,
};
use reqwest::Client as ReqwestClient;
use sea_orm::{Database, DatabaseConnection};

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
    pub leptos_options: LeptosOptions,
    pub random: ChaCha8Rng,
    pub reqwest: ReqwestClient,
    pub discord: DiscordClient,
    pub google: GoogleClient,
    pub db: DatabaseConnection,
    pub keys: Keys,
    pub mem: MemoryStorage,
}

impl ServerStateWrapper {
    pub async fn new(
        environment: &Environment,
        leptos_options: LeptosOptions,
    ) -> anyhow::Result<ServerStateWrapper> {
        let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
        let reqwest = ReqwestClient::builder()
            .brotli(true)
            .build()
            .context("Failed to initialize reqwest::Client")?;

        let discord = oauth::discord::create_basic_client(environment);
        let google = oauth::google::create_basic_client(environment);

        let db = Database::connect(format!(
            "postgres://postgres:{}@{}/simple_messenger",
            environment.postgres_password, environment.postgres_host,
        ))
        .await
        .context("SeaORM connection failed")?;
        Migrator::up(&db, None).await?;

        let keys = Keys::new(environment.jwt_secret.as_bytes());
        let mem = MemoryStorage::new();

        let this = ServerState {
            random,
            reqwest,
            discord,
            google,
            db,
            leptos_options,
            keys,
            mem,
        };

        Ok(Self {
            inner: Arc::new(this),
        })
    }
}

// Required by the "axum_garde" crate
impl axum::extract::FromRef<ServerStateWrapper> for () {
    fn from_ref(_: &ServerStateWrapper) {}
}

impl axum::extract::FromRef<ServerStateWrapper> for LeptosOptions {
    fn from_ref(this: &ServerStateWrapper) -> LeptosOptions {
        this.inner.leptos_options.clone()
    }
}

// impl axum::extract::FromRef<Arc<ServerState>> for () {
//     fn from_ref(_: &Arc<ServerState>) {}
// }
