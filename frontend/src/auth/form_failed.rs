use leptos::*;
use tracing::error;

#[component]
pub fn FormFailed(action_value: RwSignal<Option<Result<(), ServerFnError>>>) -> impl IntoView {
    view! {
        {move || {
            let Some(value) = action_value() else {
                return None;
            };
            let err = value.expect_err("redirection");
    
            let msg = match err {
                ServerFnError::ServerError(val) => val,
                other => {
                    error!(description = ?other);
                    return None;
                }
            };
    
            Some(view! {
                <p class="p-1 mb-5 bg-red-400 border-red-500 rounded-md text-sm text-white">
                    "Error(s):"<br/>
                    {msg}
                </p>
            })
        }}
    }
}
