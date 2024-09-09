use backend_api::routes::auth::oauth::{
    discord::{OAuth2DiscordRequest, Oauth2Discord},
    google::{OAuth2GoogleRequest, Oauth2Google},
};
use leptos::*;
use leptos_use::use_window;
use tracing::error;

use crate::{
    components::alert_message::{use_alert_message, MessageVariant},
    utils::{client::use_client, error::log_rpc_error},
};

#[component]
pub fn OAuth2Links(
    #[prop(default = MaybeSignal::Static(Default::default()), into)] class: MaybeSignal<String>,
) -> impl IntoView {
    let client = use_client();
    let window = use_window();
    let alert = use_alert_message();

    let class = move || format!("flex flex-row justify-start gap-3 {}", class());

    let google = {
        let client = client.clone();
        create_action(move |_: &()| {
            let client = client.clone();
            async move {
                client
                    .rpc
                    .call::<Oauth2Google>(&OAuth2GoogleRequest {})
                    .await
            }
        })
    };

    let discord = create_action(move |_: &()| {
        let client = client.clone();
        async move {
            client
                .rpc
                .call::<Oauth2Discord>(&OAuth2DiscordRequest {})
                .await
        }
    });

    let google_value = google.value();
    let discord_value = discord.value();

    let on_google = move |_| {
        google.dispatch(());
    };

    let on_discord = move |_| {
        discord.dispatch(());
    };

    {
        let window = window.clone();
        let alert = alert.clone();

        create_effect(move |_| {
            let Some(rpc_result) = google_value.get() else {
                return;
            };

            let result = match rpc_result {
                Ok(val) => val,
                Err(err) => {
                    log_rpc_error(err);
                    return;
                }
            };

            match result {
                Ok(val) => {
                    window
                        .as_ref()
                        .unwrap()
                        .location()
                        .set_href(&val.uri)
                        .unwrap();
                }
                Err(err) => {
                    error!(description = format!("{err:?}"), "OAuth2 Error");
                    alert.create("OAuth2 Error", MessageVariant::Failure, Default::default());
                }
            }
        });
    }

    create_effect(move |_| {
        let Some(rpc_result) = discord_value.get() else {
            return;
        };

        let result = match rpc_result {
            Ok(val) => val,
            Err(err) => {
                log_rpc_error(err);
                return;
            }
        };

        match result {
            Ok(val) => {
                window
                    .as_ref()
                    .unwrap()
                    .location()
                    .set_href(&val.uri)
                    .unwrap();
            }
            Err(err) => {
                error!(description = format!("{err:?}"), "OAuth2 Error");
                alert.create("OAuth2 Error", MessageVariant::Failure, Default::default());
            }
        }
    });

    view! {
        <div class=class>
            <button on:click=on_google>
                <img class="w-10 h-10 p-1 hover:bg-slate-100 rounded" src="/assets/google_logo.svg" />
            </button>

            <button on:click=on_discord>
                <img class="w-10 h-10 p-1 hover:bg-slate-100 rounded" src="/assets/discord_logo_blue.svg" />
            </button>
        </div>
    }
}
