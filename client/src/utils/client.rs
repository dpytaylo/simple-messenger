use codee::string::FromToStringCodec;
use leptos::*;
use leptos_use::{use_cookie_with_options, SameSite, UseCookieOptions};

#[derive(Debug, Clone)]
pub struct Client {
    pub auth: Signal<Option<String>>,
    set_auth: WriteSignal<Option<String>>,
}

pub fn provide_client() {
    let (auth, set_auth) = use_cookie_with_options::<String, FromToStringCodec>(
        "auth",
        UseCookieOptions::default()
            .max_age(3_600_000)
            .same_site(SameSite::Strict),
    );

    provide_context(Client { auth, set_auth });
}

pub fn use_client() -> Client {
    expect_context::<Client>()
}

impl Client {
    pub fn authorizate(&self, token: String) {
        self.set_auth.set(Some(token));
    }
}
