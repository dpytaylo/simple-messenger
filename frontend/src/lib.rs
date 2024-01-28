use leptos::*;
use leptos_router::{Route, Routes};

use crate::{
    auth::{
        authentication::Authentication, registration::Registration,
        registration_details::RegistrationDetails,
    },
    home::Home,
};

mod auth;
mod chat;
mod error;
pub mod error_template;
mod home;

#[component]
pub fn Frontend() -> impl IntoView {
    view! {
        <div class="font-content">
            <Routes>
                <Route path="" view=Home />
                <Route path="authentication" view=Authentication />
                <Route path="registration" view=Registration />
                <Route path="registration_details" view=RegistrationDetails />
            </Routes>
        </div>
    }
}
