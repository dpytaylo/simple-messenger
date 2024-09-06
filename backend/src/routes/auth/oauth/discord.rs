use std::sync::Arc;

use anyhow::{Context, Error};
use api::entities::{
    avatar_uri::{AvatarUri, AVATAR_SIZE},
    email::Email,
    registration_kind::RegistrationKind,
};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use tower_sessions::Session;

use super::{AuthRequest, OAuthError, AUTH_SUCCESS_PAGE_URL, SIGN_UP_OAUTH2_PAGE_URL};
use crate::{
    environment::Environment,
    routes::auth::generate_jwt_token,
    session::{
        insert_session_key, JWT_TOKEN_KEY, REGISTRATION_AVATAR_URI_KEY, REGISTRATION_EMAIL_KEY,
        REGISTRATION_KIND_KEY,
    },
    state::{ServerState, ServerStateWrapper},
};

pub fn routes() -> Router<ServerStateWrapper> {
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
    State(state): State<Arc<ServerState>>,
) -> Result<impl IntoResponse, OAuthError> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = state
        .discord
        .0
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".into()))
        .add_scope(Scope::new("email".into()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    state.mem.insert_oauth_state(csrf_token, pkce_verifier);
    Ok(Redirect::to(auth_url.as_str()))
}

#[derive(Deserialize)]
struct UserProfile {
    id: String,
    email: String,
    avatar: String,
}

pub async fn authorized(
    State(state): State<Arc<ServerState>>,
    session: Session,
    Query(query): Query<AuthRequest>,
) -> Result<Redirect, OAuthError> {
    let pkce_verifier = state
        .mem
        .take_oauth_state(&CsrfToken::new(query.state))
        .context("missing pkce verifier")?;

    let token = state
        .discord
        .0
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .context("failed to request token")?;

    let profile = state
        .reqwest
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed to request token")?
        .json::<UserProfile>()
        .await
        .context("failed to request token")?;

    let email = Email::new(profile.email).context("invalid email")?;
    let avatar_uri = AvatarUri::new(format!(
        "https://cdn.discordapp.com/avatars/{}/{}.webp?size={AVATAR_SIZE}",
        profile.id, profile.avatar
    ))
    .context("invalid avatar URI")?;

    let token_to_revoke = match token.refresh_token() {
        Some(val) => val.into(),
        None => token.access_token().into(),
    };

    state
        .discord
        .0
        .revoke_token(token_to_revoke)
        .unwrap()
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .context("failed to revoke token")?;

    let Some(user) = db::user::find_by_email(&state.db, &email)
        .await
        .map_err(Error::msg)?
    else {
        insert_session_key(&session, REGISTRATION_KIND_KEY, RegistrationKind::Discord).await?;
        insert_session_key(&session, REGISTRATION_EMAIL_KEY, email).await?;
        insert_session_key(&session, REGISTRATION_AVATAR_URI_KEY, avatar_uri).await?;

        return Ok(Redirect::to(SIGN_UP_OAUTH2_PAGE_URL));
    };

    let token = generate_jwt_token(&state, user.id.to_string())?;
    insert_session_key(&session, JWT_TOKEN_KEY, token).await?;

    Ok(Redirect::to(AUTH_SUCCESS_PAGE_URL))
}
