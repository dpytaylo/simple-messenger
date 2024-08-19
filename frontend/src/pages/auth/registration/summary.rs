use common::entity::user::{Email, Name, Password};
use ev::SubmitEvent;
use leptos::*;

use crate::atoms::submit_button::SubmitButton;

#[component]
pub fn Summary<F>(next_step: F, email: Email, password: Password, name: Name) -> impl IntoView
where
    F: Fn() + 'static,
{
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        next_step();
    };

    view! {
        <main class="
            absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
            max-w-xs w-full 
        ">
            <div class="mb-5 p-10 border rounded-xl shadow-md">
                <p class="mb-5 text-xl text-center">"Create a new account"</p>
                <form on:submit=on_submit>
                    <div class="mb-5 space-y-4">
                        <label class="block">
                            <p class="mb-1 text-sm">"Email"</p>
                            <input
                                class="h-8 px-2 py-1 w-full border border-gray-400 rounded-md text-sm"
                                readonly=true
                                prop:value=email.0
                            />
                        </label>

                        <label class="block">
                            <p class="mb-1 text-sm">"Password"</p>
                            <input
                                class="h-8 px-2 py-1 w-full border border-gray-400 rounded-md text-sm"
                                readonly=true
                                type="password"
                                prop:value=password.0
                            />
                        </label>

                        <label class="block">
                            <p class="mb-1 text-sm">"Name"</p>
                            <input
                                class="px-2 py-1 w-full border border-gray-400 rounded-md text-sm"
                                readonly=true
                                prop:value=name.0
                            />
                        </label>
                    </div>

                    <SubmitButton value="Register" />
                </form>
            </div>
        </main>
    }
}
