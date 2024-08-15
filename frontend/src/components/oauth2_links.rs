use leptos::*;

#[component]
pub fn OAuth2Links() -> impl IntoView {
    view! {
        <div class="flex flex-row justify-center gap-3">
            <a href="/api/auth/oauth/google" rel="external">
                <img class="w-10 h-10 p-1 hover:bg-slate-100 rounded" src="/assets/google_logo.svg" />
            </a>

            <a href="/api/auth/oauth/discord" rel="external">
                <img class="w-10 h-10 p-1 hover:bg-slate-100 rounded" src="/assets/discord_logo_blue.svg" />
            </a>
        </div>
    }
}
