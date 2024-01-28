use cfg_if::cfg_if;
use leptos_ssr_api_error::api_error;

pub mod oauth;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use redis::RedisError;
        use sea_orm::DbErr;
        use oauth2::{basic::BasicErrorResponseType, RequestTokenError, RevocationErrorResponseType, StandardErrorResponse};
    }
}

#[api_error(client)]
pub enum RegisterError {
    #[error("account with the same email already exists")]
    #[status_code(BAD_REQUEST)]
    AccountWithSameEmailAlreadyExists,

    #[error("no password")]
    #[status_code(BAD_REQUEST)]
    NoPassword,

    #[error("db error ({0})")]
    Db(#[from] DbErr),

    #[error("password error ({0})")]
    PasswordHash(#[from] scrypt::password_hash::Error),

    #[error("redis error ({0})")]
    Redis(#[from] RedisError),
}

#[api_error(client)]
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

#[api_error(client)]
pub enum AuthenticateError {
    #[error("the account does not exist")]
    #[status_code(BAD_REQUEST)]
    #[custom("InvalidEmailOrPassword")]
    AccountNotExists,

    #[error("invalid password")]
    #[status_code(BAD_REQUEST)]
    #[custom("InvalidEmailOrPassword")]
    InvalidPassword,

    #[error("not email registration type")]
    NotEmailRegistrationType,

    #[error("db error ({0})")]
    Db(#[from] DbErr),

    #[error("password error ({0})")]
    PasswordHashError(#[from] scrypt::password_hash::Error),

    #[error("redis error ({0})")]
    RedisError(#[from] RedisError),
}
