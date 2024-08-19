use std::sync::Arc;

use anyhow::{Context, Error};
use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
    Router,
};
use common::routes::auth::registration::register::RegistrationKind;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use service::query::Query as ServiceQuery;
use tower_sessions::Session;

use super::{AuthRequest, OAuthError};
use crate::{
    environment::Environment,
    session::{insert_session_key, REGISTRATION_EMAIL_KEY, REGISTRATION_KIND_KEY},
    state::{ServerState, ServerStateWrapper},
};

pub fn routes() -> Router<ServerStateWrapper> {
    Router::new()
        .route("/", get(google))
        .route("/authorized", get(authorized))
}

#[derive(Debug, Clone)]
pub struct GoogleClient(Arc<BasicClient>);

pub fn create_basic_client(environment: &Environment) -> GoogleClient {
    GoogleClient(Arc::new(
        BasicClient::new(
            ClientId::new(environment.google_client_id.clone()),
            Some(ClientSecret::new(environment.google_client_secret.clone())),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".into())
                .expect("Google auth endpoint URL"),
            Some(
                TokenUrl::new("https://oauth2.googleapis.com/token".into())
                    .expect("Google token endpoint URL"),
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(format!(
                "{}/api/auth/oauth/google/authorized",
                environment.redirect_url
            ))
            .expect("Redirect URL for Google API"),
        )
        .set_revocation_uri(
            RevocationUrl::new("https://oauth2.googleapis.com/revoke".into())
                .expect("Google revocation endpoint URL"),
        ),
    ))
}

pub async fn google(State(state): State<Arc<ServerState>>) -> Result<Redirect, OAuthError> {
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

    state.mem.insert_oauth_state(csrf_token, pkce_verifier);
    Ok(Redirect::to(auth_url.as_str()))
}

#[derive(Deserialize)]
struct UserProfile {
    email: String,
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

    if ServiceQuery::find_user_by_email(&state.db, &profile.email)
        .await
        .map_err(Error::msg)?
        .is_none()
    {
        insert_session_key(&session, REGISTRATION_KIND_KEY, RegistrationKind::Google).await?;
        insert_session_key(&session, REGISTRATION_EMAIL_KEY, profile.email).await?;

        return Ok(Redirect::to("/registration_details"));
    };

    Ok(Redirect::to("/oauth2_register_successfully"))
}
