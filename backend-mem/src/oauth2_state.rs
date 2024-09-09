use std::time::Duration;

use oauth2::{CsrfToken, PkceCodeVerifier};

use crate::ttl_map::TtlMap;

pub struct OAuth2State {
    map: TtlMap<String, String>,
}

impl OAuth2State {
    pub fn new() -> Self {
        Self {
            map: TtlMap::new(Duration::from_secs(60 * 10)), // 10 minutes
        }
    }

    pub fn insert(&self, key: &CsrfToken, verfier: &PkceCodeVerifier) {
        self.map
            .insert(key.secret().into(), verfier.secret().into());
    }

    pub fn take(&self, key: &CsrfToken) -> Option<PkceCodeVerifier> {
        self.map.take(key.secret()).map(PkceCodeVerifier::new)
    }
}
