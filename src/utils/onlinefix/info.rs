use scraper::html::Select;
use crate::{
    config::get_config,
    types::OnlineFixGameInfo,
    utils::{
        redis::Redis,
        scraper::{Scraper, ScraperError}, ErrorMessage,
    },
};
use super::{get_onlinefix_selectors, ONLINEFIX_GAMES_URL, ONLINEFIX_UPLOADS_URL};

pub async fn info(
    client: &reqwest::Client,
    redis: &Redis,
    path: &str,
) -> Result<OnlineFixGameInfo, ErrorMessage> {
    let info_redis_key = format!("info:{}", path);

    if let Some(game_info) = redis.get::<OnlineFixGameInfo>(&info_redis_key).await {
        return Ok(game_info);
    }

    let config = get_config();
    let info_ttl = config.cache.onlinefix_info_ttl;

    let url = format!("{}/{}", ONLINEFIX_GAMES_URL, path);
    let game_info_req = client.get(&url);

    let (build, download) = Scraper::new(game_info_req)
        .set_root_element(".quote > div")
        .get(scrape_info)
        .await
        .and_then(|inner| inner)
        .map_err(|err| map_info_error(err, &url))?;

    let game_info = OnlineFixGameInfo {
        build,
        download,
        url,
    };

    redis.set(&info_redis_key, &game_info, info_ttl).await;

    Ok(game_info)
}

fn scrape_info(mut select: Select) -> Result<(String, String), ScraperError> {
    let selectors = get_onlinefix_selectors();

    let element = select.next().ok_or(ScraperError::ElementNotFound)?;

    let build = element
        .select(&selectors.info_build)
        .next()
        .ok_or(ScraperError::ElementNotFound)?
        .text()
        .next()
        .ok_or(ScraperError::ElementNotFound)?
        .replace("Версия игры: ", "");

    let download_url = element
        .select(&selectors.info_download)
        .find(|element| {
            element
                .text()
                .next()
                .map(|text| text.ends_with("торрент") || text.ends_with("Torrent"))
                .unwrap_or(false)
        })
        .ok_or(ScraperError::ElementNotFoundWithId("Torrent Link"))?
        .attr("href")
        .ok_or(ScraperError::ElementNotFoundWithId("Torrent Link"))?
        .replace(ONLINEFIX_UPLOADS_URL, "");

    Ok((build, download_url))
}

fn map_info_error(err: ScraperError, info_url: &str) -> ErrorMessage {
    match err {
        ScraperError::ElementNotFound => ErrorMessage::with_log(
            "Não foi possível obter informações do jogo :parrot:",
            format!("Failed to get an element for info `{}`", info_url),
        ),
        ScraperError::JoinError(err) => ErrorMessage::with_log(
            "Ocorreu um erro interno :pleading_face:",
            format!("Failed to join tasks: {}", err),
        ),
        ScraperError::ResponseError(err) => ErrorMessage::with_log(
            "Não foi possível obter resposta para as informações :pensive:",
            format!(
                "Failed to get response for info `{}`: {}",
                info_url, err
            ),
        ),
        ScraperError::ElementNotFoundWithId(_) => {
            ErrorMessage::new("Esse jogo não possui torrent :confused:")
        }
    }
}
