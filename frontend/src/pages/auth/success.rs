use std::time::Duration;

use leptos::*;
use leptos_router::Redirect;

use crate::{
    components::alert_message::{use_alert_message, MessageOptions, MessageVariant},
    pages::app::APP_PAGE_URL,
    utils::authorization::use_authorization,
};

pub const AUTH_SUCCESS_PAGE_URL: &str = "/oauth2_authentication";

#[component]
pub fn Success() -> impl IntoView {
    let auth = use_authorization();
    // auth.authorizate()

    let alert = use_alert_message();
    alert.create(
        "Successfully authorized",
        MessageVariant::Success,
        MessageOptions {
            duration: Some(Duration::from_secs(3)),
            ..Default::default()
        },
    );

    view! {}
}
