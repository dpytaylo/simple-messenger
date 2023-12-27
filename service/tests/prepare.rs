use std::str::FromStr;

use ::entity::user;
use once_cell::sync::Lazy;
use sea_orm::{prelude::Uuid, *};

pub const FIRST_UUID: Uuid = Uuid::from_u128(271933978467241048146062564402173984327);
pub const SECOND_UUID: Uuid = Uuid::from_u128(265428574764778157879973183191968264095);

pub static USER_MODEL: Lazy<user::Model> = Lazy::new(|| user::Model {
    id: FIRST_UUID,
    email: "a@a.com".to_owned(),
    password: "123".to_owned(),
    name: "a".to_owned(),
    avatar: None,
});

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            [(&*USER_MODEL).clone()],
            [user::Model {
                id: SECOND_UUID,
                email: "b@a.com".to_owned(),
                password: "456".to_owned(),
                name: "b".to_owned(),
                avatar: None,
            }],
        ])
        .into_connection()
}
