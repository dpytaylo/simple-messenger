use leptos::*;
use leptos_router::{use_query, IntoParam, Params};
use tracing::error;

#[derive(Params, PartialEq)]
struct AuthenticationParams {
    error: String,
}

#[component]
pub fn Authentication() -> impl IntoView {
    let params = use_query::<AuthenticationParams>();
    let error_msg = move || {
        params.with(|params| match params {
            Ok(val) => Some(val.error.clone()),
            Err(err) => match err {
                leptos_router::ParamsError::MissingParam(_) => None,
                leptos_router::ParamsError::Params(err) => {
                    error!("Failed to deserialize AuthenticationParams: {err}");
                    None
                }
            },
        })
    };

    view! {
        <div class="
            absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 p-10
            max-w-xs w-full border rounded-xl shadow-md
        ">
            <p class="mb-2 text-xl text-center">"Welcome!"</p>

            <Show when=move || error_msg().is_some()>
                <div class="p-2 mb-2 bg-red-400 rounded text-gray-50">
                    <p class="text-base text-center">"Error!"</p>
                    <p class="text-sm break-words">{error_msg}</p>
                </div>
            </Show>

            <form>
                <div class="mb-5 space-y-2 text-sm">
                    <label class="block">
                        "Email"
                        <br/>
                        <input type="text" id="email" class="px-2 w-full h-7 border border-gray-400 rounded-sm" />
                    </label>

                    <label class="block">
                        "Password"
                        <br/>
                        <input type="password" id="password" class="px-2 w-full h-7 border border-gray-400 rounded-sm" />
                    </label>
                </div>

                <input
                    type="Submit"
                    value="Log in"
                    class="py-1 w-full h-9 text-slate-50 font-semibold bg-blue-400 border border-gray-400 rounded-sm"
                />
            </form>

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
    }
}

#[server]
async fn authenticate(email: String, password: String) -> Result<(), ServerFnError> {
    Ok(())
}

#[server]
async fn authenticate_google() -> Result<(), ServerFnError> {
    Ok(())
}
