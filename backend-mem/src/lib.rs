use oauth2_registration::OAuth2Registration;
use oauth2_state::OAuth2State;

pub mod oauth2_registration;
pub mod oauth2_state;
mod ttl_map;

pub struct MemoryDbService {
    pub oauth2_state: OAuth2State,
    pub oauth2_registration: OAuth2Registration,
}

impl MemoryDbService {
    pub fn new() -> Self {
        Self {
            oauth2_state: OAuth2State::new(),
            oauth2_registration: OAuth2Registration::new(),
        }
    }
}
