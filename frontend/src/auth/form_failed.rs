use std::fmt::Debug;

use leptos::*;

#[component]
pub fn FormFailed<T: Clone + Debug + 'static>(
    action_value: RwSignal<Option<Result<Result<(), T>, ServerFnError>>>,
) -> impl IntoView {
    view! {
        {move || {
            let value = action_value()?;

            let error = match value {
                Ok(Ok(())) => return None,
                Ok(Err(err)) => format!("{err:?}"),
                Err(err) => format!("{err:?}"),
            };

            Some(view! {
                <p class="
                    p-1 mb-5 bg-red-400 border-red-500 rounded-md
                    text-sm text-white break-words
                ">
                    "Error(s):"<br/>
                    {error}
                </p>
            })
        }}
    }
}
