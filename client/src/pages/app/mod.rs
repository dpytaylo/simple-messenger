use leptos::*;
use leptos_router::Redirect;

use self::main::Main;
use self::menu::Menu;
use self::workzone::Workzone;
use crate::{pages::auth::sign_in::SIGN_IN_PAGE_URL, utils::client::use_client};

pub mod channels;
pub mod chat;
pub mod control_panel;
pub mod main;
pub mod menu;
pub mod top;
pub mod workzone;

pub const APP_PAGE_URL: &str = "/app";

#[component]
pub fn App() -> impl IntoView {
    let auth = use_client();

    if auth.auth.get_untracked().is_none() {
        return view! {
            <Redirect path=SIGN_IN_PAGE_URL />
        }
        .into_view();
    };

    view! {
        // <p>"Your authorization token is "{move || format!("{:?}", (auth.auth)())}</p>

        <div class="w-screen h-screen flex overflow-hidden">
            <Menu/>
            <Workzone/>
            <Main/>
        </div>
    }
    .into_view()
}
