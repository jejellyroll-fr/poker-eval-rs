use crate::deck::JOKER_DECK_RANK_CHARS;
use crate::handval::HandVal;

/// Hand types for games with Jokers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JokerRulesHandType {
    /// High card (no pair).
    NoPair,
    /// One pair.
    OnePair,
    /// Two pair.
    TwoPair,
    /// Three of a kind.
    Trips,
    /// Straight.
    Straight,
    /// Flush.
    Flush,
    /// Full house.
    FullHouse,
    /// Four of a kind.
    Quads,
    /// Straight flush.
    StFlush,
    /// Five of a kind (possible with Joker).
    Quints,
}

/// Human-readable names for Joker hand types.
pub static JOKER_RULES_HAND_TYPE_NAMES: [&str; 10] = [
    "NoPair", "OnePair", "TwoPair", "Trips", "Straight", "Flush", "FlHouse", "Quads", "StFlush",
    "Quints",
];

/// Padded human-readable names for Joker hand types.
pub static JOKER_RULES_HAND_TYPE_NAMES_PADDED: [&str; 10] = [
    "NoPair  ", "OnePair ", "TwoPair ", "Trips   ", "Straight", "Flush   ", "FlHouse ", "Quads   ",
    "StFlush ", "Quints  ",
];

/// Number of significant cards for each Joker hand type.
pub static JOKER_RULES_N_SIG_CARDS: [usize; 10] = [5, 4, 3, 3, 1, 5, 2, 2, 1, 1];

/// Representation of the "Five Straight" for low-value straights.
pub const FIVE_STRAIGHT: u64 = (1 << 14) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5);

impl JokerRulesHandType {
    /// Converts a numerical value to a `JokerRulesHandType`.
    pub fn from_usize(value: usize) -> Option<JokerRulesHandType> {
        match value {
            0 => Some(JokerRulesHandType::NoPair),
            1 => Some(JokerRulesHandType::OnePair),
            2 => Some(JokerRulesHandType::TwoPair),
            3 => Some(JokerRulesHandType::Trips),
            4 => Some(JokerRulesHandType::Straight),
            5 => Some(JokerRulesHandType::Flush),
            6 => Some(JokerRulesHandType::FullHouse),
            7 => Some(JokerRulesHandType::Quads),
            8 => Some(JokerRulesHandType::StFlush),
            9 => Some(JokerRulesHandType::Quints),
            _ => None,
        }
    }
    /// Returns the numerical index of the hand type.
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl HandVal {
    /// Returns the `JokerRulesHandType` associated with this `HandVal`.
    pub fn get_joker_hand_type(&self) -> JokerRulesHandType {
        JokerRulesHandType::from_usize(self.hand_type() as usize)
            .unwrap_or(JokerRulesHandType::NoPair)
    }

    /// Returns a string representation of the hand value using Joker rules.
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

    // Other methods needed for HandVal...
}
