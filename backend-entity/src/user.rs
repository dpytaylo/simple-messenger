use api::entities::{avatar_uri::AvatarUri, email::Email, registration_kind::RegistrationKind};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub registration_type: RegistrationKind,
    pub email: Email,
    pub password: Option<String>,
    pub avatar: Option<AvatarUri>,
}
