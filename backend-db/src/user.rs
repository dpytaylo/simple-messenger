use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct UserEntity {
    pub id: Uuid,
    pub registration_type: String,
    pub email: String,
    pub password: Option<String>,
    pub avatar: Option<String>,
}

pub async fn create(
    conn: &PgPool,
    email: &str,
    password: Option<&str>,
    avatar: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            insert into "user" (registration_type, email, password, avatar)
            values ($1, $2, $3, $4)
        "#,
        "email",
        email,
        password,
        avatar,
    )
    .execute(*&conn)
    .await?;

    Ok(())
}

pub async fn find_by_id(conn: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
            select * from "user"
            where id = $1
        "#,
        id
    )
    .fetch_one(*&conn)
    .await?;

    Ok(user)
}

pub async fn find_by_email(conn: &PgPool, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
            select * from "user"
            where email = $1
        "#,
        email
    )
    .fetch_one(*&conn)
    .await?;

    Ok(user)
}
