use std::sync::Arc;

use anyhow::{Context, Error};
use api::{
    entities::{
        avatar_uri::{AvatarUri, AVATAR_SIZE},
        email::Email,
        registration_kind::RegistrationKind,
    },
    routes::auth::oauth::discord::{
        OAuth2DiscordError, OAuth2DiscordRequest, OAuth2DiscordResponse,
    },
};
use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
    Router,
};
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use rpc::server::error::{IntoProcFailure, ProcedureError};
use serde::Deserialize;
use thiserror::Error;
use tower_sessions::Session;
use tracing::{info, instrument};

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
    Router::new().route("/authorized", get(authorized))
}

#[derive(Debug, Clone)]
pub struct DiscordClient(Arc<BasicClient>);

pub fn create_client(env: &Environment) -> anyhow::Result<DiscordClient> {
    Ok(DiscordClient(Arc::new(
        BasicClient::new(
            ClientId::new(env.discord_client_id.clone()),
            Some(ClientSecret::new(env.discord_client_secret.clone())),
            AuthUrl::new("https://discord.com/oauth2/authorize".into())
                .context("Discord auth endpoint URL")?,
            Some(
                TokenUrl::new("https://discord.com/api/oauth2/token".into())
                    .context("Discord token endpoint URL")?,
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(
                env.host_url
                    .join("/api/auth/oauth/discord/authorized")?
                    .into(),
            )
            .context("redirect URL for Discord API")?,
        )
        .set_revocation_uri(
            RevocationUrl::new("https://discord.com/api/oauth2/token/revoke".into())
                .context("Discord revocation endpoint URL")?,
        ),
    )))
}

#[derive(Debug, Error)]
pub enum OAuth2DiscordSErr {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<OAuth2DiscordError> for OAuth2DiscordSErr {
    fn into_procedure_error(self) -> impl IntoProcFailure<OAuth2DiscordError> {
        match self {
            Self::Other(_) => OAuth2DiscordError::Other.into_proc_failure(),
        }
    }
}

#[instrument(skip(state), err)]
pub async fn oauth2_discord(
    State(state): State<Arc<ServerState>>,
    request: OAuth2DiscordRequest,
) -> Result<OAuth2DiscordResponse, OAuth2DiscordSErr> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = state
        .discord
        .0
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".into()))
        .add_scope(Scope::new("email".into()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    state.mem.insert_oauth2_state(csrf_token, pkce_verifier);
    Ok(OAuth2DiscordResponse {
        uri: auth_url.to_string(),
    })
}

#[derive(Deserialize)]
struct UserProfile {
    id: String,
    email: String,
    avatar: String,
}

#[instrument(skip_all, err)]
pub async fn authorized(
    State(state): State<Arc<ServerState>>,
    session: Session,
    Query(query): Query<AuthRequest>,
) -> Result<Redirect, OAuthError> {
    let pkce_verifier = state
        .mem
        .take_oauth2_state(&CsrfToken::new(query.state))
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

    info!("session id = {:?}", session.id());

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
