use rand::seq::SliceRandom;
use std::cmp::Ordering;

const CARD_BACK_EMOJI: &str = "<:carta:1234868369031827529>";

#[derive(PartialEq)]
pub enum BlackjackResult {
    Unfinished,
    Win,
    Lose,
    Tie,
}

#[derive(PartialEq)]
pub enum BlackjackCard {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
impl BlackjackCard {
    pub fn get_card(&self) -> (u8, &str) {
        match self {
            Self::Ace => (1, "<:A_card:1232268148091125883>"),
            Self::Two => (2, "<:2_card:1232669305691176991>"),
            Self::Three => (3, "<:3_card:1232273259186225153>"),
            Self::Four => (4, "<:4_card:1232769913421299795>"),
            Self::Five => (5, "<:5_card:1232657418341842987>"),
            Self::Six => (6, "<:6_card:1232417467456950392>"),
            Self::Seven => (7, "<:7_card:1232468927175589989>"),
            Self::Eight => (8, "<:8_card:1232805117225472076>"),
            Self::Nine => (9, "<:9_card:1232471210726522953>"),
            Self::Ten => (10, "<:10_card:1232677832316944384>"),
            Self::Jack => (10, "<:J_card:1232674182303715338>"),
            Self::Queen => (10, "<:Q_card:1232285671352307753>"),
            Self::King => (10, "<:K_card:1232810402652491868>"),
        }
    }

    pub fn is_ace(&self) -> bool {
        matches!(self, Self::Ace)
    }
}

const ALL_CARDS: [BlackjackCard; 13] = [
    BlackjackCard::Ace,
    BlackjackCard::Two,
    BlackjackCard::Three,
    BlackjackCard::Four,
    BlackjackCard::Five,
    BlackjackCard::Six,
    BlackjackCard::Seven,
    BlackjackCard::Eight,
    BlackjackCard::Nine,
    BlackjackCard::Ten,
    BlackjackCard::Jack,
    BlackjackCard::Queen,
    BlackjackCard::King,
];

pub struct BlackjackHand {
    cards: Vec<BlackjackCard>,
    pub value: u8,
}
impl BlackjackHand {
    pub fn new() -> Self {
        Self {
            cards: vec![],
            value: 0,
        }
    }

    pub fn take_card(&mut self, deck: &mut Vec<BlackjackCard>) {
        let card = deck.pop().unwrap();
        self.cards.push(card);
    
        let mut val: u8 = 0;
        let mut aces = 0;
    
        for card in &self.cards {
            if card.is_ace() {
                val += 11;
                aces += 1;
            } else {
                val += card.get_card().0;
            }
        }
    
        while val > 21 && aces > 0 {
            val -= 10;
            aces -= 1;
        }
    
        self.value = val;
    }

    pub fn get_field(&self) -> String {
        if self.cards.len() > 1 {
            let cards: String = self.cards.iter().map(|c| c.get_card().1).collect();
    
            return format!("{} ({})", cards, self.value);
        }

        format!(
            "{}{} ({})",
            self.cards[0].get_card().1,
            CARD_BACK_EMOJI,
            self.value
        )
    }
}

pub struct BlackjackGame {
    pub player_hand: BlackjackHand,
    pub dealer_hand: BlackjackHand,
    pub result: BlackjackResult,
    deck: Vec<BlackjackCard>,
}
impl BlackjackGame {
    pub fn new(decks: u8) -> Self {
        let mut deck: Vec<BlackjackCard> = (0..(decks as u16) * 4).flat_map(|_| ALL_CARDS).collect();

        deck.shuffle(&mut rand::thread_rng());

        let mut dealer_hand = BlackjackHand::new();
        dealer_hand.take_card(&mut deck);

        let mut game = Self {
            player_hand: BlackjackHand::new(),
            dealer_hand,
            result: BlackjackResult::Unfinished,
            deck,
        };

        game.take_card();
        game.take_card();

        game
    }

    pub fn is_finished(&self) -> bool {
        self.result != BlackjackResult::Unfinished
    }

    pub fn take_card(&mut self) {
        self.player_hand.take_card(&mut self.deck);

        if self.player_hand.value >= 21 {
            self.finish();
        }
    }

    pub fn dealer_turn(&mut self) {
        while self.dealer_hand.value < 17 {
            self.dealer_hand.take_card(&mut self.deck);
        }

        self.finish();
    }

    fn finish(&mut self) {
        self.result = match self.player_hand.value.cmp(&self.dealer_hand.value) {
            Ordering::Equal => BlackjackResult::Tie,
            Ordering::Greater if self.player_hand.value <= 21 => BlackjackResult::Win,
            Ordering::Less if self.dealer_hand.value > 21 => BlackjackResult::Win,
            _ => BlackjackResult::Lose,
        };
    }
}