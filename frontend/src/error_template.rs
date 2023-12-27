use cfg_if::cfg_if;
use http::StatusCode;
use leptos::*;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;
use leptos_router::A;

#[component]
pub fn ErrorTemplate(code: StatusCode, #[prop(optional)] error: String) -> impl IntoView {
    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    cfg_if! { if #[cfg(feature="ssr")] {
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(code);
        }
    }}

    let error_code = code.as_u16();
    let code_str = code.canonical_reason().to_owned();

    let error =
        (!error.is_empty()).then(move || view! { <p class="text-xl mb-2 text-center">{error}</p> });

    view! {
        <div class="
            absolute top-2/5 left-1/2 -translate-x-1/2 -translate-y-1/2 p-10
            max-w-2xl
        ">
            <p class="text-4xl text-center">
                <span class="font-semibold">{error_code}</span>
                " "{code_str}" 🥲"
            </p>
            <hr class="w-full h-px my-4 bg-gray-200 border-0" />
            <div class="mx-4">
                {error}
                <p class="text-base text-center">
                    <A href="/" class="text-blue-500 hover:text-blue-300">
                        "Return to the main page."
                    </A>
                </p>
            </div>
        </div>
    }
}
