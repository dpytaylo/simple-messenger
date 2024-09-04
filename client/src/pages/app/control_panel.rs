use leptos::*;

#[component]
pub fn ControlPanel() -> impl IntoView {
    view! {
        <div class="h-20 px-2 flex justify-between items-center bg-slate-400">
            <div class="flex items-center gap-2">
                <div class="w-16 h-16 bg-slate-200" />
                <p class="font-medium">"Username"</p>
            </div>
            <div class="flex gap-2">
                <div class="w-10 h-10 bg-slate-200" />
                <div class="w-10 h-10 bg-slate-200" />
                <div class="w-10 h-10 bg-slate-200" />
            </div>
        </div>
    }
}
