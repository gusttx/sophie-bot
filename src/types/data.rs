use crate::{config::{get_config, Config}, database::Db, env::{get_env, Env}};
use tokio::sync::RwLock;
use reqwest::Client;
use crate::utils::redis::Redis;
use super::MevalContext;

#[cfg(feature = "onlinefix")]
use reqwest::{header, ClientBuilder};

pub struct Data {
    pub message: RwLock<String>,
    pub database: Db,
    #[cfg(feature = "onlinefix")]
    pub onlinefix_client: Client,
    pub weather_client: Client,
    pub redis: Redis,
    pub meval_context: MevalContext,
    pub config: &'static Config,
    pub env: &'static Env,
}

impl Data {
    pub fn new(database: Db, redis: Redis) -> Self {
        Self {
            message: RwLock::new("Hello, World!".to_string()),
            database,
            #[cfg(feature = "onlinefix")]
            onlinefix_client: get_onlinefix_client(),
            weather_client: Client::new(),
            redis,
            meval_context: MevalContext::new(),
            config: get_config(),
            env: get_env(),
        }
    }
}

#[cfg(feature = "onlinefix")]
fn get_onlinefix_client() -> reqwest::Client {
    let env = get_env();
    let user = &env.onlinefix_auth.user_id;
    let password = &env.onlinefix_auth.password;
    
    let mut cookie = header::HeaderValue::from_str(
        &format!("dle_user_id={}; dle_password={}", user, password),
    ).expect("Failed to create onlinefix header value");
    cookie.set_sensitive(true);
    
    let referer = header::HeaderValue::from_static(
        crate::utils::onlinefix::ONLINEFIX_BASE_URL
    );
    
    let mut headers = header::HeaderMap::with_capacity(2);
    headers.insert(header::COOKIE, cookie);
    headers.insert(header::REFERER, referer);
    
    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("Failed to create onlinefix client")
}
