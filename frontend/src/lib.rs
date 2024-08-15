use components::alert_message::AlertMessageProvider;
use leptos::*;
use leptos_router::{Route, Routes};
use pages::{
    app::{App, APP_PAGE_URL},
    auth::{
        authentication::AUTHENTICATION_PAGE_URL, registration::REGISTRATION_PAGE_URL,
        registration_details::REGISTRATION_DETAILS_PAGE_URL, success::AUTH_SUCCESS_PAGE_URL,
    },
};
use utils::authorization::provide_authorization;

use crate::{
    pages::auth::{
        authentication::Authentication, registration::Registration,
        registration_details::RegistrationDetails,
    },
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

    view! {
        <div class="font-content">
            <AlertMessageProvider/>
            <Routes>
                <Route path="" view=Root />
                <Route path=AUTHENTICATION_PAGE_URL view=Authentication />
                <Route path=REGISTRATION_PAGE_URL view=Registration />
                <Route path=REGISTRATION_DETAILS_PAGE_URL view=RegistrationDetails />
                <Route path=APP_PAGE_URL view=App />
                <Route path=AUTH_SUCCESS_PAGE_URL view=App />
            </Routes>
        </div>
    }
}
