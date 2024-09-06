use time::ext::NumericalDuration;
use time::OffsetDateTime;
use tower_cookies::{
    cookie::{Expiration, SameSite},
    Cookie,
};

// pub const SESSION_TOKEN: &str = "session-token";

pub fn create_secure_cookie(key: &'static str, value: String) -> Cookie<'static> {
    Cookie::build((key, value))
        .http_only(true) // Defences against XSS
        .secure(true) // Only secure connention (https)
        .same_site(SameSite::Lax) // Defences against CSRF (https://portswigger.net/web-security/csrf)
        .path("/")
        .expires(Expiration::DateTime(
            OffsetDateTime::now_utc().saturating_add(1.hours()),
        ))
        .build()
}
