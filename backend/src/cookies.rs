use tower_cookies::{
    cookie::{Expiration, SameSite},
    Cookie,
};

pub const REGISTRATION_EMAIL: &str = "registration_email";
pub const REGISTRATION_TYPE: &str = "registration_type";
pub const REGISTRATION_PASSWORD: &str = "registration_password";
pub const REGISTRATION_AVATAR_URI: &str = "registration_avatar_uri";

pub const SESSION_TOKEN: &str = "session-token";

pub fn create_secure_cookie(key: &'static str, value: String) -> Cookie {
    Cookie::build((key, value))
        .http_only(true) // Defences against XSS
        .secure(true) // Only secure connention (https)
        .same_site(SameSite::Strict) // Defences against CSRF (https://portswigger.net/web-security/csrf)
        .path("/")
        .expires(Expiration::Session)
        .build()
}
