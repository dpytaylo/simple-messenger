use leptos::*;

#[component]
pub fn OrBreak() -> impl IntoView {
    view! {
        <div class="inline-flex items-center justify-center w-full">
            <hr class="w-full h-px my-8 bg-gray-200 border-0" />
            <span class="absolute px-3 font-medium text-gray-900 -translate-x-1/2 bg-white left-1/2">"or"</span>
        </div>
    }
}
