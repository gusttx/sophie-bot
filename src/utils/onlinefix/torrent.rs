use crate::{
    config::get_config,
    types::{OnlineFixTorrent, TorrentContent},
    utils::{
        percent_encode,
        redis::Redis,
        scraper::{Scraper, ScraperError},
        ErrorMessage,
    },
};
use reqwest::Client;
use serde_bencode::{de, ser, value::Value as BencodeValue};
use sha1::{Digest, Sha1};
use super::ONLINEFIX_UPLOADS_URL;

const INFO_KEY: &[u8] = b"info";

pub async fn torrent(
    client: &Client,
    redis: &Redis,
    path: &str,
) -> Result<OnlineFixTorrent, ErrorMessage> {
    let torrent_redis_key = format!("torrent:{}", path);

    if let Some(torrent) = redis.get::<OnlineFixTorrent>(&torrent_redis_key).await {
        return Ok(torrent);
    }

    let config = get_config();
    let torrent_ttl = config.cache.onlinefix_torrent_ttl;

    let base_url = &format!("{}{}", ONLINEFIX_UPLOADS_URL, path);

    let torrent_path = get_torrent_path(client, base_url).await?;
    let download_url = format!("{}{}", base_url, torrent_path);
    
    let torrent_bytes = client
        .get(&download_url)
        .send()
        .await
        .map_err(|err| {
            ErrorMessage::with_log(
                "Ocorreu um erro ao baixar o torrent :t_rex:",
                format!("Erro ao baixar torrent de `{}`: {}", download_url, err),
            )
        })?
        .bytes()
        .await
        .map_err(|err| {
            ErrorMessage::with_log(
                "Ocorreu um erro ao processar o torrent :t_rex:",
                format!(
                    "Erro ao ler bytes do torrent de `{}`: {}",
                    download_url, err
                ),
            )
        })?;

    let onlinefix_torrent = extract_torrent_info(&torrent_bytes, download_url)?;
    redis.set(&torrent_redis_key, &onlinefix_torrent, torrent_ttl).await;

    Ok(onlinefix_torrent)
}

async fn get_torrent_path(client: &Client, url: &str) -> Result<String, ErrorMessage> {
    let req = client.get(url);

    Scraper::new(req)
        .set_root_element("a")
        .get(|select| {
            select
                .skip(1)
                .next()
                .and_then(|element| element.attr("href").map(String::from))
                .ok_or(ScraperError::ElementNotFound)
        })
        .await
        .and_then(|inner| inner)
        .map_err(|err| map_torrent_error(err, url))
}

fn extract_torrent_info(bytes: &[u8], url: String) -> Result<OnlineFixTorrent, ErrorMessage> {
    let content: TorrentContent = de::from_bytes(bytes).map_err(|err| {
        ErrorMessage::with_log(
            "Ocorreu um erro ao processar o torrent :sauropod:",
            format!(
                "Erro ao deserializar conteúdo do torrent de `{}`: {}",
                url, err
            ),
        )
    })?;

    let info_hash = calculate_info_hash(bytes, &url)?;

    let trackers = content
        .announce_list
        .iter()
        .flatten()
        .map(|tracker| format!("&tr={}", percent_encode(tracker)))
        .collect::<String>();

    let magnet = format!(
        "magnet:?xt=urn:btih:{}&dn={}{}",
        info_hash,
        percent_encode(&content.info.name),
        trackers
    );

    Ok(OnlineFixTorrent {
        url: url,
        magnet,
        name: content.info.name,
        files: content.info.files,
    })
}

fn calculate_info_hash(bytes: &[u8], url: &str) -> Result<String, ErrorMessage> {
    let bencode_value: BencodeValue = de::from_bytes(bytes).map_err(|err| {
        ErrorMessage::with_log(
            "Ocorreu um erro ao processar o torrent :sauropod:",
            format!("Erro ao parsear bencode de `{}`: {}", url, err),
        )
    })?;

    let BencodeValue::Dict(bencode_dict) = bencode_value else {
        return Err(ErrorMessage::with_log(
            "[E1] Torrent com formato inválido :sauropod:",
            format!("Valor bencode não é um dicionário em `{}`", url),
        ))
    };

    let Some(info_section) = bencode_dict.get(INFO_KEY) else {
        return Err(ErrorMessage::with_log(
            "[E2] Torrent com formato inválido :sauropod:",
            format!("Seção 'info' não encontrada no torrent de `{}`", url),
        ))
    };

    let info_bytes = ser::to_bytes(info_section).map_err(|err| {
        ErrorMessage::with_log(
            "Ocorreu um erro ao processar o torrent :sauropod:",
            format!("Erro ao serializar seção 'info' de `{}`: {}", url, err),
        )
    })?;

    let mut hasher = Sha1::new();
    hasher.update(&info_bytes);
    let hash_result = hasher.finalize();

    let hex_hash = hash_result
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect();

    Ok(hex_hash)
}

fn map_torrent_error(err: ScraperError, url: &str) -> ErrorMessage {
    match err {
        ScraperError::ElementNotFound | ScraperError::ElementNotFoundWithId(_) => {
            ErrorMessage::with_log(
                "Não foi possível obter o torrent do jogo :parrot:",
                format!("Failed to get torrent link from `{}`", url),
            )
        }
        ScraperError::JoinError(err) => ErrorMessage::with_log(
            "Ocorreu um erro interno :pleading_face:",
            format!("Failed to join tasks: {}", err),
        ),
        ScraperError::ResponseError(err) => ErrorMessage::with_log(
            "Não foi possível obter resposta para o torrent :pensive:",
            format!("Failed to get response for torrent `{}`: {}", url, err),
        ),
    }
}
