use std::time::Duration;

use api::entities::{avatar_uri::AvatarUri, email::Email, registration_kind::RegistrationKind};

use crate::ttl_map::TtlMap;

#[derive(Clone)]
pub struct RegistrationData {
    pub registration_kind: RegistrationKind,
    pub email: Email,
    pub avatar_uri: Option<AvatarUri>,
}

pub struct OAuth2Registration {
    map: TtlMap<String, RegistrationData>,
}

impl OAuth2Registration {
    pub fn new() -> Self {
        Self {
            map: TtlMap::new(Duration::from_secs(15 * 60)), // 15 minutes
        }
    }

    pub fn insert(&self, key: String, value: RegistrationData) {
        self.map.insert(key, value);
    }

    // TODO &String -> &str
    pub fn get(&self, key: &String) -> Option<RegistrationData> {
        self.map.get(key)
    }

    pub fn take(&self, key: &String) -> Option<RegistrationData> {
        self.map.take(key)
    }
}
