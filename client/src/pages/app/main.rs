use leptos::*;

use crate::pages::app::{chat::Chat, top::Top};

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <div class="w-full h-full flex flex-col">
            <Top/>
            <Chat/>
        </div>
    }
}
