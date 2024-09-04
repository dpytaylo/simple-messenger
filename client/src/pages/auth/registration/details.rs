use backend_api::{
    entities::user::Name,
    routes::auth::registration::is_name_available::{IsNameAvailable, IsNameAvailableRequest},
};
use garde::{Unvalidated, Validate};
use leptos::*;
use tracing::error;

use crate::{
    atoms::button::{Button, ButtonKind},
    components::alert_message::{use_alert_message, MessageVariant},
    utils::{error::log_rpc_error, rpc_provider::use_rpc_client},
};

#[component]
pub fn Details<NF>(
    #[prop(optional)] back_step: Option<Box<dyn Fn()>>,
    next_step: NF,
    name: RwSignal<Option<Name>>,
) -> impl IntoView
where
    NF: Fn() + 'static,
{
    let alert = use_alert_message();
    let name_node: NodeRef<html::Input> = create_node_ref();

    let back_button = match back_step {
        Some(back_step) => view! {
            <Button
                kind=ButtonKind::Secondary
                class="mt-4 min-[500px]:mt-0 w-full min-[500px]:w-28 h-12 min-[500px]:h-10"
                on:click=move |_| back_step()
            >
                "Return back"
            </Button>
        }
        .into_view(),
        None => view! { <div /> }.into_view(),
    };

    let (name_error, set_name_error) = create_signal(None);
    let disabled = Signal::derive(move || name_error().is_some());

    let is_name_available = create_action(move |input: &IsNameAvailableRequest| {
        let rpc_client = use_rpc_client();
        let input = input.clone();

        async move { rpc_client.call::<IsNameAvailable>(&input).await }
    });

    let is_name_available_value = is_name_available.value();

    let on_continue = move |_| {
        let name_value = name_node.get().unwrap().value();

        let name_value = match Unvalidated::new(Name(name_value)).validate() {
            Ok(val) => val,
            Err(err) => {
                set_name_error(Some(err.to_string()));
                return;
            }
        };

        name.set(Some(name_value.clone().into_inner()));
        let request = IsNameAvailableRequest {
            name: name_value.into_inner(),
        };

        is_name_available.dispatch(request);
    };

    create_effect(move |_| {
        let Some(rpc_result) = is_name_available_value() else {
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
    });

    view! {
        <div class="w-full lg:h-lvh bg-white lg:bg-slate-100">
            <div class="mx-auto mt-20 lg:mt-0 lg:mb-20 lg:relative lg:top-9/20 lg:-translate-y-1/2 max-w-screen-lg w-full px-4 sm:px-12 lg:py-16 rounded-xl bg-white">
                <div class="lg:grid lg:grid-cols-2 lg:gap-x-12">
                    <div>
                        <p class="text-4xl lg:text-5xl">"Choose a nickname"</p>
                        <p class="mt-4">"Your friend can find you via your nickname."</p>
                        <p class="mt-2">"You will be able to change it later in the settings."</p>
                    </div>
                    <div class="mt-10 lg:mt-0">
                        <label class="block">
                            <p>"Nickname"</p>
                            <input
                                class="mt-1 h-11 px-2 py-1 w-full border border-gray-400 rounded-md"
                                class=("border-2", move || name_error().is_some())
                                class=("border-red-500", move || name_error().is_some())

                                type="text"
                                name="name"
                                required=true
                                placeholder="your nickname"

                                attr:value=name.get_untracked().map(|val| val.0).unwrap_or_default()

                                on:input=move |ev| {
                                    set_name_error(Name(event_target_value(&ev)).validate().err().map(|vaL| vaL.to_string()));
                                }

                                _ref = name_node
                            />
                            <p class="my-1 p-1 h-8 text-red-500 text-sm">
                                {name_error}
                            </p>
                        </label>
                    </div>
                </div>

                <div class="mt-16 sm:mt-32 flex flex-col-reverse items-stretch min-[500px]:flex-row min-[500px]:justify-between">
                    {back_button}
                    <Button
                        kind=ButtonKind::Primary
                        disabled=disabled
                        class="w-full min-[500px]:w-28 h-12 min-[500px]:h-10"
                        on:click=on_continue
                    >
                        "Continue"
                    </Button>
                </div>
            </div>
        </div>
    }
}
