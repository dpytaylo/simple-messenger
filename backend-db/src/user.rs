use api::entities::{
    avatar_uri::AvatarUri, email::Email, registration_kind::RegistrationKind, username::Username,
};
use entity::user::User;
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::Result;

#[derive(Debug, FromRow)]
pub struct UserEntity {
    pub id: Uuid,
    pub registration_kind: String,
    pub email: String,
    pub password: Option<String>,
    pub name: String,
    pub avatar: Option<String>,
}

pub async fn create(
    conn: &PgPool,
    registration_kind: RegistrationKind,
    email: &Email,
    password_hash: Option<&str>,
    name: &Username,
    avatar: Option<&AvatarUri>,
) -> Result<User> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
            insert into "user" (registration_kind, email, password, name, avatar)
            values ($1, $2, $3, $4, $5)
            returning id, registration_kind, email, password, name, avatar
        "#,
        registration_kind.to_string(),
        email.value(),
        password_hash,
        name.value(),
        avatar.map(AvatarUri::value),
    )
    .fetch_one(*&conn)
    .await?;

    user.to_model()
}

pub async fn find_by_id(conn: &PgPool, id: Uuid) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
            select id, registration_kind, email, password, name, avatar from "user"
            where id = $1
        "#,
        id
    )
    .fetch_optional(*&conn)
    .await?;

    user.map(UserEntity::to_model).transpose()
}

pub async fn find_by_email(conn: &PgPool, email: &Email) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
            select id, registration_kind, email, password, name, avatar from "user"
            where email = $1
        "#,
        email.value()
    )
    .fetch_optional(*&conn)
    .await?;

    user.map(UserEntity::to_model).transpose()
}

pub async fn find_by_name(conn: &PgPool, username: &Username) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
            select id, registration_kind, email, password, name, avatar from "user"
            where name = $1
        "#,
        username.value()
    )
    .fetch_optional(*&conn)
    .await?;

    user.map(UserEntity::to_model).transpose()
}

impl UserEntity {
    fn to_model(self) -> Result<User> {
        let avatar = match self.avatar {
            Some(val) => Some(AvatarUri::new(val)?),
            None => None,
        };

        Ok(User {
            id: self.id,
            registration_type: self.registration_kind.parse()?,
            email: Email::new(self.email)?,
            password: self.password,
            avatar,
        })
    }
}
