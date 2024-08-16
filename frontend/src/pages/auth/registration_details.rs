use std::time::Duration;

use common::{
    entity::user::{Email, Name},
    routes::auth::register::{RegisterClientError, RegisterRequest, RegisterResponse},
};
use garde::Validate;
use leptos::*;
use leptos_router::{use_navigate, ActionForm, NavigateOptions};
use tracing::error;

use crate::{
    atoms::submit_button::SubmitButton,
    components::alert_message::{use_alert_message, MessageOptions, MessageVariant},
    pages::{app::APP_PAGE_URL, auth::form_failed::FormFailed},
    utils::{authorization::use_authorization, error::wrap_action_value},
};

pub const REGISTRATION_DETAILS_PAGE_URL: &str = "/registration_details";

#[component]
pub fn RegistrationDetails() -> impl IntoView {
    let navigate = use_navigate();
    let alert = use_alert_message();
    let authorization = use_authorization();

    let action = create_server_action::<Register>();
    let action_value = action.value();

    let (name_error, set_name_error) = create_signal(None);
    let disabled = Signal::derive(move || name_error().is_some());

    let (form_failed, set_form_failed) = create_signal(None);
    wrap_action_value(alert.clone(), action_value, move |val| {
        let mut form = None;

        match val {
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
            Err(err) => match err {
                RegisterClientError::AccountWithSameEmailAlreadyExists => {
                    form = Some("Account with the same email already exists!".into());
                }
                err @ (RegisterClientError::NoPassword | RegisterClientError::Other) => {
                    let description = format!("{err:?}");

                    error!(title = "Registration failed", description = description);
                    alert.create(
                        "Registration failed",
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
            absolute top-2/5 left-1/2 -translate-x-1/2 -translate-y-1/2
            max-w-xs w-full 
        ">
            <div class="mb-5 p-10 border rounded-xl shadow-md">
                <p class="mb-5 text-xl text-center">"Details"</p>
                <FormFailed value=form_failed />
                <ActionForm action=action>
                    <div class="mb-5 space-y-4 text-sm">
                        <label class="block">
                            <p class="mb-1">"Name"</p>
                            <input
                                class="h-8 px-2 py-1 w-full border border-gray-400 rounded-md text-sm"
                                class=("border-2", move || name_error().is_some())
                                class=("border-red-500", move || name_error().is_some())

                                type="text"
                                name="name"
                                required=true
                                placeholder="your name"

                                on:input=move |ev| {
                                    set_name_error(Name(event_target_value(&ev)).validate().err().map(|vaL| vaL.to_string()));
                                }
                            />
                            {move || name_error().map(|err| view! {
                                <p class="my-1 p-1 text-red-500">{err}</p>
                            })}
                        </label>
                    </div>

                    <SubmitButton value="Finish" disabled=disabled />
                </ActionForm>
            </div>
        </main>
    }
}

#[server]
async fn register(
    name: String,
) -> Result<Result<RegisterResponse, RegisterClientError>, ServerFnError> {
    use std::sync::Arc;

    use backend::session::{
        REGISTRATION_AVATAR_URI_KEY, REGISTRATION_EMAIL_KEY, REGISTRATION_KIND_KEY,
        REGISTRATION_PASSWORD_KEY,
    };
    use backend::state::ServerState;
    use garde::Unvalidated;
    use leptos_axum::*;
    use tower_sessions::Session;

    let state = expect_context::<Arc<ServerState>>();
    let session: Session = extract()
        .await
        .map_err(|_| ServerFnError::new("failed to extract session"))?;

    let Some(kind) = session
        .get(REGISTRATION_KIND_KEY)
        .await
        .map_err(|_| ServerFnError::new("session error"))?
    else {
        return Err(ServerFnError::new("REGISTRATION_KIND_KEY not found"));
    };

    let Some(email) = session
        .get(REGISTRATION_EMAIL_KEY)
        .await
        .map_err(|_| ServerFnError::new("session error"))?
    else {
        return Err(ServerFnError::new("REGISTRATION_EMAIL_KEY not found"));
    };

    let password = session
        .get(REGISTRATION_PASSWORD_KEY)
        .await
        .map_err(|_| ServerFnError::new("session error"))?;

    let avatar = session
        .get(REGISTRATION_AVATAR_URI_KEY)
        .await
        .map_err(|_| ServerFnError::new("session error"))?;

    let request = RegisterRequest {
        kind,
        email: Email(email),
        password: password,
        name: Name(name),
        avatar,
    };

    let request = Unvalidated::new(request)
        .validate()
        .map_err(|_| ServerFnError::new("validation failed"))?;

    Ok(backend::routes::auth::register::register(state, request)
        .await
        .map_err(|val| val.into()))
}
