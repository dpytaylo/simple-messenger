use common::MAX_USER_NAME_SIZE;
use leptos::*;
use leptos_router::ActionForm;
use tracing::debug;
use validator::Validate;

use crate::validation;

#[derive(Validate)]
struct ValidName {
    #[validate(length(min = 1, max = "MAX_USER_NAME_SIZE"))]
    name: String,
}

#[component]
pub fn RegistrationDetails() -> impl IntoView {
    let register_action = create_server_action::<Register>();

    let (name_error, set_name_error) = create_signal(None);

    let name_error_msg = move || {
        name_error().map(|err: String| {
            view! {
                <p class="my-1 p-1 text-red-500">{err}</p>
            }
        })
    };

    view! {
        <div class="
            absolute top-2/5 left-1/2 -translate-x-1/2 -translate-y-1/2 p-10
            max-w-xs w-full border rounded-xl shadow-md
        ">
            <p class="mb-5 text-xl text-center">"Details"</p>
            <ActionForm action=register_action>
                <div class="mb-5 space-y-4 text-sm">
                    <label class="block">
                        <p class="mb-1">"Name"</p>
                        <input
                            type="text"
                            name="name"
                            required=true
                            placeholder="your name"
                            class="px-2 py-1 w-full border border-gray-400 rounded-md"
                            on:input=move |ev| {
                                set_name_error(validate_name(event_target_value(&ev)).err());
                            }
                            class=("border-2", move || name_error().is_some())
                            class=("border-red-500", move || name_error().is_some())
                        />
                        {name_error_msg}
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
    }
}

fn validate_name(name: String) -> Result<(), String> {
    let validator = ValidName { name };
    debug!("{:?}", validator.validate());

    validator.validate().map_err(|err| {
        let mut errors = validation::flatten(err);
        errors.sort();

        // debug!("{:?}", &errors);

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

#[server]
async fn register(name: String) -> Result<(), ServerFnError> {
    use backend::INTERNAL_SERVER_ERROR_STR;
    use leptos_axum::extract;
    use tower_cookies::Cookies;
    use tracing::error;

    if let Err(_) = extract(|cookies: Cookies| async move {}).await {
        error!(description = "Failed to extract");
        return Err(ServerFnError::ServerError(INTERNAL_SERVER_ERROR_STR.into()));
    }

    leptos_axum::redirect("/");
    Ok(())
}
