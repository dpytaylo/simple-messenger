use std::time::Duration;

use common::{
    entity::user::{Email, Password},
    routes::auth::authenticate::{Authenticate, AuthenticateError, AuthenticateRequest},
};
use garde::Validate;
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use tracing::error;

use crate::{
    atoms::{
        anchor::Anchor,
        button::{Button, ButtonKind},
    },
    components::{
        alert_message::{use_alert_message, MessageOptions, MessageVariant},
        oauth2_links::OAuth2Links,
    },
    pages::{app::APP_PAGE_URL, auth::registration::sign_up::SIGN_UP_PAGE_URL},
    utils::{client::use_client, error::log_rpc_error, rpc_provider::use_rpc_client},
};

pub const SIGN_IN_PAGE_URL: &str = "/sign-in";

#[component]
pub fn SignIn() -> impl IntoView {
    let navigate = use_navigate();
    let alert = use_alert_message();
    let authorization = use_client();

    let (email, set_email) = create_signal("".to_owned());
    let (password, set_password) = create_signal("".to_owned());

    let (email_error, set_email_error) = create_signal(None);
    let (password_error, set_password_error) = create_signal(None);
    let disabled = Signal::derive(move || email_error().is_some() || password_error().is_some());

    let authenticate = create_action(move |input: &AuthenticateRequest| {
        let rpc_client = use_rpc_client();
        let input = input.clone();

        async move { rpc_client.call::<Authenticate>(&input).await }
    });
    let authenticate_value = authenticate.value();

    let on_continue = move |_| {
        authenticate.dispatch(AuthenticateRequest {
            email: Email(email.get_untracked()),
            password: Password(password.get_untracked()),
        });
    };

    create_effect(move |_| {
        let Some(rpc_result) = authenticate_value.get() else {
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
                    "Successfully authenticated",
                    MessageVariant::Success,
                    MessageOptions {
                        duration: Some(Duration::from_secs(3)),
                        ..Default::default()
                    },
                );
                navigate(APP_PAGE_URL, NavigateOptions::default());
            }
            Err(err) => match err {
                AuthenticateError::InvalidCredentials => {
                    alert.create(
                        "Invalid email or password",
                        MessageVariant::Failure,
                        Default::default(),
                    );
                }
                AuthenticateError::Other => {
                    let description = format!("{err:?}");
                    error!(description = description, "Authentication failed");

                    alert.create(
                        "Authentication failed",
                        MessageVariant::Failure,
                        MessageOptions {
                            description: Some(description),
                            ..Default::default()
                        },
                    );
                }
            },
        }
    });

    view! {
        <div class="w-full lg:h-lvh bg-white lg:bg-slate-100">
            <div class="mx-auto mt-20 lg:mt-0 lg:mb-20 lg:relative lg:top-9/20 lg:-translate-y-1/2 max-w-screen-lg w-full px-4 sm:px-12 lg:py-16 rounded-xl bg-white">
                <div class="lg:grid lg:grid-cols-2 lg:gap-x-12">
                    <div>
                        <p class="text-4xl lg:text-5xl">"Welcome back!"</p>
                        <p class="mt-4">"Authenticate via email or one of the supported OAuth2 services."</p>
                    </div>
                    <div class="mt-10 lg:mt-0">
                        <div>
                            <label class="block">
                                <p>"Email"</p>
                                <input
                                    class="mt-1 h-11 px-2 py-1 w-full border border-gray-400 rounded-md"
                                    class=("border-2", move || email_error().is_some())
                                    class=("border-red-500", move || email_error().is_some())

                                    type="text"
                                    name="email"
                                    required=true
                                    placeholder="your email"
                                    autocomplete="email"

                                    on:input=move |ev| {
                                        let email = event_target_value(&ev);
                                        set_email(email.clone());

                                        let err = Email(email).validate().err().map(|val| val.to_string());
                                        set_email_error(err);
                                    }
                                />
                                <p class="my-1 p-1 h-8 text-red-500 text-sm">
                                    {email_error}
                                </p>
                            </label>

                            <label class="block">
                                <p>"Password"</p>
                                <input
                                    class="mt-1 h-11 px-2 py-1 w-full border border-gray-400 rounded-md"
                                    class=("border-2", move || password_error().is_some())
                                    class=("border-red-500", move || password_error().is_some())

                                    type="password"
                                    name="password"
                                    required=true
                                    placeholder="your password"
                                    autocomplete="current-password"

                                    on:input=move |ev| {
                                        let password = event_target_value(&ev);
                                        set_password(password.clone());

                                        let err = Password(password).validate().err().map(|val| val.to_string());
                                        set_password_error(err);
                                    }
                                />
                                <p class="my-1 p-1 h-8 text-red-500 text-sm">
                                    {password_error}
                                </p>
                            </label>
                        </div>

                        <p class="mt-2">"Via OAuth2 services"</p>
                        <OAuth2Links class="mt-1" />
                    </div>
                </div>
                <div class="mt-16 sm:mt-32 flex flex-col-reverse min-[500px]:flex-row min-[500px]:justify-between">
                    <div class="mt-4 min-[500px]:mt-0 flex flex-col items-stretch text-center">
                        <Anchor href=SIGN_UP_PAGE_URL>"Create a new account"</Anchor>
                    </div>
                    <Button
                        kind=ButtonKind::Primary
                        disabled=disabled
                        class="mt-4 min-[500px]:mt-0 w-full min-[500px]:w-28 h-12 min-[500px]:h-10"
                        on:click=on_continue
                    >
                        "Sign In"
                    </Button>
                </div>
            </div>
        </div>
    }
}
