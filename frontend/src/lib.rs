use components::alert_message::AlertMessageProvider;
use leptos::*;
use leptos_router::{Route, Routes};
use pages::{
    app::{App, APP_PAGE_URL},
    auth::{
        registration::{
            oauth2::{SignUpOAuth2, SIGN_UP_OAUTH2_PAGE_URL},
            sign_up::SIGN_UP_PAGE_URL,
        },
        sign_in::SIGN_IN_PAGE_URL,
        success::{AuthSuccess, AUTH_SUCCESS_PAGE_URL},
    },
};
use utils::{authorization::provide_authorization, rpc_provider::provide_rpc_client};

use crate::{
    pages::auth::{registration::sign_up::SignUp, sign_in::SignIn},
    pages::root::Root,
};

mod atoms;
mod components;
mod pages;
mod utils;

pub use pages::error_template::ErrorTemplate;

#[component]
pub fn Frontend() -> impl IntoView {
    provide_authorization();
    provide_rpc_client("http://localhost:3000/api/rpc/");

    view! {
        <div class="font-content">
            <AlertMessageProvider/>
            <Routes>
                <Route path="" view=Root />
                <Route path=SIGN_IN_PAGE_URL view=SignIn />
                <Route path=SIGN_UP_PAGE_URL view=SignUp />
                <Route path=SIGN_UP_OAUTH2_PAGE_URL view=SignUpOAuth2 />
                <Route path=APP_PAGE_URL view=App />
                <Route path=AUTH_SUCCESS_PAGE_URL view=AuthSuccess />
            </Routes>
        </div>
    }
}
