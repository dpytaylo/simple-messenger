pub mod oauth;
pub mod session;

const SESSION_STORAGE: u32 = 0;
const OAUTH_STATE_STORAGE: u32 = 1;

const SELECT: &str = "SELECT";
