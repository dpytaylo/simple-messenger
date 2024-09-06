use error::DbError;
use sqlx::{migrate::MigrateError, PgPool};

pub mod channel;
pub mod error;
pub mod message;
pub mod user;

pub type Result<T> = std::result::Result<T, DbError>;

pub async fn migrate(conn: &PgPool) -> std::result::Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(conn).await
}
