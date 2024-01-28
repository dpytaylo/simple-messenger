use std::borrow::Cow;

use common::entity::user::{Email, Password, MAX_USER_EMAIL_SIZE, MAX_USER_PASSWORD_SIZE};
use garde::Validate;
use leptos::{ev::Event, *};
use leptos_router::{ActionForm, A};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::form_failed::FormFailed;

#[component]
pub fn Registration() -> impl IntoView {
    let next_step_action = create_server_action::<GoToRegistrationDetailsStep>();

    let (password, set_password) = create_signal("".to_owned());
    let (confirm, set_confirm) = create_signal("".to_owned());

    let (email_error, set_email_error) = create_signal(None);
    let (password_error, set_password_error) = create_signal(None);
    let confirm_error =
        move || with!(|password, confirm| validate_confirm(password, confirm).err());

    view! {
        <main class="
            absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
            max-w-xs w-full 
        ">
            <div class="mb-5 p-10 border rounded-xl shadow-md">
                <p class="mb-5 text-xl text-center">"Create a new account"</p>
                <FormFailed action_value=next_step_action.value() />
                <ActionForm action=next_step_action>
                    <div class="mb-5 space-y-4 text-sm">
                        <label class="block">
                            <p class="mb-1">"Email"</p>
                            <input
                                type="text"
                                name="email"
                                // maxlength(email length) in bytes could be greater than MAX_USER_EMAIL_SIZE
                                maxlength=MAX_USER_EMAIL_SIZE
                                required=true
                                placeholder="your email"
                                class="px-2 py-1 w-full border border-gray-400 rounded-md"
                                autocomplete="email"
                                on:input=move |ev| {
                                    let err = Email(event_target_value(&ev)).validate(&()).err().map(|val| val.to_string());
                                    set_email_error(err);
                                }
                                class=("border-2", move || email_error().is_some())
                                class=("border-red-500", move || email_error().is_some())
                            />
                            {move || email_error().map(|err| view! {
                                <p class="my-1 p-1 text-red-500">{err}</p>
                            })}
                        </label>

                        <label class="block">
                            <p class="mb-1">"Password"</p>
                            <input
                                type="password"
                                name="password"
                                maxlength=MAX_USER_PASSWORD_SIZE
                                required=true
                                placeholder="your password"
                                autocomplete="new-password"
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_password(value.clone());

                                    let err = Password(value).validate(&()).err().map(|val| val.to_string());
                                    set_password_error(err);
                                }
                                class="px-2 py-1 w-full border border-gray-400 rounded-md"
                                class=("border-2", move || password_error().is_some())
                                class=("border-red-500", move || password_error().is_some())
                            />
                            {move || password_error().map(|err| view! {
                                <p class="my-1 p-1 text-red-500">{err}</p>
                            })}
                        </label>

                        <label class="block">
                            <p class="mb-1">"Confirm password"</p>
                            <input
                                type="password"
                                name="confirm"
                                maxlength=MAX_USER_PASSWORD_SIZE
                                required=true
                                placeholder="repeat your password"
                                autocomplete="new-password"
                                on:input=move |ev: Event| set_confirm(event_target_value(&ev))
                                class="px-2 py-1 w-full border border-gray-400 rounded-md"
                                class=("border-2", move || confirm_error().is_some())
                                class=("border-red-500", move || confirm_error().is_some())
                            />
                            {move || confirm_error().map(|err| view! {
                                <p class="my-1 p-1 text-red-500">{err}</p>
                            })}
                        </label>
                    </div>

                    <input
                        type="submit"
                        value="Register"
                        class="
                            py-1 w-full h-9 rounded-md hover:cursor-pointer text-white 
                            enabled:bg-blue-500 enabled:hover:bg-blue-600 enabled:hover:cursor-pointer
                            disabled:bg-zinc-300 disabled:hover:cursor-default
                        "
                        disabled=move || email_error().is_some() || password_error().is_some() || confirm_error().is_some()
                    />
                </ActionForm>

                <div class="inline-flex items-center justify-center w-full">
                    <hr class="w-full h-px my-8 bg-gray-200 border-0" />
                    <span class="absolute px-3 font-medium text-gray-900 -translate-x-1/2 bg-white left-1/2">"or"</span>
                </div>

                <div class="flex flex-row justify-center gap-3">
                    <a href="/api/auth/oauth/google" rel="external">
                        <img class="w-10 h-10 p-1 hover:bg-slate-100 rounded" src="/assets/google_logo.svg" />
                    </a>

                    <a href="/api/auth/oauth/discord" rel="external">
                        <img class="w-10 h-10 p-1 hover:bg-slate-100 rounded" src="/assets/discord_logo_blue.svg" />
                    </a>
                </div>
            </div>

            <div class="px-10 py-5 border rounded-xl shadow-md text-center text-sm">
                "Already registered? "<A href="/authentication" class="text-blue-500 hover:text-blue-300">"Log in."</A>
            </div>
        </main>
    }
}

fn validate_email(_email: String) -> Result<(), Cow<'static, str>> {
    // #[derive(Validate)]
    // struct ValidateEmail {
    //     #[validate(email(
    //         message = "Invalid email, only supports emails based on the HTML5 spec"
    //     ))]
    //     email: String,
    // }

    // let email = ValidateEmail { email };
    // email.validate().map_err(|err| {
    //     let mut errors = err.into_errors();
    //     match errors.remove("email").unwrap() {
    //         ValidationErrorsKind::Field(mut val) => val.remove(0).message.unwrap(),
    //         _ => unreachable!(),
    //     }
    // })

    Ok(())
}

fn validate_password(_password: &str) -> Result<(), &'static str> {
    // if password.is_empty() {
    //     return Err("The password field should not be empty");
    // } else if password.len() > MAX_USER_PASSWORD_SIZE {
    //     return Err("Shorten the password");
    // }

    Ok(())
}

fn validate_confirm(password: &str, confirm: &str) -> Result<(), &'static str> {
    if password != confirm {
        return Err("Passwords are not the same");
    }

    Ok(())
}

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum GoToRegistrationDetailsStepError {
    #[error("extraction error")]
    ExtractionFailed,

    #[error("password and confirm are not equal")]
    PasswordAndConfirmNotEqual,
}

#[server]
async fn go_to_registration_details_step(
    email: String,
    password: String,
    confirm: String,
) -> Result<Result<(), GoToRegistrationDetailsStepError>, ServerFnError> {
    Ok(go_to_registration_details_step_inner(email, password, confirm).await)
}

#[cfg(feature = "ssr")]
async fn go_to_registration_details_step_inner(
    email: String,
    password: String,
    confirm: String,
) -> Result<(), GoToRegistrationDetailsStepError> {
    use backend::cookies::{self, REGISTRATION_EMAIL, REGISTRATION_PASSWORD, REGISTRATION_TYPE};
    use backend::state::ServerState;
    use leptos_axum::extract;
    use service::RegistrationType;
    use tower_cookies::Cookies;

    use crate::error::ExtractionError;

    let cookies: Cookies = extract::<_, ExtractionError>()
        .await
        .map_err(|_| GoToRegistrationDetailsStepError::ExtractionFailed)?;

    validate_confirm(&password, &confirm)
        .map_err(|_| GoToRegistrationDetailsStepError::PasswordAndConfirmNotEqual)?;

    cookies.add(cookies::create_secure_cookie(
        REGISTRATION_TYPE,
        RegistrationType::Email.to_string(),
    ));

    cookies.add(cookies::create_secure_cookie(REGISTRATION_EMAIL, email));

    cookies.add(cookies::create_secure_cookie(
        REGISTRATION_PASSWORD,
        password,
    ));

    leptos_axum::redirect("/registration_details");
    Ok(())
}
