use std::sync::Arc;

use anyhow::{Context, Error};
use api::{
    entities::{email::Email, registration_kind::RegistrationKind},
    routes::auth::oauth::google::{OAuth2GoogleError, OAuth2GoogleRequest, OAuth2GoogleResponse},
};
use axum::{
    extract::{Query, State},
    response::Redirect,
};
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use rpc::server::error::{IntoProcFailure, ProcedureError};
use serde::Deserialize;
use thiserror::Error;
use tower_sessions::Session;
use tracing::instrument;

use super::{AuthRequest, OAuthError};
use crate::{
    environment::Environment,
    session::{insert_session_key, REGISTRATION_EMAIL_KEY, REGISTRATION_KIND_KEY},
    state::ServerState,
};

#[derive(Debug, Clone)]
pub struct GoogleClient(Arc<BasicClient>);

pub fn create_client(env: &Environment) -> anyhow::Result<GoogleClient> {
    Ok(GoogleClient(Arc::new(
        BasicClient::new(
            ClientId::new(env.google_client_id.clone()),
            Some(ClientSecret::new(env.google_client_secret.clone())),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".into())
                .context("Google auth endpoint URL")?,
            Some(
                TokenUrl::new("https://oauth2.googleapis.com/token".into())
                    .context("Google token endpoint URL")?,
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(
                env.host_url
                    .join("/api/auth/oauth/google/authorized")?
                    .into(),
            )
            .context("redirect URL for Google API")?,
        )
        .set_revocation_uri(
            RevocationUrl::new("https://oauth2.googleapis.com/revoke".into())
                .context("Google revocation endpoint URL")?,
        ),
    )))
}

#[derive(Debug, Error)]
pub enum OAuth2GoogleSErr {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ProcedureError<OAuth2GoogleError> for OAuth2GoogleSErr {
    fn into_procedure_error(self) -> impl IntoProcFailure<OAuth2GoogleError> {
        match self {
            Self::Other(_) => OAuth2GoogleError::Other.into_proc_failure(),
        }
    }
}

#[instrument(skip(state), err)]
pub async fn oauth2_google(
    State(state): State<Arc<ServerState>>,
    request: OAuth2GoogleRequest,
) -> Result<OAuth2GoogleResponse, OAuth2GoogleSErr> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = state
        .google
        .0
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".into(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    state.mem.oauth2_state.insert(&csrf_token, &pkce_verifier);
    Ok(OAuth2GoogleResponse {
        uri: auth_url.to_string(),
    })
}

#[derive(Deserialize)]
struct UserProfile {
    email: String,
}

#[instrument(skip_all, err)]
pub async fn authorized(
    State(state): State<Arc<ServerState>>,
    session: Session,
    Query(query): Query<AuthRequest>,
) -> Result<Redirect, OAuthError> {
    let pkce_verifier = state
        .mem
        .oauth2_state
        .take(&CsrfToken::new(query.state))
        .context("missing pkce verifier")?;

    let token = state
        .google
        .0
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .context("failed to request token")?;

    let profile = state
        .reqwest
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed to request token")?
        .json::<UserProfile>()
        .await
        .context("failed to request token")?;

    let email = Email::new(profile.email).context("invalid email")?;

    let token_to_revoke = match token.refresh_token() {
        Some(val) => val.into(),
        None => token.access_token().into(),
    };

    state
        .google
        .0
        .revoke_token(token_to_revoke)
        .unwrap()
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .context("failed to revoke token")?;

    if db::user::find_by_email(&state.db, &email)
        .await
        .map_err(Error::msg)?
        .is_none()
    {
        insert_session_key(&session, REGISTRATION_KIND_KEY, RegistrationKind::Google).await?;
        insert_session_key(&session, REGISTRATION_EMAIL_KEY, email).await?;

        // return Ok(Redirect::to(SIGN_UP_OAUTH2_PAGE_URL));
    };

    // Ok(Redirect::to(AUTH_SUCCESS_PAGE_URL))

    todo!();
}
