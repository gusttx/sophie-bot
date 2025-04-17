use std::borrow::Cow;

use crate::{
    types::{Context, UnitResult, Weather, WeatherSearch},
    utils::{discord::{embed::Embed, reply::Reply}, percent_encode},
};
use deunicode::deunicode;
use log::error;
use poise::command;

const WEATHER_API_URL: &str = "https://api.weatherapi.com/v1";
const NO_LOCATION_FOUND: &str = "Nenhuma cidade encontrada";
const EMPTY_LIST: &str = "[]";

const EMBED_COLOR_DAY: u32 = 0x00FFF0;
const EMBED_COLOR_NIGHT: u32 = 0x001088;

async fn autocomplete_weather(ctx: Context<'_>, search: &str) -> Vec<Cow<'static, str>> {
    let search = deunicode(&search.trim().to_lowercase());
    let key = format!("weather_search:{}", search);
    let default = vec![Cow::Borrowed(NO_LOCATION_FOUND)];

    if !(3..=80).contains(&search.len()) {
        return default;
    }

    let data = ctx.data();
    let redis = &data.redis;
    let client = &data.weather_client;
    let api_key = &data.env.weather_api_key;
    let search_ttl = data.config.cache.weather_search_ttl;

    let searches = redis.get::<Vec<Cow<'static, str>>>(&key).await;

    if let Some(searches) = searches {
        if searches.is_empty() {
            return default;
        }
        return searches;
    }

    let search = percent_encode(search);
    let url = format!(
        "{}/search.json?key={}&q={}&lang=pt",
        WEATHER_API_URL, api_key, search
    );

    let response = match client.get(&url).send().await {
        Ok(response) => response,
        Err(err) => {
            error!("Failed to get weather autocomplete: {}", err);
            redis.set::<String>(key, &EMPTY_LIST.to_string(), search_ttl).await;
            return default;
        }
    };

    let searches: Vec<Cow<'_, str>> = match response.json::<Vec<WeatherSearch>>().await {
        Ok(result) => result
            .into_iter()
            .map(|w| {
                Cow::Owned(format!(
                    "{} - {}, {}",
                    w.location.name, w.location.region, w.location.country
                ))
            })
            .collect(),

        Err(err) => {
            error!("Failed to parse weather autocomplete response: {}", err);
            redis.set::<String>(key, &EMPTY_LIST.to_string(), search_ttl).await;
            return default;
        }
    };

    redis.set(&key, &searches, search_ttl).await;

    if searches.is_empty() {
        return default;
    }

    searches
}

/// Get the weather of a location
#[command(slash_command, user_cooldown = 10)]
pub async fn weather(
    ctx: Context<'_>,
    #[autocomplete = autocomplete_weather]
    #[description = "Location to get the weather"]
    #[min_length = 3]
    #[max_length = 80]
    location: String,
) -> UnitResult {
    let user_location = location;
    let mut location = deunicode(&user_location.to_lowercase());
    let key = format!("weather:{}", location);

    let no_found = Reply::ephemeral("Nenhuma cidade encontrada :pleading_face:");

    if location == NO_LOCATION_FOUND {
        return no_found.send_ok(&ctx).await;
    }

    let data = ctx.data();
    let redis = &data.redis;
    let client = &data.weather_client;
    let api_key = &data.env.weather_api_key;
    let weather_ttl = data.config.cache.weather_ttl;

    location = percent_encode(location);
    
    if let Some(searches) = redis.get::<Vec<String>>(format!("weather_search:{}", location)).await {
        match searches.into_iter().nth(0) {
            Some(loc) => location = loc,
            None => {
                return no_found.send_ok(&ctx).await;
            }
        }
    }

    let weather = redis.get::<Weather>(&key).await;

    if let Some(weather) = weather {
        return create_weather_reply(weather).send_ok(&ctx).await;
    }

    let url = format!(
        "{}/current.json?key={}&q={}&lang=pt",
        WEATHER_API_URL, api_key, location
    );

    let response = match client.get(&url).send().await {
        Ok(response) => response,
        Err(err) => {
            error!("Failed to get weather: {}", err);
            return Reply::ephemeral("Ocorreu um erro ao buscar a previsão do tempo :t_rex:")
                .send_ok(&ctx)
                .await;
        }
    };

    let Ok(weather) = response.json::<Option<Weather>>().await else {
        return no_found.send_ok(&ctx).await;
    };


    redis.set(&key, &weather, weather_ttl).await;

    let Some(weather) = weather else {
        return no_found.send_ok(&ctx).await;
    };

    create_weather_reply(weather).send_ok(&ctx).await
}

fn create_weather_reply(weather: Weather) -> Reply {
    let Weather { current, location } = weather;

    let color = if current.is_day == 1 {
        EMBED_COLOR_DAY
    } else {
        EMBED_COLOR_NIGHT
    };

    let title = format!(
        ":sunny: Previsão para {} - {}, {}",
        location.name, location.region, location.country
    );

    let embed = Embed::new(color, title)
        .inline_field(
            format!("Temperatura: {} °C", current.temp_c),
            format!("Temperatura: {} °F", current.temp_f),
        )
        .inline_field(
            format!("Sensação: {} °C", current.feelslike_c),
            format!("Sensação: {} °F", current.feelslike_f),
        )
        .field("Umidade do ar", format!("{}%", current.humidity))
        .small_image(format!("https:{}", current.condition.icon))
        .desc(format!("Atualizado <t:{}:R>", current.last_updated_epoch))
        .footer(format!("{} - via weatherapi.com", super::DEPARTMENT_NAME));

    Reply::with_embed(embed)
}
