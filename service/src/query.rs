use ::entity::{user, user::Entity as User};
use sea_orm::{prelude::Uuid, *};

pub struct Query;

impl Query {
    pub async fn find_user_by_id(db: &DbConn, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn find_user_by_email(
        db: &DbConn,
        email: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }
}
