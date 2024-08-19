use common::{
    entity::user::Email,
    routes::auth::registration::is_email_available::{IsEmailAvailable, IsEmailAvailableRequest},
};
use ev::SubmitEvent;
use garde::Validate;
use leptos::*;
use leptos_router::A;
use tracing::error;

use crate::{
    atoms::submit_button::SubmitButton,
    components::{
        alert_message::{use_alert_message, MessageVariant},
        oauth2_links::OAuth2Links,
        or_break::OrBreak,
    },
    utils::{error::log_rpc_error, rpc_provider::use_rpc_client},
};

#[component]
pub fn EmailPage<F>(next_step: F, set_email: WriteSignal<Option<Email>>) -> impl IntoView
where
    F: Fn() + Clone + 'static,
{
    let email_node: NodeRef<html::Input> = create_node_ref();

    let (email_error, set_email_error) = create_signal(None);
    let disabled = Signal::derive(move || email_error().is_some());

    let is_email_available = create_action(move |input: &IsEmailAvailableRequest| {
        let input = input.clone();

        let alert = use_alert_message();
        let rpc_client = use_rpc_client();
        let next_step = next_step.clone();

        async move {
            let rpc_result = rpc_client.call::<IsEmailAvailable>(&input).await;

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
                        set_email(Some(input.email));
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
        }
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        is_email_available.dispatch(IsEmailAvailableRequest {
            email: Email(email_node.get().unwrap().value()),
        });
    };

    view! {
        <div class="mb-5 p-10 border rounded-xl shadow-md">
            <p class="mb-5 text-xl text-center">"Create a new account"</p>
            <form on:submit=on_submit>
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
                </div>

                <SubmitButton value="Next" disabled=disabled />
            </form>

            <OrBreak/>
            <OAuth2Links/>
        </div>

        <div class="px-10 py-5 border rounded-xl shadow-md text-center">
            "Already registered? "<A href="/authentication" class="text-blue-500 hover:text-blue-300">"Log in."</A>
        </div>
    }
}
