use backend_api::entities::user::{Password, MAX_USER_PASSWORD_SIZE};
use garde::{Unvalidated, Validate};
use leptos::{ev::Event, *};

use crate::atoms::button::{Button, ButtonKind};

#[component]
pub fn PasswordPage<BF, NF>(
    back_step: BF,
    next_step: NF,
    password: RwSignal<Option<Password>>,
) -> impl IntoView
where
    BF: Fn() + 'static,
    NF: Fn() + 'static,
{
    let password_node: NodeRef<html::Input> = create_node_ref();

    let password_rw = password;
    let (password, set_password) = create_signal("".to_owned());
    let (confirm, set_confirm) = create_signal("".to_owned());

    let (password_error, set_password_error) = create_signal(None);
    let confirm_error =
        move || with!(|password, confirm| validate_confirm(password, confirm).err());

    let disabled = Signal::derive(move || password_error().is_some() || confirm_error().is_some());

    let on_continue = move |_| {
        let password_value = password_node.get().unwrap().value();

        let password_value = match Unvalidated::new(Password(password_value)).validate() {
            Ok(val) => val,
            Err(err) => {
                set_password_error(Some(err.to_string()));
                return;
            }
        };

        password_rw.set(Some(password_value.into_inner()));
        next_step();
    };

    view! {
        <div class="w-full lg:h-lvh bg-white lg:bg-slate-100">
            <div class="mx-auto mt-20 lg:mt-0 lg:mb-20 lg:relative lg:top-9/20 lg:-translate-y-1/2 max-w-screen-lg w-full px-4 sm:px-12 lg:py-16 rounded-xl bg-white">
                <div class="lg:grid lg:grid-cols-2 lg:gap-x-12">
                    <div>
                        <p class="text-4xl lg:text-5xl">"Create a strong password"</p>
                        <p class="mt-4">"For example, "<span class="font-mono">"`qwerty123`"</span>" is not a very good choice."</p>
                    </div>
                    <div class="mt-10 lg:mt-0">
                        <label class="block">
                            <p>"Password"</p>
                            <input
                                class="mt-1 h-11 px-2 py-1 w-full border border-gray-400 rounded-md"
                                class=("border-2", move || password_error().is_some())
                                class=("border-red-500", move || password_error().is_some())

                                type="password"
                                name="password"
                                maxlength=MAX_USER_PASSWORD_SIZE
                                required=true
                                placeholder="your password"
                                autocomplete="new-password"

                                attr:value=password_rw.get_untracked().map(|val| val.0).unwrap_or_default()

                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_password(value.clone());

                                    let err = Password(value).validate().err().map(|val| val.to_string());
                                    set_password_error(err);
                                }

                                node_ref=password_node
                            />
                            <p class="my-1 p-1 h-8 text-red-500 text-sm">
                                {password_error}
                            </p>
                        </label>

                        <label class="block">
                            <p>"Confirm password"</p>
                            <input
                                class="mt-1 h-11 px-2 py-1 w-full border border-gray-400 rounded-md"
                                class=("border-2", move || confirm_error().is_some())
                                class=("border-red-500", move || confirm_error().is_some())

                                type="password"
                                name="confirm"
                                maxlength=MAX_USER_PASSWORD_SIZE
                                required=true
                                placeholder="repeat your password"
                                autocomplete="new-password"

                                attr:value=password_rw.get_untracked().map(|val| val.0).unwrap_or_default()

                                on:input=move |ev: Event| set_confirm(event_target_value(&ev))
                            />
                            <p class="my-1 p-1 h-8 text-red-500 text-sm">
                                {confirm_error}
                            </p>
                        </label>
                    </div>
                </div>

                <div class="mt-16 sm:mt-32 flex flex-col-reverse items-stretch min-[500px]:flex-row min-[500px]:justify-between">
                    <Button
                        kind=ButtonKind::Secondary
                        class="mt-4 min-[500px]:mt-0 w-full min-[500px]:w-28 h-12 min-[500px]:h-10"
                        on:click=move |_| back_step()
                    >
                        "Return back"
                    </Button>
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

fn validate_confirm(password: &str, confirm: &str) -> Result<(), &'static str> {
    if password != confirm {
        return Err("Passwords are not the same");
    }

    Ok(())
}
