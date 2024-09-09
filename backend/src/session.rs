use anyhow::Context;
use serde::{de::DeserializeOwned, Serialize};
use tower_sessions::Session;

pub const JWT_TOKEN_KEY: &str = "jwt_token";
pub const REGISTRATION_KIND_KEY: &str = "registration_kind";
pub const REGISTRATION_EMAIL_KEY: &str = "registration_email";
pub const REGISTRATION_PASSWORD_KEY: &str = "registration_password";
pub const REGISTRATION_AVATAR_URI_KEY: &str = "registration_avatar_uri";

pub async fn insert_session_key(
    session: &Session,
    key: &str,
    value: impl Serialize,
) -> anyhow::Result<()> {
    session
        .insert(key, value)
        .await
        .context("failed to insert session key")
}

pub async fn get_session_value<T>(session: &Session, key: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    session
        .get(key)
        .await
        .context("failed to get session value")?
        .with_context(|| format!("there is no session value with the '{key}' key"))
}
