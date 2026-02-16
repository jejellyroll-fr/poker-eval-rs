//! Manila (Asian Stud) 32-card deck implementation.
//!
//! Manila uses a 32-card deck, typically removing cards 2-6.

use super::std_deck::{StdDeck, StdDeckCardMask};
use super::traits::{CardMask, Deck};

/// Number of cards in a Manila deck (32).
pub const MANILA_DECK_N_CARDS: usize = 32;

/// Manila Card Mask.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct ManilaCardMask {
    pub mask: u32,
}

impl ManilaCardMask {
    pub fn new() -> Self {
        Self { mask: 0 }
    }

    pub fn from_raw(mask: u64) -> Self {
        Self { mask: mask as u32 }
    }

    pub fn as_raw(&self) -> u64 {
        self.mask as u64
    }

    pub fn to_std_mask(&self) -> StdDeckCardMask {
        let mut std_mask = StdDeckCardMask::new();
        for i in 0..MANILA_DECK_N_CARDS {
            if self.card_is_set(i) {
                std_mask.set(ManilaDeck::to_std_index(i));
            }
        }
        std_mask
    }
}

impl CardMask for ManilaCardMask {
    fn new() -> Self {
        Self::new()
    }

    fn is_empty(&self) -> bool {
        self.mask == 0
    }

    fn and(&mut self, other: &Self) {
        self.mask &= other.mask;
    }

    fn or(&mut self, other: &Self) {
        self.mask |= other.mask;
    }

    fn xor(&mut self, other: &Self) {
        self.mask ^= other.mask;
    }

    fn not(&mut self) {
        self.mask = !self.mask;
    }

    fn num_cards(&self) -> usize {
        self.mask.count_ones() as usize
    }

    fn card_is_set(&self, index: usize) -> bool {
        (self.mask & (1 << index)) != 0
    }

    fn set(&mut self, index: usize) {
        self.mask |= 1 << index;
    }

    fn mask_to_string(&self) -> String {
        let mut cards = Vec::new();
        for i in 0..MANILA_DECK_N_CARDS {
            if self.card_is_set(i) {
                cards.push(ManilaDeck::card_to_string(i));
            }
        }
        cards.join(" ")
    }
}

pub struct ManilaDeck;

impl ManilaDeck {
    /// Maps a 0..31 Manila deck index to a 0..51 standard deck index.
    /// Manila cards are 7, 8, 9, T, J, Q, K, A of each suit.
    /// (8 ranks * 4 suits = 32 cards)
    pub fn to_std_index(manila_index: usize) -> usize {
        let suit = manila_index / 8;
        let rank_offset = manila_index % 8; // 0=7, 7=Ace
        suit * 13 + (rank_offset + 5)
    }

    /// Maps a 0..51 standard deck index to a 0..31 Manila deck index.
    pub fn from_std_index(std_index: usize) -> Option<usize> {
        let suit = std_index / 13;
        let rank = std_index % 13;
        if rank < 5 {
            None // 2, 3, 4, 5, 6 are not in Manila deck
        } else {
            Some(suit * 8 + (rank - 5))
        }
    }
}

impl Deck for ManilaDeck {
    type Mask = ManilaCardMask;
    const N_CARDS: usize = MANILA_DECK_N_CARDS;

    fn card_to_string(card_index: usize) -> String {
        let std_idx = Self::to_std_index(card_index);
        StdDeck::card_to_string(std_idx)
    }

    fn string_to_card(card_str: &str) -> Option<usize> {
        let std_idx = StdDeck::string_to_card(card_str)?;
        Self::from_std_index(std_idx)
    }

    fn string_to_mask(cards_str: &str) -> Result<(Self::Mask, usize), String> {
        let mut mask = ManilaCardMask::new();
        let mut count = 0;
        for s in cards_str.split_whitespace() {
            if let Some(card) = Self::string_to_card(s) {
                mask.set(card);
                count += 1;
            } else {
                return Err(format!("Invalid Manila card: {}", s));
            }
        }
        Ok((mask, count))
    }
}
