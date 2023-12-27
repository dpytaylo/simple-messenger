use ::redis::{Client, RedisError};
use api_error_derive::ApiError;
use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
    Router,
};
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType},
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RequestTokenError, RevocationErrorResponseType, RevocationUrl,
    Scope, StandardErrorResponse, TokenResponse, TokenUrl,
};
use sea_orm::{DatabaseConnection, DbErr};
use serde::Deserialize;
use service::{query::Query as ServiceQuery, RegistrationType};
use thiserror::Error;
use tower_cookies::Cookies;

use crate::{
    auth::oauth::OAUTH_STATE_EXPIRED,
    cookies::{self, REGISTRATION_EMAIL_TOKEN, REGISTRATION_TYPE_TOKEN},
    environment::Environment,
    redis::oauth,
    state::ServerState,
};

pub fn routes() -> Router<ServerState> {
    Router::new()
        .route("/", get(google))
        .route("/authorized", get(authorized))
}

pub fn create_basic_client(environment: &Environment) -> BasicClient {
    BasicClient::new(
        ClientId::new(environment.google_client_id.clone()),
        Some(ClientSecret::new(environment.google_client_secret.clone())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_owned())
            .expect("Google auth endpoint URL"),
        Some(
            TokenUrl::new("https://oauth2.googleapis.com/token".to_owned())
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
        RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_owned())
            .expect("Google revocation endpoint URL"),
    )
}

#[derive(ApiError, Debug, Error)]
pub enum GoogleError {
    #[error("redis error ({0})")]
    RedisError(#[from] RedisError),
}

pub async fn google(
    State(client): State<BasicClient>,
    State(redis): State<Client>,
) -> Result<Redirect, GoogleError> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, crsf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_owned(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    oauth::insert_state(&redis, crsf_token, pkce_verifier, OAUTH_STATE_EXPIRED).await?;
    Ok(Redirect::to(auth_url.as_str()))
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(ApiError, Debug, Error)]
pub enum AuthorizedError {
    #[error("redis error ({0})")]
    RedisError(#[from] RedisError),

    #[error("request token error ({0})")]
    RequestTokenError(
        #[from]
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),

    #[error("reqwest error ({0})")]
    Reqwest(#[from] reqwest::Error),

    #[error("failed to revoke token ({0})")]
    FailedToRevokeToken(
        #[from]
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<RevocationErrorResponseType>,
        >,
    ),

    #[error("db error ({0})")]
    Db(#[from] DbErr),
}

#[derive(Deserialize)]
pub struct UserProfile {
    email: String,
}

pub async fn authorized(
    Query(query): Query<AuthRequest>,
    cookies: Cookies,
    State(redis): State<Client>,
    State(client): State<BasicClient>,
    State(reqwest): State<reqwest::Client>,
    State(db): State<DatabaseConnection>,
) -> Result<Redirect, AuthorizedError> {
    let pkce_verifier =
        PkceCodeVerifier::new(oauth::take_state(&redis, CsrfToken::new(query.state)).await?);

    let token = client
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(oauth2::reqwest::async_http_client)
        .await?;

    let profile = reqwest
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(token.access_token().secret().to_owned())
        .send()
        .await?
        .json::<UserProfile>()
        .await?;

    let token_to_revoke = match token.refresh_token() {
        Some(val) => val.into(),
        None => token.access_token().into(),
    };

    client
        .revoke_token(token_to_revoke)
        .unwrap()
        .request_async(oauth2::reqwest::async_http_client)
        .await?;

    if ServiceQuery::find_user_by_email(&db, &profile.email)
        .await?
        .is_none()
    {
        cookies.add(cookies::create_secure_cookie(
            REGISTRATION_EMAIL_TOKEN,
            profile.email,
        ));
        cookies.add(cookies::create_secure_cookie(
            REGISTRATION_TYPE_TOKEN,
            RegistrationType::Google.to_string(),
        ));
        return Ok(Redirect::to("/registration_details"));
    };

    Ok(Redirect::to("/auth/successfully_authenticated"))
}
