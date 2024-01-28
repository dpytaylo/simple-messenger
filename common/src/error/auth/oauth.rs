use cfg_if::cfg_if;
use leptos_ssr_api_error::api_error;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use redis::RedisError;
    }
}

#[api_error(client)]
pub enum OAuthError {
    #[error("redis error ({0})")]
    RedisError(#[from] RedisError),
}
