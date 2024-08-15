use common::{
    entity::user::{Email, Password, MAX_USER_PASSWORD_SIZE},
    routes::auth::register_data::{
        RegisterDataClientError, RegisterDataRequest, RegisterDataResponse,
    },
};
use garde::Validate;
use leptos::{ev::Event, *};
use leptos_router::{ActionForm, A};
use tracing::error;

use crate::{
    atoms::submit_button::SubmitButton,
    components::{
        alert_message::{use_alert_message, MessageVariant},
        oauth2_links::OAuth2Links,
        or_break::OrBreak,
    },
    pages::app::APP_PAGE_URL,
    utils::error::wrap_action_value,
};

pub const REGISTRATION_PAGE_URL: &str = "/registration";

#[component]
pub fn Registration() -> impl IntoView {
    let alert = use_alert_message();
    let action = create_server_action::<GoToRegistrationDetails>();
    let action_value = action.value();

    // let action_value = create_memo(move |_| action_value());

    let email_node: NodeRef<html::Input> = create_node_ref();
    let password_node: NodeRef<html::Input> = create_node_ref();

    let (password, set_password) = create_signal("".to_owned());
    let (confirm, set_confirm) = create_signal("".to_owned());

    let (email_error, set_email_error) = create_signal(None);
    let (password_error, set_password_error) = create_signal(None);
    let confirm_error =
        move || with!(|password, confirm| validate_confirm(password, confirm).err());

    let disabled = Signal::derive(move || {
        email_error().is_some() || password_error().is_some() || confirm_error().is_some()
    });

    wrap_action_value(alert.clone(), action_value, move |val| match val {
        Ok(_) => (),
        Err(err) => match err {
            RegisterDataClientError::Other => {
                error!(
                    title = "Registration failed",
                    description = format!("{err:?}")
                );
                alert.create(
                    "Registration failed",
                    MessageVariant::Failure,
                    Default::default(),
                );
            }
        },
    });

    view! {
        <main class="
            absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
            max-w-xs w-full 
        ">
            <div class="mb-5 p-10 border rounded-xl shadow-md">
                <p class="mb-5 text-xl text-center">"Create a new account"</p>
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

                                node_ref=email_node
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
                                maxlength=MAX_USER_PASSWORD_SIZE
                                required=true
                                placeholder="your password"
                                autocomplete="new-password"

                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_password(value.clone());

                                    let err = Password(value).validate().err().map(|val| val.to_string());
                                    set_password_error(err);
                                }

                                node_ref=password_node
                            />
                            {move || password_error().map(|err| view! {
                                <p class="my-1 p-1 text-red-500">{err}</p>
                            })}
                        </label>

                        <label class="block">
                            <p class="mb-1">"Confirm password"</p>
                            <input
                                class="px-2 py-1 w-full border border-gray-400 rounded-md"
                                class=("border-2", move || confirm_error().is_some())
                                class=("border-red-500", move || confirm_error().is_some())

                                type="password"
                                name="confirm"
                                maxlength=MAX_USER_PASSWORD_SIZE
                                required=true
                                placeholder="repeat your password"
                                autocomplete="new-password"

                                on:input=move |ev: Event| set_confirm(event_target_value(&ev))
                            />
                            {move || confirm_error().map(|err| view! {
                                <p class="my-1 p-1 text-red-500">{err}</p>
                            })}
                        </label>
                    </div>

                    <SubmitButton value="Register" disabled=disabled />
                </ActionForm>

                <OrBreak/>
                <OAuth2Links/>
            </div>

            <div class="px-10 py-5 border rounded-xl shadow-md text-center">
                "Already registered? "<A href="/authentication" class="text-blue-500 hover:text-blue-300">"Log in."</A>
            </div>
        </main>
    }
}

fn validate_confirm(password: &str, confirm: &str) -> Result<(), &'static str> {
    if password != confirm {
        return Err("Passwords are not the same");
    }

    Ok(())
}

#[server]
async fn go_to_registration_details(
    email: String,
    password: String,
) -> Result<Result<RegisterDataResponse, RegisterDataClientError>, ServerFnError> {
    use garde::Unvalidated;
    use leptos_axum::*;
    use tower_sessions::Session;

    let session: Session = extract()
        .await
        .map_err(|_| ServerFnError::new("failed to extract session"))?;

    let request = Unvalidated::new(RegisterDataRequest {
        email: Email(email),
        password: Password(password),
    })
    .validate()
    .map_err(|_| ServerFnError::new("validation failed"))?;

    Ok(
        backend::routes::auth::register_data::register_data(session, request)
            .await
            .inspect(|_| redirect(APP_PAGE_URL))
            .map_err(|val| val.into()),
    )
}
