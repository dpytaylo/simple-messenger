use std::time::Duration;

use common::routes::auth::registration::register::{Register, RegisterRequest};
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use tracing::error;

use super::details::Details;
use super::email::EmailPage;
use super::password::PasswordPage;
use super::summary::Summary;
use crate::components::alert_message::{use_alert_message, MessageOptions, MessageVariant};
use crate::pages::app::APP_PAGE_URL;
use crate::utils::client::use_client;
use crate::utils::error::log_rpc_error;
use crate::utils::rpc_provider::use_rpc_client;

pub const SIGN_UP_PAGE_URL: &str = "/sign-up";

#[derive(Debug, Clone, PartialEq)]
enum SignUpStep {
    Email,
    Password,
    Details,
    Summary,
    End,
}

impl SignUpStep {
    fn back(self) -> Self {
        match self {
            Self::Email => panic!("SignUpStep::back() called on SignUpStep::Email"),
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
            Self::End => panic!("SignUpStep::next() called on SignUpStep::End"),
        }
    }
}

#[component]
pub fn SignUp() -> impl IntoView {
    let navigate = use_navigate();
    let alert = use_alert_message();
    let authorization = use_client();

    let (step, set_step) = create_signal(SignUpStep::Email);

    let email = create_rw_signal(None);
    let password = create_rw_signal(None);
    let name = create_rw_signal(None);

    let register = create_action(move |input: &RegisterRequest| {
        let rpc_client = use_rpc_client();
        let input = input.clone();

        async move { rpc_client.call::<Register>(&input).await }
    });
    let register_value = register.value();

    let back_step = move || set_step.update(|val| *val = val.clone().back());
    let next_step = move || set_step.update(|val| *val = val.clone().next());

    create_effect(move |_| match step() {
        SignUpStep::End => register.dispatch(RegisterRequest {
            email: email.get_untracked().unwrap(),
            password: password.get_untracked().unwrap(),
            name: name.get_untracked().unwrap(),
            avatar: None,
        }),
        _ => (),
    });

    create_effect(move |_| {
        let Some(rpc_result) = register_value.get() else {
            return;
        };

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
    });

    view! {
        <Show
            when=move || step() == SignUpStep::Email
        >
            <EmailPage next_step email />
        </Show>
        <Show
            when=move || step() == SignUpStep::Password
        >
            <PasswordPage back_step next_step password />
        </Show>
        <Show
            when=move || step() == SignUpStep::Details
        >
            <Details back_step=Box::new(back_step) next_step name />
        </Show>
        <Show
            when=move || step() == SignUpStep::Summary
        >
            <Summary
                back_step
                next_step
                email=email.get_untracked().unwrap()
                password=password.get_untracked().unwrap()
                name=name.get_untracked().unwrap()
            />
        </Show>
    }
}
