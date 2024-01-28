use std::sync::Arc;

use ::redis::Client;
use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
    Router,
};
use common::{
    entity::user::USER_AVATAR_SIZE,
    error::auth::{oauth::OAuthError, AuthorizedError},
};
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, TokenResponse,
    TokenUrl,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use service::{query::Query as ServiceQuery, RegistrationType};
use tower_cookies::Cookies;

use crate::{
    auth::{oauth::OAUTH_STATE_EXPIRED, AuthRequest},
    cookies::{self, REGISTRATION_AVATAR_URI, REGISTRATION_EMAIL, REGISTRATION_TYPE},
    environment::Environment,
    redis::oauth,
    state::ServerState,
};

pub fn routes() -> Router<ServerState> {
    Router::new()
        .route("/", get(discord))
        .route("/authorized", get(authorized))
}

#[derive(Debug, Clone)]
pub struct DiscordClient(Arc<BasicClient>);

pub fn create_basic_client(environment: &Environment) -> DiscordClient {
    DiscordClient(Arc::new(
        BasicClient::new(
            ClientId::new(environment.discord_client_id.clone()),
            Some(ClientSecret::new(environment.discord_client_secret.clone())),
            AuthUrl::new("https://discord.com/oauth2/authorize".into())
                .expect("Discord auth endpoint URL"),
            Some(
                TokenUrl::new("https://discord.com/api/oauth2/token".into())
                    .expect("Discord token endpoint URL"),
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(format!(
                "{}/api/auth/oauth/discord/authorized",
                environment.redirect_url
            ))
            .expect("Redirect URL for Discord API"),
        )
        .set_revocation_uri(
            RevocationUrl::new("https://discord.com/api/oauth2/token/revoke".into())
                .expect("Discord revocation endpoint URL"),
        ),
    ))
}

pub async fn discord(
    State(oauth): State<DiscordClient>,
    State(redis): State<Client>,
) -> Result<Redirect, OAuthError> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, crsf_token) = oauth
        .0
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".into()))
        .add_scope(Scope::new("email".into()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    oauth::insert_state(&redis, crsf_token, pkce_verifier, OAUTH_STATE_EXPIRED).await?;
    Ok(Redirect::to(auth_url.as_str()))
}

#[derive(Deserialize)]
struct UserProfile {
    id: String,
    email: String,
    avatar: String,
}

pub async fn authorized(
    Query(query): Query<AuthRequest>,
    cookies: Cookies,
    State(client): State<DiscordClient>,
    State(redis): State<Client>,
    State(reqwest): State<reqwest::Client>,
    State(db): State<DatabaseConnection>,
) -> Result<Redirect, AuthorizedError> {
    let pkce_verifier =
        PkceCodeVerifier::new(oauth::take_state(&redis, CsrfToken::new(query.state)).await?);

    let token = client
        .0
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(oauth2::reqwest::async_http_client)
        .await?;

    let profile = reqwest
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await?
        .json::<UserProfile>()
        .await?;

    let avatar_uri = format!(
        "https://cdn.discordapp.com/avatars/{}/{}.webp?size={USER_AVATAR_SIZE}",
        profile.id, profile.avatar
    );

    let token_to_revoke = match token.refresh_token() {
        Some(val) => val.into(),
        None => token.access_token().into(),
    };

    client
        .0
        .revoke_token(token_to_revoke)
        .unwrap()
        .request_async(oauth2::reqwest::async_http_client)
        .await?;

    if ServiceQuery::find_user_by_email(&db, &profile.email)
        .await?
        .is_none()
    {
        cookies.add(cookies::create_secure_cookie(
            REGISTRATION_EMAIL,
            profile.email,
        ));

        cookies.add(cookies::create_secure_cookie(
            REGISTRATION_TYPE,
            RegistrationType::Discord.to_string(),
        ));

        cookies.add(cookies::create_secure_cookie(
            REGISTRATION_AVATAR_URI,
            avatar_uri,
        ));

        return Ok(Redirect::to("/registration_details"));
    };

    Ok(Redirect::to("/auth/successfully_authenticated"))
}
