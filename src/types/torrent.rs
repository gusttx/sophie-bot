use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TorrentInfo {
    pub name: String,
    pub files: Vec<TorrentInfoFile>,
}

#[derive(Deserialize, Serialize)]
pub struct TorrentInfoFile {
    pub length: u64,
    pub path: Vec<String>,
}

#[derive(Deserialize)]
pub struct TorrentContent {
    pub info: TorrentInfo,
    #[serde(rename = "announce-list")]
    pub announce_list: Vec<Vec<String>>,
}