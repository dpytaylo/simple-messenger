use leptos::*;
use leptos_router::A;

use crate::{
    components::alert_message::{use_alert_message, MessageVariant},
    pages::auth::{registration::sign_up::SIGN_UP_PAGE_URL, sign_in::SIGN_IN_PAGE_URL},
};

#[component]
pub fn Root() -> impl IntoView {
    let alert = use_alert_message();

    let on_click = move |_| {
        alert.create("error", MessageVariant::Failure, Default::default());
    };

    view! {
        <header class="flex-shrink-0 sticky w-full top-0 left-0 h-11 bg-stone-800 backdrop-blur border-b border-gray-200 z-10">
            <div class="px-5 py-[6px] h-full flex flex-wrap justify-end">
                <A href=SIGN_IN_PAGE_URL class="
                    flex items-center justify-center
                    h-full px-4 bg-blue-500
                    hover:bg-blue-600 hover:cursor-pointer
                    rounded-md
                ">
                    <span class="text-white text-sm">
                        "Sign In"
                    </span>
                </A>
            </div>
        </header>

        <main>
            <div class="mx-auto mt-48 mb-10 w-fit text-5xl font-bold">
                <p class="mb-2">"A "<span class="italic tracking-wide">"new"</span>" messenger."</p>
                <p>
                    "New technologies."
                </p>
            </div>

            <A href=SIGN_UP_PAGE_URL class="
                mx-auto py-4 w-60
                flex items-center justify-center
                h-full px-4 bg-lime-500
                hover:bg-lime-600 hover:cursor-pointer
                rounded-md
            ">
                <span class="text-white text-3xl">
                    "Try it now!"
                </span>
            </A>

            <button on:click=on_click>
                "show error"
            </button>
        </main>
    }
}
