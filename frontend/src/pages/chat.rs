use leptos::*;

#[component]
pub fn chat() -> impl IntoView {
    // let (loaded, set_loaded) = create_signal(false);

    // let (message_sending, set_message_sending) = create_signal(false);
    // let messages = create_resource(|| (), |_| ());

    // create_effect(move |_| {
    //     request_animation_frame(move || set_loaded(true));
    // });

    view! {
        <main class="pt-5 pb-10 h-full font-inter">
            <div class="mx-auto max-w-screen-lg h-full bg-white border border-gray-200 rounded-md">
                <div class="mx-auto w-full max-w-2xl h-full flex flex-col justify-end gap-5">
                    <div class="flex-auto h-0 flex flex-col justify-end">
                    </div>
                </div>
            </div>
        </main>
    }
}

#[component]
fn ChatStub() -> impl IntoView {
    view! {
        <div class="flex items-center animate-pulse">
            <div class="inline-block ml-4 mr-3 w-10 h-10 bg-slate-300 rounded-full" />
            <div class="px-5 py-5 w-5/12 bg-white border border-gray-200 rounded-xl space-y-1">
                <div class="h-3 bg-slate-300 rounded" />

                <div class="h-3" />

                <div class="grid grid-cols-3 gap-4">
                    <div class="h-3 bg-slate-300 rounded col-span-2"></div>
                    <div class="h-3 bg-slate-300 rounded col-span-1"></div>
                </div>

                <div class="h-3 bg-slate-300 rounded" />
                <div class="h-3 bg-slate-300 rounded" />
            </div>
        </div>

        <div class="flex items-center animate-pulse">
            <div class="inline-block ml-4 mr-3 w-10 h-10 bg-slate-300 rounded-full" />
            <div class="px-5 py-5 w-1/2 bg-white border border-gray-200 rounded-xl space-y-1">
                <div class="w-2/3 h-3 bg-slate-300 rounded" />

                <div class="h-3" />

                <div class="h-3 bg-slate-300 rounded" />
                <div class="h-3 bg-slate-300 rounded" />
                <div class="h-3 bg-slate-300 rounded" />
                <div class="w-1/3 h-3 bg-slate-300 rounded" />
            </div>
        </div>

        <div class="flex items-center animate-pulse">
            <div class="inline-block ml-4 mr-3 w-10 h-10 bg-slate-300 rounded-full" />
            <div class="px-5 py-5 w-5/12 bg-white border border-gray-200 rounded-xl space-y-1">
                <div class="h-3 bg-slate-300 rounded" />
                <div class="h-3 bg-slate-300 rounded" />
                <div class="h-3 bg-slate-300 rounded" />
                <div class="w-2/3 h-3 bg-slate-300 rounded" />
            </div>
        </div>
    }
}
