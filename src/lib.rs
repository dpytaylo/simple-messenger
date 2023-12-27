use cfg_if::cfg_if;
use frontend::{error_template::ErrorTemplate, Frontend};
use http::StatusCode;
use leptos::{component, view, IntoView};
use leptos_meta::*;
use leptos_router::Router;

#[cfg(feature = "ssr")]
pub mod server;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/simple-messenger.css"/>
        <Title text="Simple Messenger"/>

        <Link rel="preload" href="/assets/fonts/Inter/Inter.woff2" as_="font" crossorigin="anonymous" />
        <Style>
            "
            @font-face {
                font-family: 'Inter';
                src: local('Inter'), url('/assets/fonts/Inter/Inter.woff2');
            }
            "
        </Style>

        <Router fallback=|| {
            view! {
                <ErrorTemplate
                    code=StatusCode::NOT_FOUND
                    error="This is not the web page you are looking for.".to_owned()
                />
            }
        }>
            <Frontend/>
        </Router>
    }
}

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();

        leptos::mount_to_body(move || {
            view! { <App/> }
        });
    }
}}
