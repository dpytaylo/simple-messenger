use std::time::Duration;

use backend_api::{
    entities::{email::Email, password::Password},
    routes::auth::authenticate::{Authenticate, AuthenticateError, AuthenticateRequest},
};
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
    utils::{client::use_client, defer::defer, error::log_rpc_error},
};

pub const SIGN_IN_PAGE_URL: &str = "/sign-in";

#[component]
pub fn SignIn() -> impl IntoView {
    let navigate = use_navigate();
    let alert = use_alert_message();
    let client = use_client();

    let email_node: NodeRef<html::Input> = create_node_ref();
    let password_node: NodeRef<html::Input> = create_node_ref();

    let (email_error, set_email_error) = create_signal(None);
    let (password_error, set_password_error) = create_signal(None);
    let disabled = Signal::derive(move || email_error().is_some() || password_error().is_some());

    let authenticate = create_action({
        let client = client.clone();
        move |input: &AuthenticateRequest| {
            let client = client.clone();
            let input = input.clone();

            async move { client.rpc.call::<Authenticate>(&input).await }
        }
    });
    let authenticate_value = authenticate.value();
    let (is_processing, set_is_processing) = create_signal(false);

    let on_continue = move |_| {
        set_is_processing(true);

        let email_value = email_node.get().unwrap().value();
        let password_value = password_node.get().unwrap().value();

        let (email, password) = match (Email::new(email_value), Password::new(password_value)) {
            (Ok(val), Ok(val2)) => (val, val2),
            (email_err, password_err) => {
                if let Err(err) = email_err {
                    set_email_error(Some(err.to_string()));
                }

                if let Err(err) = password_err {
                    set_password_error(Some(err.to_string()));
                }

                set_is_processing(false);
                return;
            }
        };

        authenticate.dispatch(AuthenticateRequest { email, password });
    };

    create_effect(move |_| {
        let Some(rpc_result) = authenticate_value.get() else {
            return;
        };

        defer! {
            set_is_processing(false);
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
                client.authorizate(val.token);
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
                                        let err = Email::new(event_target_value(&ev)).err().map(|val| val.to_string());
                                        set_email_error(err);
                                    }

                                    node_ref=email_node
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
                                        let err = Password::new(event_target_value(&ev)).err().map(|val| val.to_string());
                                        set_password_error(err);
                                    }

                                    node_ref=password_node
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
                        is_processing=is_processing
                        class="mt-4 min-[500px]:mt-0 w-full min-[500px]:w-fit h-12 min-[500px]:h-10"
                        on:click=on_continue
                    >
                        "Sign In"
                    </Button>
                </div>
            </div>
        </div>
    }
}
