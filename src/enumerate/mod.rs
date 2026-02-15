//! Card enumeration and combination utilities for poker hand analysis.
//! Contains internal helper functions for exhaustive and Monte Carlo enumeration.

mod card_enum;
mod card_enum_dead;

pub mod evaluation;
pub mod game_params;
pub mod inner_loops;
mod montecarlo;
pub mod qmc;
pub mod result;

pub use crate::errors::PokerError;
pub use evaluation::*;
pub use inner_loops::*;

use crate::enumdefs::{Game, GameParams};
use crate::tables::t_cardmasks::StdDeckCardMask;
use crate::tables::t_jokercardmasks::JokerDeckCardMask;
use std::ops::BitOr;

/// Trait for abstracting over card mask types (standard deck and joker deck).
///
/// Provides common operations needed for card enumeration: bitmask access,
/// overlap detection, and string conversion.
pub trait CardMask: BitOr<Output = Self> + Copy + PartialEq + std::fmt::Debug {
    /// Returns the raw 64-bit bitmask representing this card or card set.
    fn mask(&self) -> u64;
    /// Returns `true` if this mask shares any bits with `other`.
    fn overlaps(&self, other: &Self) -> bool;
    /// Returns a debug string representation of the mask.
    fn to_debug_string(&self) -> String;
    /// Returns a human-readable string representation (e.g., `"AhKd"`).
    fn to_string_representation(&self) -> String;
}

impl CardMask for StdDeckCardMask {
    fn mask(&self) -> u64 {
        self.as_raw()
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.as_raw() & other.as_raw()) != 0
    }
    fn to_debug_string(&self) -> String {
        format!("StdDeckCardMask: mask={:#066b}", self.as_raw())
    }
    fn to_string_representation(&self) -> String {
        self.mask_to_string()
    }
}

impl CardMask for JokerDeckCardMask {
    fn mask(&self) -> u64 {
        self.cards_n
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.cards_n & other.cards_n) != 0
    }
    fn to_debug_string(&self) -> String {
        format!("JokerDeckCardMask: cards_n={:#066b}", self.cards_n)
    }
    fn to_string_representation(&self) -> String {
        self.mask_to_string()
    }
}

impl BitOr for StdDeckCardMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        StdDeckCardMask::from_raw(self.as_raw() | rhs.as_raw())
    }
}

impl BitOr for JokerDeckCardMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        JokerDeckCardMask {
            cards_n: self.cards_n | rhs.cards_n,
        }
    }
}

impl Game {
    /// Returns the `GameParams` for this game variant, or `None` if unsupported.
    pub fn game_params(self) -> Option<GameParams> {
        match self {
            Game::Holdem => Some(GameParams {
                game: Game::Holdem,
                minpocket: 2,
                maxpocket: 2,
                maxboard: 5,
                haslopot: 0,
                hashipot: 1,
                name: "Holdem Hi",
            }),
            Game::Holdem8 => Some(GameParams {
                game: Game::Holdem8,
                minpocket: 2,
                maxpocket: 2,
                maxboard: 5,
                haslopot: 1,
                hashipot: 1,
                name: "Holdem Hi/Low 8-or-better",
            }),
            Game::Omaha => Some(GameParams {
                game: Game::Omaha,
                minpocket: 4,
                maxpocket: 4,
                maxboard: 5,
                haslopot: 0,
                hashipot: 1,
                name: "Omaha Hi",
            }),
            Game::Omaha5 => Some(GameParams {
                game: Game::Omaha5,
                minpocket: 5,
                maxpocket: 5,
                maxboard: 5,
                haslopot: 0,
                hashipot: 1,
                name: "Omaha 5cards Hi",
            }),
            Game::Omaha6 => Some(GameParams {
                game: Game::Omaha6,
                minpocket: 6,
                maxpocket: 6,
                maxboard: 5,
                haslopot: 0,
                hashipot: 1,
                name: "Omaha 6cards Hi",
            }),
            Game::Omaha8 => Some(GameParams {
                game: Game::Omaha8,
                minpocket: 4,
                maxpocket: 4,
                maxboard: 5,
                haslopot: 1,
                hashipot: 1,
                name: "Omaha Hi/Low 8-or-better",
            }),
            Game::Omaha85 => Some(GameParams {
                game: Game::Omaha85,
                minpocket: 5,
                maxpocket: 5,
                maxboard: 5,
                haslopot: 1,
                hashipot: 1,
                name: "Omaha 5cards Hi/Low",
            }),
            Game::Stud7 => Some(GameParams {
                game: Game::Stud7,
                minpocket: 3,
                maxpocket: 7,
                maxboard: 0,
                haslopot: 0,
                hashipot: 1,
                name: "Stud 7cards Hi",
            }),
            Game::Stud78 => Some(GameParams {
                game: Game::Stud78,
                minpocket: 3,
                maxpocket: 7,
                maxboard: 0,
                haslopot: 1,
                hashipot: 1,
                name: "Stud 7cards Hi/Low",
            }),
            Game::Stud7nsq => Some(GameParams {
                game: Game::Stud7nsq,
                minpocket: 3,
                maxpocket: 7,
                maxboard: 0,
                haslopot: 1,
                hashipot: 1,
                name: "Stud 7cards Hi/Low no qualifier",
            }),
            Game::Razz => Some(GameParams {
                game: Game::Razz,
                minpocket: 3,
                maxpocket: 7,
                maxboard: 0,
                haslopot: 1,
                hashipot: 0,
                name: "Razz (7-card Stud A-5 Low)",
            }),
            Game::Draw5 => Some(GameParams {
                game: Game::Draw5,
                minpocket: 0,
                maxpocket: 5,
                maxboard: 0,
                haslopot: 0,
                hashipot: 1,
                name: "5-card Draw Hi with joker",
            }),
            Game::Draw58 => Some(GameParams {
                game: Game::Draw58,
                minpocket: 0,
                maxpocket: 5,
                maxboard: 0,
                haslopot: 1,
                hashipot: 1,
                name: "5-card Draw Hi/Low 8-or-better with joker",
            }),
            Game::Draw5nsq => Some(GameParams {
                game: Game::Draw5nsq,
                minpocket: 0,
                maxpocket: 5,
                maxboard: 0,
                haslopot: 1,
                hashipot: 1,
                name: "5-card Draw Hi/Low no qualifier with joker",
            }),
            Game::Lowball => Some(GameParams {
                game: Game::Lowball,
                minpocket: 0,
                maxpocket: 5,
                maxboard: 0,
                haslopot: 1,
                hashipot: 0,
                name: "5-card Draw A-5 Lowball with joker",
            }),
            Game::Lowball27 => Some(GameParams {
                game: Game::Lowball27,
                minpocket: 0,
                maxpocket: 5,
                maxboard: 0,
                haslopot: 1,
                hashipot: 0,
                name: "5-card Draw 2-7 Lowball",
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::card_enum::*;
    use super::card_enum_dead::*;
    // use crate::combinations::*; // Unused
    use super::*;
    use crate::deck::STD_DECK_N_CARDS;
    use crate::tables::t_cardmasks::STD_DECK_CARD_MASKS_TABLE;

    #[test]
    fn test_cardmask_trait_mask() {
        let mask = StdDeckCardMask::from_raw(0b1010);
        assert_eq!(CardMask::mask(&mask), 0b1010);
    }

    #[test]
    fn test_cardmask_overlaps_true() {
        let mask1 = StdDeckCardMask::from_raw(0b1010);
        let mask2 = StdDeckCardMask::from_raw(0b1100);
        assert!(mask1.overlaps(&mask2)); // bit 3 overlaps
    }

    #[test]
    fn test_cardmask_overlaps_false() {
        let mask1 = StdDeckCardMask::from_raw(0b0101);
        let mask2 = StdDeckCardMask::from_raw(0b1010);
        assert!(!mask1.overlaps(&mask2));
    }

    #[test]
    fn test_enumerate_1_cards_count() {
        let deck: Vec<StdDeckCardMask> =
            (0..5).map(|i| StdDeckCardMask::from_raw(1 << i)).collect();
        let mut count = 0;
        enumerate_1_cards(&deck, |_| count += 1);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_enumerate_2_cards_count() {
        let deck: Vec<StdDeckCardMask> =
            (0..5).map(|i| StdDeckCardMask::from_raw(1 << i)).collect();
        let mut count = 0;
        enumerate_2_cards(&deck, |_, _| count += 1);
        // C(5,2) = 10
        assert_eq!(count, 10);
    }

    #[test]
    fn test_enumerate_5_cards_count() {
        let deck: Vec<StdDeckCardMask> =
            (0..7).map(|i| StdDeckCardMask::from_raw(1 << i)).collect();
        let mut count = 0;
        enumerate_5_cards(&deck, |_, _, _, _, _| count += 1);
        // C(7,5) = 21
        assert_eq!(count, 21);
    }

    #[test]
    fn test_enumerate_2_cards_d_with_dead() {
        let deck: Vec<StdDeckCardMask> =
            (0..5).map(|i| StdDeckCardMask::from_raw(1 << i)).collect();
        let dead = StdDeckCardMask::from_raw(1 << 0); // card 0 is dead
        let mut count = 0;
        enumerate_2_cards_d(&deck, dead, |_, _| count += 1);
        // C(4,2) = 6 (excluding one card)
        assert_eq!(count, 6);
    }

    #[test]
    fn test_enumerate_n_cards_d_c52_5() {
        let deck: Vec<StdDeckCardMask> = STD_DECK_CARD_MASKS_TABLE.to_vec();
        let dead = StdDeckCardMask::new();
        let mut count = 0;
        enumerate_n_cards_d(&deck, dead, 5, |_| count += 1);
        // C(52,5) = 2,598,960
        assert_eq!(count, 2598960);
    }

    #[test]
    fn test_enumerate_n_cards_d_with_dead_cards() {
        // Use a small deck to avoid performance issues with enumerate_n_cards_d
        let deck: Vec<StdDeckCardMask> =
            (0..6).map(|i| StdDeckCardMask::from_raw(1 << i)).collect();
        let dead = deck[0]; // card 0 is dead
        let mut count = 0;
        enumerate_n_cards_d(&deck, dead, 2, |_| count += 1);
        // C(5,2) = 10 (6 cards - 1 dead = 5 remaining)
        assert_eq!(count, 10);
    }

    #[test]
    fn test_bitor_stddeckcardmask() {
        let mask1 = StdDeckCardMask::from_raw(0b0101);
        let mask2 = StdDeckCardMask::from_raw(0b1010);
        let result = mask1 | mask2;
        assert_eq!(result.as_raw(), 0b1111);
    }

    #[test]
    fn test_std_deck_size() {
        assert_eq!(STD_DECK_N_CARDS, 52);
        assert_eq!(STD_DECK_CARD_MASKS_TABLE.len(), 52);
    }
}
