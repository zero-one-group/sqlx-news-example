pub const APP_USER_AGENT: &str = "curl/7.54.1";

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::new();
}
