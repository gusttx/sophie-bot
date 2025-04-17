mod search;
mod info;
mod torrent;

use std::sync::OnceLock;
use scraper::Selector;

pub use search::*;
pub use info::*;
pub use torrent::*;

pub const ONLINEFIX_BASE_URL: &str = "https://online-fix.me";
pub const ONLINEFIX_GAMES_URL: &str = "https://online-fix.me/games";
pub const ONLINEFIX_UPLOADS_URL: &str = "https://uploads.online-fix.me";
pub const ONLINEFIX_LOGO_URL: &str = "https://online-fix.me/templates/FixLand/images/oflogo.gif";

static ONLINEFIX_SELECTORS: OnceLock<OnlineFixSelector> = OnceLock::new();

struct OnlineFixSelector {
    search_image: Selector,
    search_url: Selector,
    search_title: Selector,
    search_views: Selector,
    search_release: Selector,

    info_build: Selector,
    info_download: Selector
}

fn get_onlinefix_selectors() -> &'static OnlineFixSelector {
    ONLINEFIX_SELECTORS.get_or_init(|| OnlineFixSelector {
        search_image: Selector::parse("img").unwrap(),
        search_url: Selector::parse("a.big-link").unwrap(),
        search_title: Selector::parse("h2.title").unwrap(),
        search_views: Selector::parse(".fa.fa-eye").unwrap(),
        search_release: Selector::parse(".preview-text b").unwrap(),

        info_build: Selector::parse("b").unwrap(),
        info_download: Selector::parse("a.btn").unwrap(),
    })
}