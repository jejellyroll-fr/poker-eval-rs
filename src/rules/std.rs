use crate::handval::HandVal;
// In rules_std.rs
use crate::deck::STD_DECK_RANK_CHARS;

/// Extracts the top five cards from a suit mask (or any card mask).
/// Returns ranks in descending order (top, second, third, fourth, fifth).
/// Optimized to use stack allocation.
#[inline]
pub fn extract_top_five_cards(mask: u16) -> (u8, u8, u8, u8, u8) {
    let mut cards = [0u8; 5];
    let mut count = 0;

    // Iterate through the mask bits to find the cards
    // Iterating high to low to find top cards first
    for i in (0..13).rev() {
        if mask & (1 << i) != 0 {
            cards[count] = i as u8;
            count += 1;
            if count == 5 {
                break;
            }
        }
    }

    (cards[0], cards[1], cards[2], cards[3], cards[4])
}

/// Standard poker hand types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandType {
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
}

/// Human-readable names for hand types.
pub static HAND_TYPE_NAMES: [&str; 9] = [
    "NoPair", "OnePair", "TwoPair", "Trips", "Straight", "Flush", "FlHouse", "Quads", "StFlush",
];

/// Padded human-readable names for hand types (for aligned output).
pub static HAND_TYPE_NAMES_PADDED: [&str; 9] = [
    "NoPair  ", "OnePair ", "TwoPair ", "Trips   ", "Straight", "Flush   ", "FlHouse ", "Quads   ",
    "StFlush ",
];

/// Number of significant cards for each hand type (e.g., 5 for Flush, 2 for Quads).
pub static N_SIG_CARDS: [usize; 9] = [5, 4, 3, 3, 1, 5, 2, 2, 1];

/// Representation of the "Five Straight" (A-2-3-4-5) for low-value straights check.
pub const FIVE_STRAIGHT: u64 = (1 << 14) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5);

impl HandType {
    /// Converts a numerical value to a `HandType`.
    pub fn from_usize(value: usize) -> Option<HandType> {
        match value {
            0 => Some(HandType::NoPair),
            1 => Some(HandType::OnePair),
            2 => Some(HandType::TwoPair),
            3 => Some(HandType::Trips),
            4 => Some(HandType::Straight),
            5 => Some(HandType::Flush),
            6 => Some(HandType::FullHouse),
            7 => Some(HandType::Quads),
            8 => Some(HandType::StFlush),
            _ => None,
        }
    }

    /// Returns the numerical index of the hand type.
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl std::fmt::Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", HAND_TYPE_NAMES[self.as_usize()])
    }
}

impl HandVal {
    /// Returns the `HandType` associated with this `HandVal`.
    pub fn get_hand_type(&self) -> HandType {
        HandType::from_usize(self.hand_type() as usize).unwrap_or(HandType::NoPair)
    }

    /// Returns a string representation of the hand value using standard rules.
    pub fn std_rules_hand_val_to_string(&self) -> String {
        let hand_type = self.get_hand_type();
        let mut result = format!("{} (", HAND_TYPE_NAMES[hand_type.as_usize()]);

        for i in 0..N_SIG_CARDS[hand_type.as_usize()] {
            let card_value = match i {
                0 => self.top_card(),
                1 => self.second_card(),
                2 => self.third_card(),
                3 => self.fourth_card(),
                4 => self.fifth_card(),
                _ => unreachable!(),
            } as usize; // Convert card_value to usize here

            let card_char = STD_DECK_RANK_CHARS.chars().nth(card_value).unwrap_or('?');
            result.push_str(&format!(" {}", card_char));
        }

        result.push(')');
        result
    }

    // Other methods needed for HandVal...
}
