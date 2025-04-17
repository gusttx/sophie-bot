use rand::Rng;

pub enum JankenponResult {
    Win,
    Tie,
    Lose,
}

#[derive(poise::ChoiceParameter, Copy, Clone)]
pub enum JankenponChoice {
    #[name = "👊 Pedra"]
    Rock,
    #[name = "🖐 Papel"]
    Paper,
    #[name = "✌ Tesoura"]
    Scissors,
}

impl JankenponChoice {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..3) {
            0 => JankenponChoice::Rock,
            1 => JankenponChoice::Paper,
            _ => JankenponChoice::Scissors,
        }
    }

    pub fn parse(s: impl AsRef<str>) -> Self {
        match s.as_ref() {
            "rock" => JankenponChoice::Rock,
            "paper" => JankenponChoice::Paper,
            _ => JankenponChoice::Scissors,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Self::Rock => "pedra",
            Self::Paper => "papel",
            Self::Scissors => "tesoura"
        }
    }

    pub fn get_emoji(&self) -> &str {
        match self {
            Self::Rock => ":fist:",
            Self::Paper => ":hand_splayed:",
            Self::Scissors => ":v:",
        }
    }

    pub fn compare(&self, other: Self) -> JankenponResult {
        match (self, other) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => JankenponResult::Win,

            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => JankenponResult::Lose,

            _ => JankenponResult::Tie,
        }
    }
}