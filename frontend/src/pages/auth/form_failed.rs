use leptos::*;

#[component]
pub fn FormFailed(#[prop(optional, into)] value: MaybeSignal<Option<String>>) -> impl IntoView {
    view! {
        {move || {
            let value = value()?;

            Some(view! {
                <div class="
                    p-1 mb-5 bg-red-400 border-red-500 rounded-md
                    text-sm text-white break-words
                ">
                    <p>"Error(s):"</p>
                    <p>{value}</p>
                </div>
            })
        }}
    }
}
