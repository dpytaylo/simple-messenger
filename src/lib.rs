use cfg_if::cfg_if;
use frontend::{ErrorTemplate, Frontend};
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
                    error="This is not the web page you are looking for.".into()
                />
            }
        }>
            <Frontend/>
        </Router>
    }
}

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use tracing_web::{MakeWebConsoleWriter, performance_layer};
    use tracing_subscriber::fmt::format::Pretty;
    use tracing_subscriber::prelude::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
        console_error_panic_hook::set_once();

        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false) // Only partially supported across browsers
            .without_time()   // std::time is not available in browsers
            .with_writer(MakeWebConsoleWriter::new()) // write events to the console
            .with_filter(tracing::level_filters::LevelFilter::DEBUG);

        let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(perf_layer)
            .init();

        leptos::mount_to_body(move || {
            view! { <App/> }
        });
    }
}}
