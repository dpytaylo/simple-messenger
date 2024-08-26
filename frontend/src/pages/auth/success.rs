use std::time::Duration;

use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

use crate::{
    components::alert_message::{use_alert_message, MessageOptions, MessageVariant},
    pages::app::APP_PAGE_URL,
};

pub const AUTH_SUCCESS_PAGE_URL: &str = "/authorization-success";

#[component]
pub fn AuthSuccess() -> impl IntoView {
    let navigate = use_navigate();

    create_effect(move |_| {
        let alert = use_alert_message();
        alert.create(
            "Successfully authorized",
            MessageVariant::Success,
            MessageOptions {
                duration: Some(Duration::from_secs(3)),
                ..Default::default()
            },
        );

        navigate(APP_PAGE_URL, NavigateOptions::default());
    });

    view! {}
}
