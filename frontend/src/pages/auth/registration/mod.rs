use std::time::Duration;

use common::routes::auth::registration::register::{Register, RegisterRequest};
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use tracing::error;

use self::details::Details;
use self::email::EmailPage;
use self::password::PasswordPage;
use self::summary::Summary;
use crate::components::alert_message::{use_alert_message, MessageOptions, MessageVariant};
use crate::pages::app::APP_PAGE_URL;
use crate::utils::authorization::use_authorization;
use crate::utils::error::log_rpc_error;
use crate::utils::rpc_provider::use_rpc_client;

mod details;
mod email;
mod password;
mod summary;

pub const REGISTRATION_PAGE_URL: &str = "/registration";

#[derive(Debug, Clone, PartialEq)]
enum RegistrationStep {
    Email,
    Password,
    Details,
    Summary,
    End,
}

impl RegistrationStep {
    fn back(self) -> Self {
        match self {
            Self::Email => panic!("RegisterStep::back() called on RegisterStep::Email"),
            Self::Password => Self::Email,
            Self::Details => Self::Password,
            Self::Summary => Self::Details,
            Self::End => Self::Summary,
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Email => Self::Password,
            Self::Password => Self::Details,
            Self::Details => Self::Summary,
            Self::Summary => Self::End,
            Self::End => panic!("RegisterStep::next() called on RegisterStep::End"),
        }
    }
}

#[component]
pub fn Registration() -> impl IntoView {
    let (step, set_step) = create_signal(RegistrationStep::Email);

    let email = create_rw_signal(None);
    let password = create_rw_signal(None);
    let name = create_rw_signal(None);

    let register = create_action(move |input: &RegisterRequest| {
        let input = input.clone();

        let navigate = use_navigate();
        let alert = use_alert_message();
        let authorization = use_authorization();
        let rpc_client = use_rpc_client();

        async move {
            let rpc_result = rpc_client.call::<Register>(&input).await;
            let result = match rpc_result {
                Ok(val) => val,
                Err(err) => {
                    log_rpc_error(err);
                    return;
                }
            };

            match result {
                Ok(val) => {
                    authorization.authorizate(val.token);
                    alert.create(
                        "Successfully registered",
                        MessageVariant::Success,
                        MessageOptions {
                            duration: Some(Duration::from_secs(3)),
                            ..Default::default()
                        },
                    );
                    navigate(APP_PAGE_URL, NavigateOptions::default());
                }
                Err(err) => {
                    let description = format!("{err:?}");
                    error!(description = description, "Registration failed");

                    alert.create(
                        "Registration failed",
                        MessageVariant::Failure,
                        MessageOptions {
                            description: Some(description),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    });

    let back_step = move || set_step.update(|val| *val = val.clone().back());
    let next_step = move || set_step.update(|val| *val = val.clone().next());

    view! {
        <Show
            when=move || step() == RegistrationStep::Email
        >
            <EmailPage next_step email />
        </Show>
        <Show
            when=move || step() == RegistrationStep::Password
        >
            <PasswordPage back_step next_step password />
        </Show>
        <Show
            when=move || step() == RegistrationStep::Details
        >
            <Details back_step next_step name />
        </Show>
        <Show
            when=move || step() == RegistrationStep::Summary
        >
            <Summary
                back_step
                next_step
                email=email.get_untracked().unwrap()
                name=name.get_untracked().unwrap()
                password=password.get_untracked().unwrap()
            />
        </Show>
    }
}
