use crate::deck_joker::JOKER_DECK_RANK_CHARS;
use crate::handval::HandVal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JokerRulesHandType {
    NoPair,
    OnePair,
    TwoPair,
    Trips,
    Straight,
    Flush,
    FullHouse,
    Quads,
    StFlush,
    Quints,
}

pub static JOKER_RULES_HAND_TYPE_NAMES: [&str; 10] = [
    "NoPair", "OnePair", "TwoPair", "Trips", "Straight", "Flush", "FlHouse", "Quads", "StFlush",
    "Quints",
];

pub static JOKER_RULES_HAND_TYPE_NAMES_PADDED: [&str; 10] = [
    "NoPair  ", "OnePair ", "TwoPair ", "Trips   ", "Straight", "Flush   ", "FlHouse ", "Quads   ",
    "StFlush ", "Quints  ",
];

pub static JOKER_RULES_N_SIG_CARDS: [usize; 10] = [5, 4, 3, 3, 1, 5, 2, 2, 1, 1];

// Représentation du "Five Straight" pour les straights de basse valeur.
pub const FIVE_STRAIGHT: u64 = (1 << 14) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5);

impl JokerRulesHandType {
    // Cette méthode doit retourner un HandType
    pub fn from_usize(value: usize) -> JokerRulesHandType {
        match value {
            0 => JokerRulesHandType::NoPair,
            1 => JokerRulesHandType::OnePair,
            2 => JokerRulesHandType::TwoPair,
            3 => JokerRulesHandType::Trips,
            4 => JokerRulesHandType::Straight,
            5 => JokerRulesHandType::Flush,
            6 => JokerRulesHandType::FullHouse,
            7 => JokerRulesHandType::Quads,
            8 => JokerRulesHandType::StFlush,
            9 => JokerRulesHandType::Quints,
            _ => panic!("Invalid hand type value"),
        }
    }
    // Cette méthode retourne l'index de l'enum HandType
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl HandVal {
    pub fn get_joker_hand_type(&self) -> JokerRulesHandType {
        JokerRulesHandType::from_usize(self.hand_type() as usize)
    }

    pub fn joker_rules_hand_val_to_string(&self) -> String {
        let hand_type = self.get_joker_hand_type();
        let mut result = format!("{} (", JOKER_RULES_HAND_TYPE_NAMES[hand_type.as_usize()]);

        for i in 0..JOKER_RULES_N_SIG_CARDS[hand_type.as_usize()] {
            let card_value = match i {
                0 => self.top_card(),
                1 => self.second_card(),
                2 => self.third_card(),
                3 => self.fourth_card(),
                4 => self.fifth_card(),
                _ => unreachable!(),
            } as usize;

            let card_char = JOKER_DECK_RANK_CHARS.chars().nth(card_value).unwrap_or('?');
            result.push_str(&format!(" {}", card_char));
        }

        result.push(')');
        result
    }

    pub fn joker_rules_hand_val_print(&self) {
        println!("{}", self.joker_rules_hand_val_to_string());
    }

    // Autres méthodes nécessaires pour HandVal...
}
