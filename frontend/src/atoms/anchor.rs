use leptos::*;
use leptos_router::A;

#[component]
pub fn Anchor(
    #[prop(into)] href: String,
    #[prop(optional, into)] alt: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <A href=href class="text-blue-500 hover:text-blue-300" attr:alt=alt>
            {children.map(|val| val())}
        </A>
    }
}
