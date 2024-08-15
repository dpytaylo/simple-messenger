use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use oauth2::{CsrfToken, PkceCodeVerifier};

pub struct MemoryStorage {
    oauth_states: Arc<Mutex<HashMap<String, (Instant, PkceCodeVerifier)>>>,
}

// TODO replace with redis-like database
impl MemoryStorage {
    pub fn new() -> Self {
        let oauth_states: Arc<Mutex<HashMap<String, (Instant, PkceCodeVerifier)>>> =
            Default::default();

        tokio::spawn({
            let oauth_states = oauth_states.clone();
            async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(1)).await;

                    oauth_states
                        .lock()
                        .unwrap()
                        .retain(|_k, v| v.0.elapsed() < Duration::from_secs(10 * 60));
                }
            }
        });

        Self { oauth_states }
    }

    pub fn insert_oauth_state(&self, csrf_token: CsrfToken, pkce_verifier: PkceCodeVerifier) {
        self.oauth_states
            .lock()
            .unwrap()
            .insert(csrf_token.secret().into(), (Instant::now(), pkce_verifier));
    }

    pub fn take_oauth_state(&self, csrf_token: &CsrfToken) -> Option<PkceCodeVerifier> {
        self.oauth_states
            .lock()
            .unwrap()
            .remove(csrf_token.secret().as_str())
            .map(|val| val.1)
    }
}
