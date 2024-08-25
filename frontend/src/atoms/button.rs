use leptos::*;

#[derive(Debug, Default, Clone)]
pub enum ButtonKind {
    Primary,
    #[default]
    Secondary,
    Transparent,
}

#[component]
pub fn Button(
    #[prop(optional, into)] kind: MaybeSignal<ButtonKind>,
    #[prop(default = MaybeSignal::Static(false), into)] disabled: MaybeSignal<bool>,
    #[prop(default = MaybeSignal::Static(Default::default()), into)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = move || {
        let default = concat!(
            "py-1 rounded-md ",
            "hover:cursor-pointer enabled:hover:cursor-pointer ",
            "disabled:bg-zinc-300 disabled:hover:cursor-default ",
        );

        let kind_classes = match kind() {
            ButtonKind::Primary => "text-white enabled:bg-blue-500 enabled:hover:bg-blue-600 ",
            ButtonKind::Secondary => "text-white enabled:bg-gray-500 enabled:hover:bg-gray-600 ",
            ButtonKind::Transparent => {
                "text-blue-500 enabled:bg-transparent enabled:hover:text-blue-400 "
            }
        };

        format!("{default}{kind_classes}{}", class())
    };

    view! {
        <button {..attributes} class=class disabled=disabled>
            {children()}
        </button>
    }
}
