use common::{entity::user::MAX_USER_NAME_SIZE, error::auth::RegisterClientError};
use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::auth::form_failed::FormFailed;

#[component]
pub fn RegistrationDetails() -> impl IntoView {
    let register_action = create_server_action::<Register>();
    let (name_error, set_name_error) = create_signal(None);

    view! {
        <main class="
            absolute top-2/5 left-1/2 -translate-x-1/2 -translate-y-1/2
            max-w-xs w-full 
        ">
            <div class="mb-5 p-10 border rounded-xl shadow-md">
                <p class="mb-5 text-xl text-center">"Details"</p>
                <FormFailed action_value=register_action.value() />
                <ActionForm action=register_action>
                    <div class="mb-5 space-y-4 text-sm">
                        <label class="block">
                            <p class="mb-1">"Name"</p>
                            <input
                                type="text"
                                name="name"
                                // maxlength(name length) in bytes could be greater than MAX_USER_NAME_SIZE
                                maxlength=MAX_USER_NAME_SIZE
                                required=true
                                placeholder="your name"
                                class="px-2 py-1 w-full border border-gray-400 rounded-md"
                                on:input=move |ev| {
                                    set_name_error(validate_name(event_target_value(&ev)).err());
                                }
                                class=("border-2", move || name_error().is_some())
                                class=("border-red-500", move || name_error().is_some())
                            />
                            {move || name_error().map(|err| view! {
                                <p class="my-1 p-1 text-red-500">{err}</p>
                            })}
                        </label>
                    </div>

                    <input
                        type="submit"
                        value="Finish"
                        class="
                            py-1 w-full h-9 rounded-md hover:cursor-pointer text-white 
                            enabled:bg-blue-500 enabled:hover:bg-blue-600 enabled:hover:cursor-pointer
                            disabled:bg-zinc-300 disabled:hover:cursor-default
                        "
                        disabled=move || name_error().is_some()
                    />
                </ActionForm>
            </div>
        </main>
    }
}

fn validate_name(_name: String) -> Result<(), String> {
    // let validator = ValidName { name };
    // debug!("{:?}", validator.validate());

    // validator.validate().map_err(|err| {
    //     let mut errors = validation::flatten(err);
    //     errors.sort();

    //     // debug!("{:?}", &errors);

    //     let mut buffer = String::with_capacity(errors.len());
    //     for error in errors {
    //         buffer.push_str(&format!(
    //             "{}: {}\n",
    //             error.code,
    //             error.message.unwrap_or_else(|| "None".into())
    //         ));
    //     }

    //     buffer
    // })

    Ok(())
}

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum RegisterSFnError {
    #[error("extraction error")]
    ExtractionFailed,

    #[error("validation error")]
    Validation,

    #[error("invalid registration type")]
    InvalidRegistrationType,

    #[error(transparent)]
    RegisterClient(#[from] RegisterClientError),
}

#[server]
async fn register(name: String) -> Result<Result<(), RegisterSFnError>, ServerFnError> {
    Ok(register_inner(name).await)
}

#[cfg(feature = "ssr")]
async fn register_inner(name: String) -> Result<(), RegisterSFnError> {
    use std::str::FromStr;

    use backend::cookies::{REGISTRATION_EMAIL, REGISTRATION_PASSWORD, REGISTRATION_TYPE};
    use backend::{
        auth::register::{self, RegisterPayload},
        state::ServerState,
    };
    use common::entity::user::{Email, Name, Password};
    use garde::Validate;
    use leptos_axum::extract;
    use service::RegistrationType;
    use tower_cookies::Cookies;

    use crate::error::ExtractionError;

    let state: ServerState = expect_context::<ServerState>();
    let cookies: Cookies = extract::<_, ExtractionError>()
        .await
        .map_err(|_| RegisterSFnError::ExtractionFailed)?;

    let (Some(kind_cookie), Some(email_cookie)) = (
        cookies.get(REGISTRATION_TYPE),
        cookies.get(REGISTRATION_EMAIL),
    ) else {
        leptos_axum::redirect("/registration");
        return Ok(());
    };

    let kind = RegistrationType::from_str(&kind_cookie.value_trimmed())
        .map_err(|_| RegisterSFnError::InvalidRegistrationType)?;

    let password = match kind {
        RegistrationType::Email => {
            let Some(password) = cookies.get(REGISTRATION_PASSWORD) else {
                leptos_axum::redirect("/registration");
                return Ok(());
            };

            Some(Password(password.value_trimmed().into()))
        }
        _ => None,
    };

    let payload = RegisterPayload {
        kind,
        email: Email(email_cookie.value_trimmed().into()),
        password,
        name: Name(name),
    };

    payload
        .validate(&())
        .map_err(|_| RegisterSFnError::Validation)?;

    register::register(state, cookies, payload)
        .await
        .map_err(RegisterClientError::from)?;

    Ok(())
}
