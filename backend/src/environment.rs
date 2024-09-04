use std::{
    env::{self, VarError},
    fs,
};

use anyhow::{anyhow, bail, Context};

pub struct Environment {
    pub jwt_secret: String,

    pub redis_host: String,
    pub redis_password: String,

    pub postgres_host: String,
    pub postgres_password: String,

    pub redirect_url: String,

    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub google_client_id: String,
    pub google_client_secret: String,
}

// TODO replace with clap
impl Environment {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            jwt_secret: get_env("JWT_SECRET")?,

            redis_host: get_optional_env("REDIS_HOST")?.unwrap_or_else(|| "localhost:6379".into()),
            redis_password: get_secret("REDIS_PASSWORD")?,

            postgres_host: get_optional_env("POSTGRES_HOST")?
                .unwrap_or_else(|| "localhost:5432".into()),
            postgres_password: get_secret("POSTGRES_PASSWORD")?,

            redirect_url: get_env("REDIRECT_URL")?,

            discord_client_id: get_secret("DISCORD_CLIENT_ID")?,
            discord_client_secret: get_secret("DISCORD_CLIENT_SECRET")?,
            google_client_id: get_secret("GOOGLE_CLIENT_ID")?,
            google_client_secret: get_secret("GOOGLE_CLIENT_SECRET")?,
        })
    }
}

fn get_env(name: &str) -> anyhow::Result<String> {
    env::var(name).map_err(|err| match err {
        VarError::NotPresent => anyhow!("{name} must be set"),
        VarError::NotUnicode(_) => anyhow!("{name} must be encoded in valid Unicode"),
    })
}

fn get_optional_env(name: &str) -> anyhow::Result<Option<String>> {
    match env::var(name) {
        Ok(val) => Ok(Some(val)),
        Err(err) => match err {
            VarError::NotPresent => Ok(None),
            VarError::NotUnicode(_) => bail!("{name} must be encoded in valid Unicode"),
        },
    }
}

fn get_secret(name: &str) -> anyhow::Result<String> {
    let file = format!("{name}_FILE");

    match env::var(&file) {
        Ok(val) => {
            fs::read_to_string(val).with_context(|| format!("Failed to read the {file} file"))
        }
        Err(err) => {
            match err {
                VarError::NotPresent => (),
                VarError::NotUnicode(_) => bail!("{file} must be encoded in valid Unicode"),
            }

            get_env(name)
        }
    }
}
