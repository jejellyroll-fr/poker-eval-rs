//! Short Deck (36-card) implementation.
//!
//! Short Deck (also known as 6+ Hold'em) uses a deck where cards 2-5 are removed.

use super::std_deck::StdDeck;
use super::traits::{CardMask, Deck};

/// Number of cards in a short deck (36).
pub const SHORT_DECK_N_CARDS: usize = 36;

/// Short Deck Card Mask (reuses u64 but representing 36 cards).
/// Note: For simplicity and compatibility with existing evaluators,
/// we often map short deck cards back to their standard deck indices.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ShortDeckCardMask {
    pub mask: u64,
}

impl ShortDeckCardMask {
    pub fn new() -> Self {
        Self { mask: 0 }
    }
}

impl CardMask for ShortDeckCardMask {
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
        for i in 0..SHORT_DECK_N_CARDS {
            if self.card_is_set(i) {
                cards.push(ShortDeck::card_to_string(i));
            }
        }
        cards.join(" ")
    }
}

pub struct ShortDeck;

impl ShortDeck {
    /// Maps a 0..35 short deck index to a 0..51 standard deck index.
    /// Short deck cards are 6, 7, 8, 9, T, J, Q, K, A of each suit.
    /// (9 ranks * 4 suits = 36 cards)
    pub fn to_std_index(short_index: usize) -> usize {
        let suit = short_index / 9;
        let rank_offset = short_index % 9; // 0=6, 8=Ace
        suit * 13 + (rank_offset + 4)
    }

    /// Maps a 0..51 standard deck index to a 0..35 short deck index.
    pub fn from_std_index(std_index: usize) -> Option<usize> {
        let suit = std_index / 13;
        let rank = std_index % 13;
        if rank < 4 {
            None // 2, 3, 4, 5 are not in short deck
        } else {
            Some(suit * 9 + (rank - 4))
        }
    }
}

impl Deck for ShortDeck {
    type Mask = ShortDeckCardMask;
    const N_CARDS: usize = SHORT_DECK_N_CARDS;

    fn card_to_string(card_index: usize) -> String {
        let std_idx = Self::to_std_index(card_index);
        StdDeck::card_to_string(std_idx)
    }

    fn string_to_card(card_str: &str) -> Option<usize> {
        let std_idx = StdDeck::string_to_card(card_str)?;
        Self::from_std_index(std_idx)
    }

    fn string_to_mask(cards_str: &str) -> Result<(Self::Mask, usize), String> {
        let mut mask = ShortDeckCardMask::new();
        let mut count = 0;
        for s in cards_str.split_whitespace() {
            if let Some(card) = Self::string_to_card(s) {
                mask.set(card);
                count += 1;
            } else {
                return Err(format!("Invalid short deck card: {}", s));
            }
        }
        Ok((mask, count))
    }
}
