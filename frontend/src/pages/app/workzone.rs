use leptos::*;

use crate::pages::app::{channels::Channels, control_panel::ControlPanel};

#[component]
pub fn Workzone() -> impl IntoView {
    view! {
        <div class="flex-shrink-0 w-[400px] h-full flex flex-col">
            <Channels/>
            <ControlPanel/>
        </div>
    }
}
