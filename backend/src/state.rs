use anyhow::Context;
use axum::extract::FromRef;
use leptos::LeptosOptions;
use migration::{Migrator, MigratorTrait};
use oauth2::basic::BasicClient;
use rand_chacha::{
    rand_core::{OsRng, RngCore, SeedableRng},
    ChaCha8Rng,
};
use redis::Client as RedisClient;
use reqwest::Client as ReqwestClient;
use sea_orm::{Database, DatabaseConnection};

use crate::{auth::oauth, environment::Environment};

#[derive(Clone, FromRef)]
pub struct ServerState {
    pub random: ChaCha8Rng,
    pub reqwest: ReqwestClient,
    pub oauth: BasicClient,
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

        let oauth = oauth::google::create_basic_client(environment);

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
            oauth,
            redis,
            db,
            leptos_options,
        })
    }
}
