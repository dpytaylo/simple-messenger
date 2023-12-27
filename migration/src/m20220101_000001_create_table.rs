use common::{MAX_USER_EMAIL_SIZE, MAX_USER_NAME_SIZE};
use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

const FK_MESSAGE_USER: &str = "FK_Message_User";
const FK_MESSAGE_CHANNEL: &str = "FK_Message_Channel";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    RegistrationType,
    Password,
    Name,
    Avatar,
}

#[derive(Iden, EnumIter)]
enum RegistrationType {
    Table,
    Email,
    Google,
}

#[derive(DeriveIden)]
enum Channel {
    Table,
    Id,
    CreatedAt,
    Name,
}

#[derive(DeriveIden)]
enum Message {
    Table,
    Id,
    CreatedAt,
    SenderId,
    ChannelId,
    Content,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(RegistrationType::Table)
                    .values(RegistrationType::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(SimpleExpr::Custom("gen_random_uuid()".to_owned())),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string_len(MAX_USER_EMAIL_SIZE.try_into().unwrap())
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::RegistrationType)
                            .enumeration(RegistrationType::Table, RegistrationType::iter().skip(1)),
                    )
                    .col(ColumnDef::new(User::Password).text())
                    .col(
                        ColumnDef::new(User::Name)
                            .string_len(MAX_USER_NAME_SIZE.try_into().unwrap())
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Avatar).string_len(150))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Channel::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Channel::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(SimpleExpr::Custom("gen_random_uuid()".to_owned())),
                    )
                    .col(
                        ColumnDef::new(Channel::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Channel::Name)
                            .string_len(32)
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(SimpleExpr::Custom("gen_random_uuid()".to_owned())),
                    )
                    .col(
                        ColumnDef::new(Message::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Message::SenderId).uuid().not_null())
                    .col(ColumnDef::new(Message::ChannelId).uuid().not_null())
                    .col(ColumnDef::new(Message::Content).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name(FK_MESSAGE_USER)
                    .from(Message::Table, Message::SenderId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name(FK_MESSAGE_CHANNEL)
                    .from(Message::Table, Message::ChannelId)
                    .to(Channel::Table, Channel::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_MESSAGE_CHANNEL)
                    .table(Message::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_MESSAGE_USER)
                    .table(Message::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Channel::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(RegistrationType::Table).to_owned())
            .await
    }
}
