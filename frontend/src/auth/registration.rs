use common::MAX_USER_PASSWORD_SIZE;
use leptos::{ev::Event, *};
use leptos_router::{ActionForm, A};
use tracing::error;
use validator::Validate;

use crate::validation;

#[component]
pub fn Registration() -> impl IntoView {
    let next_step_action = create_server_action::<GoToRegistrationDetailsStep>();

    let (password, set_password) = create_signal("".to_owned());
    let (confirm, set_confirm) = create_signal("".to_owned());

    let next_step_result = next_step_action.value();
    let error_msg = move || {
        let Some(value) = next_step_result() else {
            return None;
        };
        let err = value.expect_err("redirection");

        let msg = match err {
            ServerFnError::ServerError(val) => val,
            other => {
                error!(description = ?other);
                return None;
            }
        };

        Some(view! {
            <p class="p-1 mb-5 bg-red-400 border-red-500 rounded-md text-sm text-white">
                "Error(s):"<br/>
                {msg}
            </p>
        })
    };

    let (email_error, set_email_error) = create_signal(None);
    let (password_error, set_password_error) = create_signal(None);
    let confirm_error =
        move || with!(|password, confirm| validate_confirm(password, confirm).err());

    view! {
        <div class="
            absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
            max-w-xs w-full 
        ">
            <div class="mb-5 p-10 border rounded-xl shadow-md">
                <p class="mb-5 text-xl text-center">"Create a new account"</p>
                {error_msg}
                <ActionForm action=next_step_action>
                    <div class="mb-5 space-y-4 text-sm">
                        <label class="block">
                            <p class="mb-1">"Email"</p>
                            <input
                                type="text"
                                name="email"
                                required=true
                                placeholder="your email"
                                class="px-2 py-1 w-full border border-gray-400 rounded-md"
                                autocomplete="email"
                                on:input=move |ev| set_email_error(validate_email(event_target_value(&ev)).err())
                                class=("border-2", move || email_error().is_some())
                                class=("border-red-500", move || email_error().is_some())
                            />
                            {move || email_error().map(|err: String| view! {
                                <p class="my-1 p-1 text-red-500">{err}</p>
                            })}
                        </label>

                        <label class="block">
                            <p class="mb-1">"Password"</p>
                            <input
                                type="password"
                                name="password"
                                // maxlength(password length) in bytes could be greater than MAX_USER_PASSWORD_SIZE
                                maxlength=MAX_USER_PASSWORD_SIZE
                                required=true
                                placeholder="your password"
                                autocomplete="new-password"
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_password(value.clone());
                                    set_password_error(validate_password(&value).err());
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
                                // maxlength(password length) in bytes could be greater than MAX_USER_PASSWORD_SIZE
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

                <a href="/api/auth/oauth/google" rel="external">
                    <img class="mx-auto w-10 h-10 p-1 hover:bg-slate-100 rounded" src="/assets/google_logo.svg" />
                </a>
            </div>

            <div class="px-10 py-5 border rounded-xl shadow-md text-center text-sm">
                "Already registered? "<A href="/authentication" class="text-blue-500 hover:text-blue-300">"Log in."</A>
            </div>
        </div>
    }
}

fn validate_email(email: String) -> Result<(), String> {
    #[derive(Validate)]
    struct ValidateEmail {
        #[validate(email(
            message = "Invalid email, only supports emails based on the HTML5 spec."
        ))]
        email: String,
    }

    let email = ValidateEmail { email };
    email.validate().map_err(|err| {
        let mut errors = validation::flatten(err);
        errors.sort();

        let mut buffer = String::with_capacity(errors.len());
        for error in errors {
            buffer.push_str(&format!(
                "{}: {}\n",
                error.code,
                error.message.unwrap_or_else(|| "None".into())
            ));
        }

        buffer
    })
}

fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.is_empty() {
        return Err("The password field should not be empty");
    } else if password.len() > MAX_USER_PASSWORD_SIZE {
        return Err("Shorten the password.");
    }

    Ok(())
}

fn validate_confirm(password: &str, confirm: &str) -> Result<(), &'static str> {
    if password != confirm {
        return Err("Passwords are not the same.");
    }

    Ok(())
}

#[server]
async fn go_to_registration_details_step(
    email: String,
    password: String,
    confirm: String,
) -> Result<(), ServerFnError> {
    use backend::cookies::{
        self, REGISTRATION_EMAIL_TOKEN, REGISTRATION_PASSWORD_TOKEN, REGISTRATION_TYPE_TOKEN,
    };
    use backend::INTERNAL_SERVER_ERROR_STR;
    use leptos_axum::extract;
    use service::RegistrationType;
    use tower_cookies::Cookies;

    validate_confirm(&password, &confirm).map_err(|err| ServerFnError::ServerError(err.into()))?;

    if let Err(_) = extract(|cookies: Cookies| async move {
        cookies.add(cookies::create_secure_cookie(
            REGISTRATION_EMAIL_TOKEN,
            email,
        ));
        cookies.add(cookies::create_secure_cookie(
            REGISTRATION_TYPE_TOKEN,
            RegistrationType::Email.to_string(),
        ));
        cookies.add(cookies::create_secure_cookie(
            REGISTRATION_PASSWORD_TOKEN,
            password,
        ));
    })
    .await
    {
        error!(description = "Failed to extract");
        return Err(ServerFnError::ServerError(INTERNAL_SERVER_ERROR_STR.into()));
    }

    leptos_axum::redirect("/registration_details");
    Ok(())
}
