use common::entity::user::{MAX_USER_EMAIL_SIZE, MAX_USER_PASSWORD_SIZE};
use leptos::*;
use leptos_router::{ActionForm, Params};

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

#[server]
async fn authenticate(email: String, password: String) -> Result<(), ServerFnError> {
    use api_error_derive::ApiErrorData;
    use axum::extract::State;
    use backend::{
        auth::authenticate::{self, AuthorizatePayload},
        state::ServerState,
    };
    use leptos_axum::extractor_with_state;
    use tower_cookies::Cookies;

    use crate::error::extraction_error;

    let state: ServerState =
        use_context::<ServerState>().ok_or(ServerFnError::ServerError("No server state".into()))?;

    match extractor_with_state(
        state,
        |State(state): State<ServerState>, cookies: Cookies| async move {
            if let Err(err) =
                authenticate::authenticate(state, cookies, AuthorizatePayload { email, password })
                    .await
            {
                let api_error: ApiErrorData = err.into();
                return Err(api_error.client_description);
            }

            Ok(())
        },
    )
    .await
    .map_err(|err| extraction_error(err))?
    {
        Ok(()) => Ok(()),
        Err(err) => Err(ServerFnError::ServerError(err)),
    }
}
