#![allow(dead_code, unused_imports)]

#[cfg(feature = "onlinefix")]
pub mod onlinefix;
#[macro_use]
pub mod discord {
    pub mod action_row;
    pub mod embed;
    pub mod reply;
}
pub mod redis;
pub mod scraper;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use log::error;

pub fn percent_encode(s: impl AsRef<str>) -> String {
    utf8_percent_encode(s.as_ref(), NON_ALPHANUMERIC).to_string()
}

pub fn format_bytes(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, units[unit_index])
}

pub struct ErrorMessage {
    pub log: Option<String>,
    pub message: String,
}

impl ErrorMessage {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            log: None,
            message: message.into(),
        }
    }

    pub fn with_log(message: impl Into<String>, log: impl Into<String>) -> Self {
        Self {
            log: Some(log.into()),
            message: message.into(),
        }
    }

    pub fn log_error(&self) {
        if let Some(log) = &self.log {
            error!("{log}");
        }
    }
}