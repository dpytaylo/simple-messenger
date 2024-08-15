use std::time::Duration;

use common::{
    entity::user::{Email, Password},
    routes::auth::authenticate::{
        AuthenticateClientError, AuthenticateRequest, AuthenticateResponse,
    },
};
use garde::Validate;
use leptos::*;
use leptos_router::{use_navigate, ActionForm, NavigateOptions};
use tracing::error;

use crate::{
    atoms::{anchor::Anchor, submit_button::SubmitButton},
    components::{
        alert_message::{use_alert_message, MessageOptions, MessageVariant},
        oauth2_links::OAuth2Links,
        or_break::OrBreak,
    },
    pages::{app::APP_PAGE_URL, auth::form_failed::FormFailed},
    utils::{authorization::use_authorization, error::wrap_action_value},
};

pub const AUTHENTICATION_PAGE_URL: &str = "authentication";

#[component]
pub fn Authentication() -> impl IntoView {
    let navigate = use_navigate();
    let alert = use_alert_message();
    let authorization = use_authorization();

    let action = create_server_action::<Authenticate>();
    let action_value = action.value();

    let (email_error, set_email_error) = create_signal(None);
    let (password_error, set_password_error) = create_signal(None);
    let disabled = Signal::derive(move || email_error().is_some() || password_error().is_some());

    let (form_failed, set_form_failed) = create_signal(None);

    wrap_action_value(alert.clone(), action_value, move |val| {
        let mut form = None;
        match val {
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
                AuthenticateClientError::InvalidCredentials => {
                    form = Some("Invalid email or password.".into());
                }
                AuthenticateClientError::Other => {
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
    });

    view! {
        <main class="
            absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
            max-w-xs w-full
        ">
            <div class="mb-5 p-10 border rounded-xl shadow-md">
                <p class="mb-3 text-xl text-center">"Welcome back!"</p>
                <FormFailed value=form_failed />
                <ActionForm action=action>
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
                                    let err = Email(event_target_value(&ev)).validate().err().map(|val| val.to_string());
                                    set_email_error(err);
                                }
                            />
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
                                    let err = Password(event_target_value(&ev)).validate().err().map(|val| val.to_string());
                                    set_password_error(err);
                                }
                            />
                        </label>
                    </div>

                    <SubmitButton value="Log In" disabled=disabled />
                </ActionForm>

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

#[server]
async fn authenticate(
    email: String,
    password: String,
) -> Result<Result<AuthenticateResponse, AuthenticateClientError>, ServerFnError> {
    use std::sync::Arc;

    use backend::state::ServerState;
    use garde::Unvalidated;

    let state = expect_context::<Arc<ServerState>>();

    let request = Unvalidated::new(AuthenticateRequest {
        email: Email(email),
        password: Password(password),
    })
    .validate()
    .map_err(|_| ServerFnError::new("validation failed"))?;

    Ok(
        backend::routes::auth::authenticate::authenticate(state, request)
            .await
            .map_err(Into::into),
    )
}
