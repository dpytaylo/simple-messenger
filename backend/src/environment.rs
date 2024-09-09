use std::env::{self, VarError};

use anyhow::anyhow;
use url::Url;

pub struct Environment {
    // Address to bind the server to. Example: `localhost:8080`.
    pub addr: String,

    // Server URL. Example: `http://localhost:8080`.
    pub host_url: Url,

    // URL to the PostgreSQL database. Example: `postgres://user:password@localhost:5432/database`.
    pub database_url: String,

    // Secret key for JWT token generation.
    pub jwt_secret: String,

    pub discord_client_id: String,
    pub discord_client_secret: String,

    pub google_client_id: String,
    pub google_client_secret: String,
}

impl Environment {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            addr: get_env("ADDR")?,
            host_url: Url::parse(&get_env("HOST_URL")?)?,
            database_url: get_env("DATABASE_URL")?,
            jwt_secret: get_env("JWT_SECRET")?,
            discord_client_id: get_env("DISCORD_CLIENT_ID")?,
            discord_client_secret: get_env("DISCORD_CLIENT_SECRET")?,
            google_client_id: get_env("GOOGLE_CLIENT_ID")?,
            google_client_secret: get_env("GOOGLE_CLIENT_SECRET")?,
        })
    }
}

fn get_env(name: &str) -> anyhow::Result<String> {
    env::var(name).map_err(|err| match err {
        VarError::NotPresent => anyhow!("{name} must be set"),
        VarError::NotUnicode(_) => anyhow!("{name} must be encoded in valid Unicode"),
    })
}
