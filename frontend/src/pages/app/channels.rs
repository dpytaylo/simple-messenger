use leptos::*;

#[component]
pub fn Channels() -> impl IntoView {
    view! {
        <div class="w-full h-full flex flex-col justify-start bg-slate-300">
            <div class="w-full h-20 px-2 flex justify-between items-center">
                <div class="flex items-center gap-2">
                    <div class="w-[60px] h-[60px] bg-slate-200" />
                    <div class="h-min">
                        <p class="font-medium">"Username"</p>
                        <p class="text-slate-500">"Hello, are you OK?"</p>
                    </div>
                </div>
                <div class="w-2 h-2 bg-slate-100" />
            </div>
        </div>
    }
}
