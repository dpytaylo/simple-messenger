use leptos::*;

#[component]
pub fn Top() -> impl IntoView {
    view! {
        <div class="w-full h-[70px] pl-2 pr-4 flex justify-between items-center bg-white border">
            <div class="flex gap-2 items-center">
                <div class="w-[50px] h-[50px] bg-slate-100" />
                <div class="h-min">
                    <p class="font-medium">"Username"</p>
                    <p class="text-slate-500 text-sm">"Last seen recently"</p>
                </div>
            </div>
            <div class="flex gap-4">
                <button>
                    <img src="/assets/search_24dp_000000_FILL0_wght300_GRAD0_opsz24.svg" class="w-7 h-7" />
                </button>
                <button>
                    <img src="/assets/call_24dp_000000_FILL0_wght300_GRAD0_opsz24.svg" class="w-7 h-7" />
                </button>
                <button>
                    <img src="/assets/more_vert_24dp_000000_FILL0_wght300_GRAD0_opsz24.svg" class="w-7 h-7" />
                </button>
            </div>
        </div>
    }
}
