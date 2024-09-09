use std::time::Duration;

use backend_api::routes::auth::registration::register_oauth2::{
    RegisterOAuth2Request, RegisterOauth2,
};
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use tracing::error;

use super::details::Details;
use crate::components::alert_message::{use_alert_message, MessageOptions, MessageVariant};
use crate::pages::app::APP_PAGE_URL;
use crate::pages::auth::registration::sign_up::SIGN_UP_PAGE_URL;
use crate::pages::auth::registration::summary_oauth2::SummaryOAuth2;
use crate::utils::client::use_client;
use crate::utils::defer::defer;
use crate::utils::error::log_rpc_error;

pub const SIGN_UP_OAUTH2_PAGE_URL: &str = "/sign-up-oauth2";

#[derive(Debug, Clone, PartialEq)]
enum SignUpOAuth2Step {
    Details,
    Summary,
    End,
}

impl SignUpOAuth2Step {
    fn back(self) -> Self {
        match self {
            Self::Details => panic!("OAuth2SignUpStep::back() called on OAuth2SignUpStep::Details"),
            Self::Summary => Self::Details,
            Self::End => Self::Summary,
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Details => Self::Summary,
            Self::Summary => Self::End,
            Self::End => panic!("OAuth2SignUpStep::next() called on OAuth2SignUpStep::End"),
        }
    }
}

#[component]
pub fn SignUpOAuth2() -> impl IntoView {
    let navigate = use_navigate();
    let alert = use_alert_message();
    let client = use_client();

    let Some(registration_token) = client.registration_token.get_untracked() else {
        error!("Registration token is missing");

        alert.create(
            "Something went wrong",
            MessageVariant::Failure,
            MessageOptions {
                description: Some("Please try sign up again".into()),
                duration: Some(Duration::from_secs(3)),
                ..Default::default()
            },
        );

        navigate(SIGN_UP_PAGE_URL, NavigateOptions::default());
        unreachable!("Page should be navigated");
    };

    let (step, set_step) = create_signal(SignUpOAuth2Step::Details);

    let name = create_rw_signal(None);

    let register = create_action({
        let client = client.clone();
        move |input: &RegisterOAuth2Request| {
            let client = client.clone();
            let input = input.clone();

            async move { client.rpc.call::<RegisterOauth2>(&input).await }
        }
    });
    let register_value = register.value();
    let is_processing = create_rw_signal(false);

    let back_step = move || set_step.update(|val| *val = val.clone().back());
    let next_step = move || set_step.update(|val| *val = val.clone().next());

    create_effect(move |_| match step() {
        SignUpOAuth2Step::End => register.dispatch(RegisterOAuth2Request {
            name: name.get_untracked().unwrap(),
            avatar: None,
        }),
        _ => (),
    });

    create_effect(move |_| {
        let Some(rpc_result) = register_value.get() else {
            return;
        };

        defer! {
            is_processing.set(false);
        }

        let result = match rpc_result {
            Ok(val) => val,
            Err(err) => {
                log_rpc_error(err);
                return;
            }
        };

        match result {
            Ok(val) => {
                client.authorizate(val.token);
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
            when=move || step() == SignUpOAuth2Step::Details
        >
            <Details next_step name />
        </Show>
        <Show
            when=move || step() == SignUpOAuth2Step::Summary
        >
            <SummaryOAuth2
                back_step
                next_step
                name=name.get_untracked().unwrap()
                is_processing
            />
        </Show>
    }
}
