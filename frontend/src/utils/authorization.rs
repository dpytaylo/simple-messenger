use leptos::*;

#[derive(Debug, Clone)]
pub struct ClientAuthorization {
    pub token: RwSignal<Option<String>>,
}

pub fn provide_authorization() {
    provide_context(ClientAuthorization {
        token: Default::default(),
    });
}

pub fn use_authorization() -> ClientAuthorization {
    expect_context::<ClientAuthorization>()
}

impl ClientAuthorization {
    pub fn authorizate(&self, token: String) {
        self.token.set(Some(token));
    }
}
