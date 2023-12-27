use redis::{AsyncCommands, Client, RedisError};

use super::{SELECT, SESSION_STORAGE};

pub async fn insert_token(
    redis: &Client,
    token: String,
    client_id: String,
    seconds: u64,
) -> Result<(), RedisError> {
    let mut connection = redis.get_async_connection().await?;

    redis::cmd(SELECT)
        .arg(SESSION_STORAGE)
        .query_async(&mut connection)
        .await?;

    connection.set_ex(token, client_id, seconds).await?;
    Ok(())
}
