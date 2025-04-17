use serde::{Deserialize, Serialize};
use super::TorrentInfoFile;

#[derive(Serialize, Deserialize)]
pub struct OnlineFixGame {
    pub image: String,
    pub title: String,
    pub url: String,
    pub release_date: String,
    pub views: String,
}

#[derive(Serialize, Deserialize)]
pub struct OnlineFixGameInfo {
    pub url: String,
    pub build: String,
    pub download: String,
}

#[derive(Serialize, Deserialize)]
pub struct OnlineFixTorrent {
    pub url: String,
    pub name: String,
    pub magnet: String,
    pub files: Vec<TorrentInfoFile>,
}

pub struct OnlineFixSearch<'a> {
    pub input: &'a String,
    pub search_url: String,
    pub games: Vec<OnlineFixGame>,
}
