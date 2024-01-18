use anyhow::Context;
use axum::extract::FromRef;
use leptos::LeptosOptions;
use migration::{Migrator, MigratorTrait};
use rand_chacha::{
    rand_core::{OsRng, RngCore, SeedableRng},
    ChaCha8Rng,
};
use redis::Client as RedisClient;
use reqwest::Client as ReqwestClient;
use sea_orm::{Database, DatabaseConnection};

use crate::{
    auth::oauth::{self, discord::DiscordClient, google::GoogleClient},
    environment::Environment,
};

#[derive(Clone, Debug, FromRef)]
pub struct ServerState {
    pub random: ChaCha8Rng,
    pub reqwest: ReqwestClient,
    pub discord: DiscordClient,
    pub google: GoogleClient,
    pub redis: RedisClient,
    pub db: DatabaseConnection,
    pub leptos_options: LeptosOptions,
}

impl ServerState {
    pub async fn new(
        environment: &Environment,
        leptos_options: LeptosOptions,
    ) -> anyhow::Result<Self> {
        let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
        let reqwest = ReqwestClient::builder()
            .brotli(true)
            .build()
            .context("Failed to initialize reqwest::Client")?;

        let discord = oauth::discord::create_basic_client(environment);
        let google = oauth::google::create_basic_client(environment);

        let redis = RedisClient::open(format!(
            "redis://:{}@{}",
            environment.redis_password, environment.redis_host,
        ))
        .context("Redis connection failed")?;

        let db = Database::connect(format!(
            "postgres://postgres:{}@{}/simple_messenger",
            environment.postgres_password, environment.postgres_host,
        ))
        .await
        .context("SeaORM connection failed")?;

        Migrator::up(&db, None).await?;

        Ok(Self {
            random,
            reqwest,
            discord,
            google,
            redis,
            db,
            leptos_options,
        })
    }
}

// Required by the "axum_garde" crate
impl axum::extract::FromRef<ServerState> for () {
    fn from_ref(_: &ServerState) {}
}
