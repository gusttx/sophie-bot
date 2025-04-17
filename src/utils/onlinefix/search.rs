use scraper::{ElementRef, Selector};
use crate::{
    config::get_config, types::{OnlineFixGame, OnlineFixSearch}, utils::{
        percent_encode,
        redis::Redis,
        scraper::{Scraper, ScraperError}, ErrorMessage,
    }
};

use super::{get_onlinefix_selectors, ONLINEFIX_BASE_URL, ONLINEFIX_GAMES_URL};

pub async fn search<'a>(
    client: &reqwest::Client,
    redis: &Redis,
    search: &'a String,
) -> Result<OnlineFixSearch<'a>, ErrorMessage> {
    let search_lower = search.to_lowercase();
    let search_redis_key = format!("search:{}", search_lower);
    let encoded_search = percent_encode(&search_lower);
    let search_url =
        format!("{}/index.php?do=search&subaction=search&story={}", ONLINEFIX_BASE_URL, encoded_search);

    if let Some(searches) = redis.get::<Vec<OnlineFixGame>>(&search_redis_key).await {
        return Ok(OnlineFixSearch {
            input: search,
            search_url,
            games: searches
        })
    };

    let config = get_config();
    let max_results = config.onlinefix.max_search_results;
    let search_ttl = config.cache.onlinefix_search_ttl;

    let games = Scraper::new(client.get(&search_url))
        .set_root_element(".news.news-search .article")
        .get(move |select| select.filter_map(parse_game).take(max_results).collect())
        .await
        .map_err(|err| map_search_error(err, search))?;

    redis.set(&search_redis_key, &games, search_ttl).await;

    Ok(OnlineFixSearch {
        input: search,
        search_url,
        games
    })
}

fn parse_game(element: ElementRef) -> Option<OnlineFixGame> {
    let selectors = get_onlinefix_selectors();

    let url = extract_attr(&element, &selectors.search_url, "href")
        .filter(|s| s.starts_with(ONLINEFIX_GAMES_URL))?;
    let image = extract_attr(&element, &selectors.search_image, "data-src")?;
    let views = extract_sibling_text(&element, &selectors.search_views)?;
    let release_date = extract_sibling_text(&element, &selectors.search_release)?;

    let title = element
        .select(&selectors.search_title)
        .next()?
        .text()
        .next()?
        .trim()
        .to_string()
        .replace(" по сети", "");

    Some(OnlineFixGame {
        image,
        url,
        title,
        views,
        release_date,
    })
}

fn map_search_error(err: ScraperError, search: &str) -> ErrorMessage {
    match err {
        ScraperError::ElementNotFound | ScraperError::ElementNotFoundWithId(_) => ErrorMessage::with_log(
            "Não foi possível obter a lista de jogos :parrot:",
            format!("Failed to get an element for search `{}`", search),
        ),
        ScraperError::JoinError(err) => ErrorMessage::with_log(
            "Ocorreu um erro interno :pleading_face:",
            format!("Failed to join tasks: {}", err),
        ),
        ScraperError::ResponseError(err) => ErrorMessage::with_log(
            "Não foi possível obter resposta para a pesquisa :pensive:",
            format!("Failed to get response for search `{}`: {}", search, err),
        ),
    }
}

fn extract_attr(element: &ElementRef, selector: &Selector, attr: &str) -> Option<String> {
    element
        .select(selector)
        .next()?
        .value()
        .attr(attr)
        .map(|s| s.to_string())
}

fn extract_sibling_text(element: &ElementRef, selector: &Selector) -> Option<String> {
    element
        .select(selector)
        .next()?
        .next_sibling()?
        .value()
        .as_text()
        .map(|s| s.trim().to_string())
}
