use std::time::Duration;

use backend_api::routes::auth::oauth::discord::authorized::{
    OAuth2DiscordAuthorizedRequest, OAuth2DiscordAuthorizedResponse, Oauth2DiscordAuthorized,
};
use leptos::*;
use leptos_router::{use_navigate, use_query, NavigateOptions, Params};
use tracing::error;

use crate::{
    components::alert_message::{use_alert_message, MessageOptions, MessageVariant},
    pages::{app::APP_PAGE_URL, auth::registration::oauth2::SIGN_UP_OAUTH2_PAGE_URL},
    utils::{client::use_client, error::log_rpc_error},
};

#[derive(Clone, Params, PartialEq)]
pub struct OAuth2Params {
    code: String,
    state: String,
}

#[component]
pub fn Discord() -> impl IntoView {
    let client = use_client();
    let alert = use_alert_message();
    let query = use_query::<OAuth2Params>();
    let navigate = use_navigate();

    let discord = create_action({
        let client = client.clone();
        move |query: &OAuth2Params| {
            let client = client.clone();
            let (code, state) = (query.code.clone(), query.state.clone());

            async move {
                client
                    .rpc
                    .call::<Oauth2DiscordAuthorized>(&OAuth2DiscordAuthorizedRequest {
                        code,
                        state,
                    })
                    .await
            }
        }
    });

    create_effect(move |_| {
        let Ok(query) = query.get() else {
            return;
        };

        discord.dispatch(query);
    });

    let discord_value = discord.value();
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
            Ok(response) => match response {
                OAuth2DiscordAuthorizedResponse::Authorized(jwt_token) => {
                    client.authorizate(jwt_token);
                    alert.create(
                        "Successfully authorized via OAuth2 Discord",
                        MessageVariant::Success,
                        MessageOptions {
                            duration: Some(Duration::from_secs(3)),
                            ..Default::default()
                        },
                    );
                    navigate(APP_PAGE_URL, NavigateOptions::default());
                }
                OAuth2DiscordAuthorizedResponse::RequiresRegistration(registration_token) => {
                    client.registration_token.set(Some(registration_token));
                    navigate(SIGN_UP_OAUTH2_PAGE_URL, NavigateOptions::default());
                }
            },
            Err(err) => {
                let description = format!("{err:?}");
                error!(
                    description = description,
                    "Failed to authorizate via OAuth2 Discord"
                );

                alert.create(
                    "Failed to authorizate via OAuth2 Discord",
                    MessageVariant::Failure,
                    MessageOptions {
                        description: Some(description),
                        ..Default::default()
                    },
                );
            }
        }
    });
}
