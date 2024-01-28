use common::entity::user::{MAX_USER_EMAIL_SIZE, MAX_USER_PASSWORD_SIZE};
use common::error::auth::AuthenticateClientError;
use leptos::*;
use leptos_router::{ActionForm, Params};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::auth::form_failed::FormFailed;

#[derive(Params, PartialEq)]
struct AuthenticationParams {
    error: String,
}

#[component]
pub fn Authentication() -> impl IntoView {
    let authenticate = create_server_action::<Authenticate>();

    let rw = authenticate.value();
    let is_error = move || rw.with(|val| val.as_ref().map(|val| val.is_err()).unwrap_or(false));

    view! {
        <main class="
            absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
            max-w-xs w-full
        ">
            <div class="mb-5 p-10 border rounded-xl shadow-md">
                <p class="mb-2 text-xl text-center">"Welcome!"</p>
                <FormFailed action_value=authenticate.value() />

                <ActionForm action=authenticate>
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
                                class=("border-2", move || is_error())
                                class=("border-red-500", move || is_error())
                            />
                        </label>

                        <label class="block">
                            <p class="mb-1">"Password"</p>
                            <input
                                type="password"
                                name="password"
                                maxlength=MAX_USER_PASSWORD_SIZE
                                required=true
                                placeholder="your password"
                                autocomplete="current-password"
                                class="px-2 py-1 w-full border border-gray-400 rounded-md"
                                class=("border-2", move || is_error())
                                class=("border-red-500", move || is_error())
                            />
                        </label>
                    </div>

                    <input
                        type="submit"
                        value="Log In"
                        class="
                            py-1 w-full h-9 rounded-md hover:cursor-pointer text-white 
                            bg-blue-500 hover:bg-blue-600
                        "
                    />
                </ActionForm>

                <div class="inline-flex items-center justify-center w-full">
                    <hr class="w-full h-px my-8 bg-gray-200 border-0" />
                    <span class="absolute px-3 font-medium text-gray-900 -translate-x-1/2 bg-white left-1/2">"or"</span>
                </div>

                <a href="/api/auth/google">
                    <img class="mx-auto w-10 h-10 p-1 hover:bg-slate-100 rounded" src="/assets/google_logo.svg" />
                </a>

                <div class="inline-flex items-center justify-center w-full">
                    <hr class="w-full h-px my-8 bg-gray-200 border-0" />
                    <span class="absolute px-3 font-medium text-gray-900 -translate-x-1/2 bg-white left-1/2">"or"</span>
                </div>

                <p class="text-center text-sm text-blue-500 hover:text-blue-300">
                    <a href="/registration">"Create a new account"</a>
                </p>
            </div>
        </main>
    }
}

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum AuthenticateSFnError {
    #[error("extraction error")]
    ExtractionFailed,

    #[error("validation error")]
    Validation,

    #[error(transparent)]
    AuthenticateClient(#[from] AuthenticateClientError),
}

#[server]
async fn authenticate(
    email: String,
    password: String,
) -> Result<Result<(), AuthenticateSFnError>, ServerFnError> {
    Ok(authenticate_inner(email, password).await)
}

#[cfg(feature = "ssr")]
async fn authenticate_inner(email: String, password: String) -> Result<(), AuthenticateSFnError> {
    use backend::{
        auth::authenticate::{self, AuthorizatePayload},
        state::ServerState,
    };
    use common::entity::user::{Email, Password};
    use garde::Validate;
    use leptos_axum::extract;
    use tower_cookies::Cookies;

    use crate::error::ExtractionError;

    let state: ServerState = expect_context::<ServerState>();
    let cookies: Cookies = extract::<_, ExtractionError>()
        .await
        .map_err(|_| AuthenticateSFnError::ExtractionFailed)?;

    let payload = AuthorizatePayload {
        email: Email(email),
        password: Password(password),
    };

    // TODO validation
    payload
        .validate(&())
        .map_err(|_| AuthenticateSFnError::Validation)?;

    authenticate::authenticate(state, cookies, payload)
        .await
        .map_err(AuthenticateClientError::from)?;
    Ok(())
}
