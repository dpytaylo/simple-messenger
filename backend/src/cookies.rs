use tower_cookies::{
    cookie::{Expiration, SameSite},
    Cookie,
};

pub const REGISTRATION_EMAIL_TOKEN: &str = "registration_email";
pub const REGISTRATION_TYPE_TOKEN: &str = "registration_type";
pub const REGISTRATION_PASSWORD_TOKEN: &str = "registration_password";
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
