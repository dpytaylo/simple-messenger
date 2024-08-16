use std::time::Duration;

use common::{
    entity::user::{Email, Password},
    routes::auth::authenticate::{Authenticate, AuthenticateError, AuthenticateRequest},
};
use ev::SubmitEvent;
use garde::Validate;
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use tracing::error;

use crate::{
    atoms::{anchor::Anchor, submit_button::SubmitButton},
    components::{
        alert_message::{use_alert_message, MessageOptions, MessageVariant},
        oauth2_links::OAuth2Links,
        or_break::OrBreak,
    },
    pages::{app::APP_PAGE_URL, auth::form_failed::FormFailed},
    utils::{authorization::use_authorization, error::log_rpc_error, rpc_provider::use_rpc_client},
};

pub const AUTHENTICATION_PAGE_URL: &str = "authentication";

#[component]
pub fn Authentication() -> impl IntoView {
    // let navigate = use_navigate();
    // let alert = use_alert_message();
    // let authorization = use_authorization();
    // let rpc_client = use_rpc_client();

    let (email, set_email) = create_signal("".to_owned());
    let (password, set_password) = create_signal("".to_owned());

    let (email_error, set_email_error) = create_signal(None);
    let (password_error, set_password_error) = create_signal(None);
    let disabled = Signal::derive(move || email_error().is_some() || password_error().is_some());

    let (form_failed, set_form_failed) = create_signal(None);

    let authenticate = create_action(move |input: &AuthenticateRequest| {
        // let navigate = navigate.clone();
        // let alert = alert.clone();
        // let authorization = authorization.clone();
        // let rpc_client = rpc_client.clone();
        let input = input.clone();

        let navigate = use_navigate();
        let alert = use_alert_message();
        let authorization = use_authorization();
        let rpc_client = use_rpc_client();

        async move {
            let rpc_result = rpc_client.call::<Authenticate>(input).await;
            let result = match rpc_result {
                Ok(val) => val,
                Err(err) => {
                    log_rpc_error(err);
                    return;
                }
            };

            let mut form = None;
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
                        form = Some("Invalid email or password.".into());
                    }
                    AuthenticateError::Other => {
                        let description = format!("{err:?}");
                        error!(title = "Authentication failed", description = description);

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

            set_form_failed(form);
        }
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        authenticate.dispatch(AuthenticateRequest {
            email: Email(email.get_untracked()),
            password: Password(password.get_untracked()),
        });
    };

    view! {
        <main class="
            absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
            max-w-xs w-full
        ">
            <div class="mb-5 p-10 border rounded-xl shadow-md">
                <p class="mb-3 text-xl text-center">"Welcome back!"</p>
                <FormFailed value=form_failed />
                <form on:submit=on_submit>
                    <div class="mb-5 space-y-4">
                        <label class="block">
                            <p class="mb-1 text-sm">"Email"</p>
                            <input
                                class="h-8 px-2 py-1 w-full border border-gray-400 rounded-md text-sm"
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
                            {move || email_error().map(|err| view! {
                                <p class="my-1 p-1 text-red-500">{err}</p>
                            })}
                        </label>

                        <label class="block">
                            <p class="mb-1 text-sm">"Password"</p>
                            <input
                                class="h-8 px-2 py-1 w-full border border-gray-400 rounded-md text-sm"
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
                            {move || password_error().map(|err| view! {
                                <p class="my-1 p-1 text-red-500">{err}</p>
                            })}
                        </label>
                    </div>

                    <SubmitButton value="Log In" disabled=disabled />
                </form>

                <OrBreak/>
                <OAuth2Links/>
                <OrBreak/>

                <p class="text-center">
                    <Anchor href="/registration">"Create a new account."</Anchor>
                </p>
            </div>
        </main>
    }
}
