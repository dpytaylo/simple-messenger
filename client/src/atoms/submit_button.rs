use leptos::*;

#[component]
pub fn SubmitButton(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
) -> impl IntoView {
    view! {
        <input
            type="submit"
            value=value
            class="
                py-1 w-full h-9 rounded-md hover:cursor-pointer text-white 
                enabled:bg-blue-500 enabled:hover:bg-blue-600 enabled:hover:cursor-pointer
                disabled:bg-zinc-300 disabled:hover:cursor-default
            "
            disabled=disabled
        />
    }
}
