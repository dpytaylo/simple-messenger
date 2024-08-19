use common::{
    entity::user::Name,
    routes::auth::registration::is_name_available::{IsNameAvailable, IsNameAvailableRequest},
};
use ev::SubmitEvent;
use garde::Validate;
use leptos::*;
use tracing::error;

use crate::{
    atoms::submit_button::SubmitButton,
    components::alert_message::{use_alert_message, MessageVariant},
    utils::{error::log_rpc_error, rpc_provider::use_rpc_client},
};

#[component]
pub fn Details<F>(next_step: F, set_name: WriteSignal<Option<Name>>) -> impl IntoView
where
    F: Fn() + Clone + 'static,
{
    let name_node: NodeRef<html::Input> = create_node_ref();

    let (name_error, set_name_error) = create_signal(None);
    let disabled = Signal::derive(move || name_error().is_some());

    let is_name_available = create_action(move |input: &IsNameAvailableRequest| {
        let input = input.clone();

        let alert = use_alert_message();
        let rpc_client = use_rpc_client();
        let next_step = next_step.clone();

        async move {
            let rpc_result = rpc_client.call::<IsNameAvailable>(&input).await;

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
                        set_name(Some(input.name));
                        next_step();
                    }
                    false => {
                        set_name_error(Some("This name is already taken".to_owned()));
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

        is_name_available.dispatch(IsNameAvailableRequest {
            name: Name(name_node.get().unwrap().value()),
        });
    };

    view! {
        <div class="mb-5 p-10 border rounded-xl shadow-md">
            <p class="mb-5 text-xl text-center">"Details"</p>
            <form on:submit=on_submit>
                <div class="mb-5 space-y-4 text-sm">
                    <label class="block">
                        <p class="mb-1 text-sm">"Name"</p>
                        <input
                            class="h-8 px-2 py-1 w-full border border-gray-400 rounded-md text-sm"
                            class=("border-2", move || name_error().is_some())
                            class=("border-red-500", move || name_error().is_some())

                            type="text"
                            name="name"
                            required=true
                            placeholder="your name"

                            on:input=move |ev| {
                                set_name_error(Name(event_target_value(&ev)).validate().err().map(|vaL| vaL.to_string()));
                            }

                            _ref = name_node
                        />
                        {move || name_error().map(|err| view! {
                            <p class="my-1 p-1 text-red-500">{err}</p>
                        })}
                    </label>
                </div>

                <SubmitButton value="Finish" disabled=disabled />
            </form>
        </div>
    }
}
