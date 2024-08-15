use std::time::Duration;

use leptos::*;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};
use uuid::Uuid;

use crate::atoms::css_transition::CssTransition;

pub fn use_alert_message() -> AlertMessages {
    expect_context::<AlertMessages>()
}

#[derive(Clone)]
pub struct AlertMessages {
    messages: RwSignal<Vec<AlertMessage>>,
}

#[derive(Default, Debug, Clone)]
pub enum MessageVariant {
    #[default]
    Success,
    Failure,
}

#[derive(Debug, Clone)]
pub struct MessageOptions {
    pub description: Option<String>,
    pub is_closable: bool,
    pub duration: Option<Duration>,
}

impl Default for MessageOptions {
    fn default() -> Self {
        Self {
            description: None,
            is_closable: true,
            duration: None,
        }
    }
}

impl AlertMessages {
    pub fn create(&self, title: impl ToString, variant: MessageVariant, options: MessageOptions) {
        self.messages.update(move |val| {
            val.push(AlertMessage {
                id: Uuid::new_v4(),
                title: title.to_string(),
                variant: variant,
                description: options.description,
                is_closable: options.is_closable,
                duration: options.duration,
            })
        })
    }

    // pub fn create_error(&self, title: impl ToString, description: impl ToString) {
    //     let title = title.to_string();
    //     let description = description.to_string();

    //     error!(title = title, description = description);

    //     self.messages.update(move |val| {
    //         val.push(AlertMessage {
    //             id: Uuid::new_v4(),
    //             title,
    //             description,
    //         })
    //     })
    // }
}

#[derive(Clone)]
pub struct AlertMessage {
    pub id: Uuid,
    pub title: String,
    pub variant: MessageVariant,
    pub description: Option<String>,
    pub is_closable: bool,
    pub duration: Option<Duration>,
}

impl AlertMessage {
    pub fn new(
        id: Uuid,
        title: String,
        variant: MessageVariant,
        description: Option<String>,
        is_closable: bool,
        duration: Option<Duration>,
    ) -> Self {
        Self {
            id,
            title,
            variant,
            description,
            is_closable,
            duration,
        }
    }
}

#[component]
pub fn AlertMessageProvider() -> impl IntoView {
    let messages = Default::default();
    provide_context(AlertMessages { messages });

    view! {
        <div class="flex-shrink-0 fixed px-2 top-3 left-1/2 -translate-x-1/2 w-full max-w-screen-md flex flex-col items-stretch z-50 gap-2">
            <For
                each=messages
                key=|val| val.id
                let:data
            >
                <Alert data />
            </For>
        </div>
    }
}

#[component]
pub fn Alert(data: AlertMessage) -> impl IntoView {
    let alert = use_alert_message();
    let node_ref = create_node_ref();
    let (show, set_show) = create_signal(true);

    let class = match data.variant {
        MessageVariant::Success => "bg-gray-50 border-gray-200",
        MessageVariant::Failure => "bg-red-100 border-red-300",
    };

    let icon = match data.variant {
        MessageVariant::Success => view! { <SuccessIcon/> },
        MessageVariant::Failure => view! { <FailureIcon/> },
    };

    let close_button_callback = move || set_show(false);

    let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
        move |_| {
            close_button_callback();
        },
        data.duration
            .map(|val| val.as_secs_f64() * 1000.0)
            .unwrap_or_default(),
    );

    if data.duration.is_some() {
        start(());
    }

    let close_button = move || {
        data.is_closable.then_some(move || {
            view! {
                <button
                    on:click=move |_| close_button_callback()
                >
                    <CloseIcon/>
                </button>
            }
        })
    };

    view! {
        <CssTransition
            node_ref=node_ref
            show=show
            appear=true
            enter_from_class="message-before-appear"
            enter_to_class="message-after-appear"
            on_after_leave=move |_| alert.messages.update(|t| t.retain(|val| val.id != data.id))
            duration=Duration::from_millis(300)
        >
            <div
                class=format!("flex gap-2 p-5 border rounded-lg shadow-lg {class}")
                _ref=node_ref
            >
                {icon}
                <div class="flex-grow">
                    <p class="text-base font-medium">{data.title}</p>
                    <p class="text-sm">{data.description}</p>
                </div>
                {close_button}
            </div>
        </CssTransition>
    }
}

#[component]
fn SuccessIcon() -> impl IntoView {
    const SUCCESS_ICON: &str = include_str!("../../assets/success_icon.svg");

    view! {
        <div
            class="flex-shrink-0 w-6 h-6 text-green-600"
            inner_html=SUCCESS_ICON
        />
    }
}

#[component]
fn FailureIcon() -> impl IntoView {
    const ALERT_ICON: &str = include_str!("../../assets/failure_icon.svg");

    view! {
        <div
            class="flex-shrink-0 w-6 h-6 text-red-600"
            inner_html=ALERT_ICON
        />
    }
}

#[component]
fn CloseIcon() -> impl IntoView {
    const CLOSE_ICON: &str = include_str!("../../assets/close_icon.svg");

    view! {
        <div
            class="w-6 h-6 text-gray-500"
            inner_html=CLOSE_ICON
        />
    }
}
