use std::sync::LazyLock;

use ::entity::{sea_orm_active_enums::RegistrationType, user};
use sea_orm::prelude::Uuid;

pub const FIRST_UUID: Uuid = Uuid::from_u128(271933978467241048146062564402173984327);
pub const SECOND_UUID: Uuid = Uuid::from_u128(265428574764778157879973183191968264095);

pub static USER_MODEL: LazyLock<user::Model> = LazyLock::new(|| user::Model {
    id: FIRST_UUID,
    registration_type: RegistrationType::Email,
    email: "a@a.com".into(),
    password: Some("123".into()),
    name: "a".into(),
    avatar: None,
});

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            [(&*USER_MODEL).clone()],
            [user::Model {
                id: SECOND_UUID,
                registration_type: RegistrationType::Email,
                email: "b@a.com".into(),
                password: "456".into(),
                name: "b".into(),
                avatar: None,
            }],
        ])
        .into_connection()
}
