use leptos::*;

#[derive(Debug, Default, Clone)]
pub enum ButtonKind {
    #[default]
    Primary,
    Secondary,
}

#[component]
pub fn Button(
    #[prop(optional, into)] kind: MaybeSignal<ButtonKind>,
    children: Children,
) -> impl IntoView {
    let class = move || {
        let default = concat!(
            "py-1 w-full h-9 rounded-md hover:cursor-pointer text-white enabled:hover:cursor-pointer ",
            "disabled:bg-zinc-300 disabled:hover:cursor-default ",
        );

        let kind_classes = match kind() {
            ButtonKind::Primary => "enabled:bg-blue-500 enabled:hover:bg-blue-600 ",
            ButtonKind::Secondary => "enabled:bg-gray-500 enabled:hover:bg-gray-600 ",
        };

        format!("{default}{kind_classes}")
    };

    view! {
        <button class=class>
            {children()}
        </button>
    }
}
