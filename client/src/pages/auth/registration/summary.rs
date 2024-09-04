use backend_api::entities::user::{Email, Name, Password};
use leptos::*;

use crate::atoms::button::{Button, ButtonKind};

#[component]
pub fn Summary<BF, NF>(
    back_step: BF,
    next_step: NF,
    email: Email,
    password: Password,
    name: Name,
) -> impl IntoView
where
    BF: Fn() + 'static,
    NF: Fn() + 'static,
{
    let on_submit = move |_| {
        next_step();
    };

    view! {
        <div class="w-full lg:h-lvh bg-white lg:bg-slate-100">
            <div class="mx-auto mt-20 lg:mt-0 lg:mb-20 lg:relative lg:top-9/20 lg:-translate-y-1/2 max-w-screen-lg w-full px-4 sm:px-12 lg:py-16 rounded-xl bg-white">
                <div class="lg:grid lg:grid-cols-2 lg:gap-x-12">
                    <div>
                        <p class="text-4xl lg:text-5xl">"Let’s summarize"</p>
                        <p class="mt-4">"Double check your data before finish."</p>
                    </div>
                    <div class="mt-10 lg:mt-0 space-y-4">
                        <label class="block">
                            <p>"Email"</p>
                            <input
                                class="mt-1 h-11 px-2 py-1 w-full border border-gray-400 rounded-md"
                                readonly=true
                                prop:value=email.0
                            />
                        </label>

                        <label class="block">
                            <p>"Password"</p>
                            <input
                                class="mt-1 h-11 px-2 py-1 w-full border border-gray-400 rounded-md"
                                readonly=true
                                type="password"
                                prop:value=password.0
                            />
                        </label>

                        <label class="block">
                            <p>"Name"</p>
                            <input
                                class="mt-1 h-11 px-2 py-1 w-full border border-gray-400 rounded-md"
                                readonly=true
                                prop:value=name.0
                            />
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
                        class="w-full min-[500px]:w-28 h-12 min-[500px]:h-10"
                        on:click=on_submit
                    >
                        "Register"
                    </Button>
                </div>
            </div>
        </div>
    }
}
