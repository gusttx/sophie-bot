use std::{sync::OnceLock, env};

fn get_env_var(var_name: &str) -> String {
    env::var(var_name)
        .expect(&format!("Failed to retrieve env var {}", var_name))
}

fn get_parsed_env_var<T>(var_name: &str) -> T
where 
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    get_env_var(var_name)
        .parse()
        .expect(&format!("Failed to parse env var {}", var_name))
}

pub struct Env {
    pub discord_token: String,
    pub mysql: MySqlEnv,
    pub redis: RedisEnv,
    #[cfg(feature = "onlinefix")]
    pub onlinefix_auth: OnlineFixAuthEnv,
    pub weather_api_key: String
}

static ENV: OnceLock<Env> = OnceLock::new();
pub fn get_env() -> &'static Env {
    ENV.get_or_init(|| Env {
        discord_token: get_env_var("DISCORD_TOKEN"),
        mysql: MySqlEnv {
            port: get_parsed_env_var("MYSQL_PORT"),
            password: get_env_var("MYSQL_ROOT_PASSWORD"),
            database: get_env_var("MYSQL_DATABASE"),
        },
        redis: RedisEnv {
            port: get_parsed_env_var("REDIS_PORT"),
            password: get_env_var("REDIS_PASSWORD"),
        },
        #[cfg(feature = "onlinefix")]
        onlinefix_auth: OnlineFixAuthEnv {
            user_id: get_env_var("ONLINEFIX_DLE_USER_ID"),
            password: get_env_var("ONLINEFIX_DLE_PASSWORD"),
        },
        weather_api_key: get_env_var("WEATHER_API_KEY"),
    })
}

pub struct MySqlEnv {
    port: u16,
    password: String,
    database: String,
}
impl MySqlEnv {
    pub fn connection_string(&self) -> String {
        format!("mysql://root:{}@localhost:{}/{}", self.password, self.port, self.database)
    }
}

pub struct RedisEnv {
    port: u16,
    password: String,
}
impl RedisEnv {
    pub fn connection_string(&self) -> String {
        format!("redis://:{}@localhost:{}", self.password, self.port)
    }
}

#[cfg(feature = "onlinefix")]
pub struct OnlineFixAuthEnv {
    pub user_id: String,
    pub password: String,
}