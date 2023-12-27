#![feature(lazy_cell)]

use entity::user;
use sea_orm::{prelude::Uuid, Set, Unchanged};
use service::{
    mutation::{CreateUserData, Mutation},
    query::Query,
};

use crate::prepare::*;

mod prepare;

async fn main() {
    let db = &prepare_mock_db();

    let user = Query::find_user_by_id(db, FIRST_UUID)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user, *USER_MODEL);

    let user = Query::find_user_by_id(db, Uuid::from_u128(u128::MAX - 1))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user.id, prepare::SECOND_UUID);

    let user = Query::find_user_by_email(db, "b@a.com")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user.id, prepare::SECOND_UUID);

    {
        let user = Mutation::create_user(
            db,
            CreateUserData {
                email: "c@a.com".to_owned(),
                password: "password".to_owned(),
                name: "c".to_owned(),
            },
        )
        .await
        .unwrap();

        assert!(user.id.is_set());
        assert_eq!(user.email, Unchanged("c@a.com".to_owned()));
        assert_eq!(user.password, Unchanged("password".to_owned()));
        assert_eq!(user.name, Unchanged("c".to_owned()));
        assert_eq!(user.avatar, Unchanged(None));
    }
}
