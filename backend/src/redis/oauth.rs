use oauth2::{CsrfToken, PkceCodeVerifier};
use redis::{AsyncCommands, Client, RedisError};

use super::{OAUTH_STATE_STORAGE, SELECT};

pub async fn insert_state(
    redis: &Client,
    crsf_token: CsrfToken,
    pkce_verifier: PkceCodeVerifier,
    seconds: u64,
) -> Result<(), RedisError> {
    let mut connection = redis.get_async_connection().await?;

    redis::cmd(SELECT)
        .arg(OAUTH_STATE_STORAGE)
        .query_async(&mut connection)
        .await?;

    connection
        .set_ex(
            crsf_token.secret().to_owned(),
            pkce_verifier.secret().to_owned(),
            seconds,
        )
        .await?;

    Ok(())
}

pub async fn take_state(redis: &Client, crsf_token: CsrfToken) -> Result<String, RedisError> {
    let mut connection = redis.get_async_connection().await?;

    redis::cmd(SELECT)
        .arg(OAUTH_STATE_STORAGE)
        .query_async(&mut connection)
        .await?;

    connection.get_del(crsf_token.secret().to_owned()).await
}
