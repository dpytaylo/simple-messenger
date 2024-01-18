use common::entity::user::MAX_USER_NAME_SIZE;
use leptos::*;
use leptos_router::ActionForm;

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

#[server]
async fn register(name: String) -> Result<(), ServerFnError> {
    use api_error_derive::ApiErrorData;
    use axum::extract::State;
    use backend::cookies::{REGISTRATION_EMAIL, REGISTRATION_PASSWORD, REGISTRATION_TYPE};
    use backend::{
        auth::register::{self, RegisterPayload},
        state::ServerState,
    };
    use leptos_axum::extract_with_state;
    use service::RegistrationType;
    use tower_cookies::Cookies;

    use crate::error::extraction_error;

    let state: ServerState =
        use_context::<ServerState>().ok_or(ServerFnError::ServerError("No server state".into()))?;

    match extract_with_state(
        state,
        |State(state): State<ServerState>, cookies: Cookies| async move {
            let Some(kind) = cookies.get(REGISTRATION_TYPE) else {
                leptos_axum::redirect("/registration");
                return Ok(());
            };

            match RegistrationType::from_str(&kind.to_string())
                .map_err(|_| leptos_axum::redirect("/registration"))?
            {
                RegistrationType::Email => todo!(),
                RegistrationType::Discord | RegistrationType::Google => {
                    cookies.get(REGISTRATION_EMAIL);
                }
            }

            cookies.get(REGISTRATION_PASSWORD);

            let payload = RegisterPayload {
                email,
                password,
                name,
            };

            if let Err(err) = register::register(state, cookies, payload).await {
                let api_error: ApiErrorData = err.into();
                return Err(api_error.client_description);
            }

            Ok(())
        },
    )
    .await
    .map_err(|err| extraction_error(err))
    {
        Ok(()) => {
            leptos_axum::redirect("/");
            Ok(())
        }
        Err(err) => Err(ServerFnError::ServerError(err)),
    }
}
