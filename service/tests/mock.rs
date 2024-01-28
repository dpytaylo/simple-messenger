#![feature(lazy_cell)]

use entity::user;
use sea_orm::{prelude::Uuid, Set, Unchanged};
use service::{
    mutation::{CreateUserData, Mutation},
    query::Query,
    RegistrationType,
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
                kind: RegistrationType::Email,
                email: "c@a.com".into(),
                password: "password".into(),
                name: "c".into(),
            },
        )
        .await
        .unwrap();

        assert!(user.id.is_set());
        assert_eq!(user.kind, Unchanged(RegistrationType::Email));
        assert_eq!(user.email, Unchanged("c@a.com".into()));
        assert_eq!(user.password, Unchanged("password".into()));
        assert_eq!(user.name, Unchanged("c".into()));
        assert_eq!(user.avatar, Unchanged(None));
    }
}
