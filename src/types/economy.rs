pub enum CoinsStatus {
    DIED,
    BAD,
    OK,
    GOOD,
    RICH
}

impl CoinsStatus {
    pub fn get(value: u32) -> Self {
        match value {
            0 => Self::DIED,
            _ if value < 500 => Self::BAD,
            _ if value < 2000 => Self::OK,
            _ if value < 10000 => Self::GOOD,
            _ => Self::RICH
        }
    }

    pub fn get_emoji(&self) -> &str {
        match self {
            Self::DIED => ":skull:",
            Self::BAD => ":rofl:",
            Self::OK => ":face_with_hand_over_mouth:",
            Self::GOOD => ":face_with_monocle:",
            Self::RICH => ":astonished:"
        }
    }
}