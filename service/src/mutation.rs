use ::entity::{
    channel, channel::Entity as Channel, message,
    sea_orm_active_enums::RegistrationType as DbRegistrationType, user, user::Entity as User,
};
use sea_orm::{prelude::Uuid, *};
use thiserror::Error;

use crate::RegistrationKind;

pub struct Mutation;

pub struct CreateUserData {
    pub kind: RegistrationKind,
    pub email: String,
    pub password: Option<String>,
    pub name: String,
    pub avatar: Option<String>,
}

pub struct CreateMessageData {
    pub sender_id: Uuid,
    pub channel_id: Uuid,
    pub content: String,
}

#[derive(Debug, Error)]
pub enum CreateMessageError {
    #[error("db error ({0})")]
    Db(#[from] DbErr),
    #[error("user with this id not found")]
    UserNotFound,
    #[error("channel with this id not found")]
    ChannelNotFound,
}

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        user_data: CreateUserData,
    ) -> Result<user::ActiveModel, DbErr> {
        let kind = match user_data.kind {
            RegistrationKind::Email => DbRegistrationType::Email,
            RegistrationKind::Discord => DbRegistrationType::Discord,
            RegistrationKind::Google => DbRegistrationType::Google,
        };

        user::ActiveModel {
            registration_type: Set(kind),
            email: Set(user_data.email),
            password: Set(user_data.password),
            name: Set(user_data.name),
            avatar: Set(user_data.avatar),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn create_channel(db: &DbConn, name: String) -> Result<channel::ActiveModel, DbErr> {
        channel::ActiveModel {
            name: Set(name),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn create_message(
        db: &DbConn,
        message_data: CreateMessageData,
    ) -> Result<message::ActiveModel, CreateMessageError> {
        User::find_by_id(message_data.sender_id)
            .one(db)
            .await?
            .ok_or(CreateMessageError::UserNotFound)?;

        Channel::find_by_id(message_data.channel_id)
            .one(db)
            .await?
            .ok_or(CreateMessageError::ChannelNotFound)?;

        message::ActiveModel {
            sender_id: Set(message_data.sender_id),
            channel_id: Set(message_data.channel_id),
            content: Set(message_data.content),
            ..Default::default()
        }
        .save(db)
        .await
        .map_err(CreateMessageError::from)
    }
}
