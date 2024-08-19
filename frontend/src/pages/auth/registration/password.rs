use common::entity::user::{Password, MAX_USER_PASSWORD_SIZE};
use ev::SubmitEvent;
use garde::Validate;
use leptos::{ev::Event, *};

use crate::atoms::submit_button::SubmitButton;

#[component]
pub fn PasswordPage<F>(next_step: F, set_password: WriteSignal<Option<Password>>) -> impl IntoView
where
    F: Fn() + 'static,
{
    let password_node: NodeRef<html::Input> = create_node_ref();

    let set_password_global = set_password;
    let (password, set_password) = create_signal("".to_owned());
    let (confirm, set_confirm) = create_signal("".to_owned());

    let (password_error, set_password_error) = create_signal(None);
    let confirm_error =
        move || with!(|password, confirm| validate_confirm(password, confirm).err());

    let disabled = Signal::derive(move || password_error().is_some() || confirm_error().is_some());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_password_global(Some(Password(password_node.get().unwrap().value())));
        next_step();
    };

    view! {
        <div class="mb-5 p-10 border rounded-xl shadow-md">
            <p class="mb-5 text-xl text-center">"Create your password"</p>
            <form on:submit=on_submit>
                <div class="mb-5 space-y-4">
                    <label class="block">
                        <p class="mb-1 text-sm">"Password"</p>
                        <input
                            class="h-8 px-2 py-1 w-full border border-gray-400 rounded-md text-sm"
                            class=("border-2", move || password_error().is_some())
                            class=("border-red-500", move || password_error().is_some())

                            type="password"
                            name="password"
                            maxlength=MAX_USER_PASSWORD_SIZE
                            required=true
                            placeholder="your password"
                            autocomplete="new-password"

                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_password(value.clone());

                                let err = Password(value).validate().err().map(|val| val.to_string());
                                set_password_error(err);
                            }

                            node_ref=password_node
                        />
                        {move || password_error().map(|err| view! {
                            <p class="my-1 p-1 text-red-500">{err}</p>
                        })}
                    </label>

                    <label class="block">
                        <p class="mb-1">"Confirm password"</p>
                        <input
                            class="px-2 py-1 w-full border border-gray-400 rounded-md"
                            class=("border-2", move || confirm_error().is_some())
                            class=("border-red-500", move || confirm_error().is_some())

                            type="password"
                            name="confirm"
                            maxlength=MAX_USER_PASSWORD_SIZE
                            required=true
                            placeholder="repeat your password"
                            autocomplete="new-password"

                            on:input=move |ev: Event| set_confirm(event_target_value(&ev))
                        />
                        {move || confirm_error().map(|err| view! {
                            <p class="my-1 p-1 text-red-500">{err}</p>
                        })}
                    </label>
                </div>

                <SubmitButton value="Next" disabled=disabled />
            </form>
        </div>
    }
}

fn validate_confirm(password: &str, confirm: &str) -> Result<(), &'static str> {
    if password != confirm {
        return Err("Passwords are not the same");
    }

    Ok(())
}
