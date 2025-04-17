use poise::serenity_prelude::{GuildId, UserId};
use serde::Deserialize;
use serde_with::{serde_as, DurationSeconds};
use std::collections::HashSet;
use std::num::NonZeroU8;
use std::sync::OnceLock;
use std::time::Duration;

static CONFIG: OnceLock<Config> = OnceLock::new();
pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let config_file = std::fs::read_to_string("config.toml")
            .expect("Failed to read configuration file 'config.toml'");
        toml::from_str(&config_file).expect("Failed to parse configuration file 'config.toml'")
    })
}

#[derive(Deserialize)]
pub struct Config {
    pub guild_ids: HashSet<GuildId>,
    pub owner_ids: HashSet<UserId>,
    pub bot: BotConfig,
    pub cache: CacheConfig,
    pub timeout: TimeoutConfig,
    pub economy: EconomyConfig,
    pub blackjack: BlackJackConfig,
    #[cfg(feature = "onlinefix")]
    pub onlinefix: OnlineFixConfig,
}

#[serde_as]
#[derive(Deserialize)]
pub struct BotConfig {
    #[serde_as(as = "DurationSeconds")]
    pub edit_tracker_duration: Duration,
    pub prefix: String,
}

#[serde_as]
#[derive(Deserialize)]
pub struct TimeoutConfig {
    #[serde_as(as = "DurationSeconds")]
    pub jankenpon: Duration,
    #[serde_as(as = "DurationSeconds")]
    pub blackjack: Duration,
    #[serde_as(as = "DurationSeconds")]
    #[cfg(feature = "onlinefix")]
    pub onlinefix: Duration,
    #[serde_as(as = "DurationSeconds")]
    pub owner_response: Duration,
}

#[derive(Deserialize)]
pub struct CacheConfig {
    #[cfg(feature = "onlinefix")]
    pub onlinefix_search_ttl: Option<u64>,
    #[cfg(feature = "onlinefix")]
    pub onlinefix_info_ttl: Option<u64>,
    #[cfg(feature = "onlinefix")]
    pub onlinefix_torrent_ttl: Option<u64>,
    pub weather_ttl: Option<u64>,
    pub weather_search_ttl: Option<u64>,
}

#[derive(Deserialize)]
pub struct EconomyConfig {
    pub initial_coins: u32,
}

#[derive(Deserialize)]
pub struct BlackJackConfig {
    pub decks: NonZeroU8,
}

#[cfg(feature = "onlinefix")]
#[derive(Deserialize)]
pub struct OnlineFixConfig {
    #[serde(deserialize_with = "validate_onlinefix_max_search_results")]
    pub max_search_results: usize,
}

#[cfg(feature = "onlinefix")]
fn validate_onlinefix_max_search_results<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = usize::deserialize(deserializer)?;

    if !(1..=10).contains(&value) {
        return Err(serde::de::Error::custom(
            "onlinefix max search results must be between 1 and 10",
        ));
    }

    Ok(value)
}
