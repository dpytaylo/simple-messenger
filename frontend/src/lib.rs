use leptos::*;
use leptos_router::{Route, Routes};

use crate::auth::{
    authentication::Authentication, registration::Registration,
    registration_details::RegistrationDetails,
};

pub mod auth;
pub mod chat;
pub mod error_template;
mod validation;

#[component]
pub fn Frontend() -> impl IntoView {
    view! {
        <div class="font-content">
            <Routes>
                <Route path="" view=|| view! { "Home url" }/>
                <Route path="authentication" view=Authentication />
                <Route path="registration" view=Registration />
                <Route path="registration_details" view=RegistrationDetails />
            </Routes>
        </div>
    }
}
