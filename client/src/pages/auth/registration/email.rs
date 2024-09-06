use backend_api::{
    entities::email::Email,
    routes::auth::registration::is_email_available::{IsEmailAvailable, IsEmailAvailableRequest},
};
use leptos::*;
use tracing::error;

use crate::{
    atoms::{
        anchor::Anchor,
        button::{Button, ButtonKind},
    },
    components::{
        alert_message::{use_alert_message, MessageVariant},
        oauth2_links::OAuth2Links,
    },
    pages::auth::sign_in::SIGN_IN_PAGE_URL,
    utils::{error::log_rpc_error, rpc_provider::use_rpc_client},
};

#[component]
pub fn EmailPage<F>(next_step: F, email: RwSignal<Option<Email>>) -> impl IntoView
where
    F: Fn() + Clone + 'static,
{
    let alert = use_alert_message();

    let (email_error, set_email_error) = create_signal(None);
    let disabled = Signal::derive(move || email_error().is_some());

    let rpc_client = use_rpc_client();
    let is_email_available = create_action(move |input: &IsEmailAvailableRequest| {
        let rpc_client = rpc_client.clone();
        let input = input.clone();

        async move { rpc_client.call::<IsEmailAvailable>(&input).await }
    });

    let is_email_available_value = is_email_available.value();

    let on_continue = move |_| {
        let request = IsEmailAvailableRequest {
            email: email.get_untracked().unwrap(),
        };
        is_email_available.dispatch(request);
    };

    create_effect(move |_| {
        let Some(rpc_result) = is_email_available_value.get() else {
            return;
        };

        let result = match rpc_result {
            Ok(val) => val,
            Err(err) => {
                log_rpc_error(err);
                return;
            }
        };

        match result {
            Ok(val) => match val.is_available {
                true => {
                    next_step();
                }
                false => {
                    set_email_error(Some("This email is already taken".to_owned()));
                }
            },
            Err(err) => {
                error!(description = format!("{err:?}"), "Registration failed");

                alert.create(
                    "Registration failed",
                    MessageVariant::Failure,
                    Default::default(),
                );
            }
        }
    });

    view! {
        <div class="w-full lg:h-lvh bg-white lg:bg-slate-100">
            <div class="mx-auto mt-20 lg:mt-0 lg:mb-20 lg:relative lg:top-9/20 lg:-translate-y-1/2 max-w-screen-lg w-full px-4 sm:px-12 lg:py-16 rounded-xl bg-white">
                <div class="lg:grid lg:grid-cols-2 lg:gap-x-12">
                    <div>
                        <p class="text-4xl lg:text-5xl">"Create a new account"</p>
                        <p class="mt-4">"Register via email or one of the supported OAuth2 services."</p>
                    </div>
                    <div class="mt-10 lg:mt-0">
                        <div>
                            <label class="block">
                                <p>"Via email"</p>
                                <input
                                    class="mt-1 h-11 px-2 py-1 w-full border border-gray-400 rounded-md"
                                    class=("border-2", move || email_error().is_some())
                                    class=("border-red-500", move || email_error().is_some())

                                    type="text"
                                    name="email"
                                    required=true
                                    placeholder="your email"
                                    autocomplete="email"

                                    attr:value=email.get_untracked().map(|val| val.into_raw()).unwrap_or_default()

                                    on:input=move |ev| {
                                        match Email::new(event_target_value(&ev)) {
                                            Ok(val) => {
                                                email.set(Some(val));
                                                set_email_error(None);
                                            }
                                            Err(err) => {
                                                set_email_error(Some(err.to_string()));
                                            }
                                        }
                                    }
                                />
                                <p class="my-1 p-1 h-8 text-red-500 text-sm">
                                    {email_error}
                                </p>
                            </label>
                        </div>

                        <p class="mt-1">"Via OAuth2 services"</p>
                        <OAuth2Links class="mt-1" />
                    </div>
                </div>
                <div class="mt-16 sm:mt-32 flex flex-col-reverse min-[500px]:flex-row min-[500px]:justify-between">
                    <div class="mt-4 min-[500px]:mt-0 flex flex-col items-stretch text-center">
                        <Anchor href=SIGN_IN_PAGE_URL>"I have already an account"</Anchor>
                    </div>
                    <Button
                        kind=ButtonKind::Primary
                        disabled=disabled
                        class="mt-4 min-[500px]:mt-0 w-full min-[500px]:w-28 h-12 min-[500px]:h-10"
                        on:click=on_continue
                    >
                        "Continue"
                    </Button>
                </div>
            </div>
        </div>
    }
}
