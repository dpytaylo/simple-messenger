use std::env::{self, VarError};

use anyhow::anyhow;
use url::Url;

pub struct Environment {
    // Server URL. Example: `http://localhost:8080`.
    pub host_url: Url,
}

impl Environment {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            host_url: Url::parse(&get_env("HOST_URL")?)?,
        })
    }
}

fn get_env(name: &str) -> anyhow::Result<String> {
    env::var(name).map_err(|err| match err {
        VarError::NotPresent => anyhow!("{name} must be set"),
        VarError::NotUnicode(_) => anyhow!("{name} must be encoded in valid Unicode"),
    })
}
