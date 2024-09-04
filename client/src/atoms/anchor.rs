use leptos::*;
use leptos_router::A;

#[component]
pub fn Anchor(
    #[prop(into)] href: String,
    #[prop(optional, into)] alt: Option<String>,
    #[prop(default = MaybeSignal::Static(Default::default()), into)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || {
        format!(
            "text-blue-500 hover:cursor-pointer hover:text-blue-400 {}",
            class()
        )
    };

    view! {
        <A {..attributes} href=href class=class attr:alt=alt>
            {children.map(|val| val())}
        </A>
    }
}
