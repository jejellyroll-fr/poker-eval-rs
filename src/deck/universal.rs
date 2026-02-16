//! Universal deck and card mask support.
//!
//! This module provides a unified interface for working with different deck sizes
//! (32, 36, 52, 53) through a single API.

use super::joker::{JokerDeck, JokerDeckCardMask};
use super::manila::{ManilaCardMask, ManilaDeck};
use super::short_deck::{ShortDeck, ShortDeckCardMask};
use super::std_deck::{StdDeck, StdDeckCardMask};
use super::traits::{CardMask, Deck};
use serde::{Deserialize, Serialize};

/// Supported deck variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub enum DeckVariant {
    #[default]
    Standard52,
    Short36,
    Manila32,
    Joker53,
}

/// A card mask that can represent any of the supported decks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UniversalCardMask {
    pub mask: u64,
    pub variant: DeckVariant,
}

impl UniversalCardMask {
    pub fn new(variant: DeckVariant) -> Self {
        Self { mask: 0, variant }
    }

    /// Converts the universal mask to a standard deck mask if possible.
    pub fn to_std_mask(&self) -> Option<StdDeckCardMask> {
        match self.variant {
            DeckVariant::Standard52 => Some(StdDeckCardMask::from_raw(self.mask)),
            DeckVariant::Short36 => {
                let mut std = StdDeckCardMask::new();
                for i in 0..36 {
                    if (self.mask & (1 << i)) != 0 {
                        std.set(ShortDeck::to_std_index(i));
                    }
                }
                Some(std)
            }
            DeckVariant::Manila32 => {
                let mut std = StdDeckCardMask::new();
                for i in 0..32 {
                    if (self.mask & (1 << i)) != 0 {
                        std.set(ManilaDeck::to_std_index(i));
                    }
                }
                Some(std)
            }
            DeckVariant::Joker53 => {
                // If the joker (bit 52) is set, we might not be able to represent it in StdDeck.
                // However, StdDeckCardMask often ignores the joker bit anyway.
                Some(StdDeckCardMask::from_raw(self.mask & 0x000FFFFFFFFFFFFF))
            }
        }
    }
}

impl CardMask for UniversalCardMask {
    fn new() -> Self {
        Self {
            mask: 0,
            variant: DeckVariant::Standard52,
        }
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
        match self.variant {
            DeckVariant::Standard52 => StdDeckCardMask::from_raw(self.mask).card_is_set(index),
            DeckVariant::Short36 => (self.mask & (1 << index)) != 0,
            DeckVariant::Manila32 => (self.mask & (1 << index)) != 0,
            DeckVariant::Joker53 => JokerDeckCardMask::from_raw(self.mask).card_is_set(index),
        }
    }

    fn set(&mut self, index: usize) {
        match self.variant {
            DeckVariant::Standard52 => {
                let m = StdDeckCardMask::from_card_index(index);
                self.mask |= m.as_raw();
            }
            DeckVariant::Short36 => {
                self.mask |= 1 << index;
            }
            DeckVariant::Manila32 => {
                self.mask |= 1 << index;
            }
            DeckVariant::Joker53 => {
                let m = JokerDeckCardMask::get_mask(index);
                self.mask |= m.as_raw();
            }
        }
    }

    fn mask_to_string(&self) -> String {
        match self.variant {
            DeckVariant::Standard52 => StdDeckCardMask::from_raw(self.mask).mask_to_string(),
            DeckVariant::Short36 => ShortDeckCardMask::from_raw(self.mask).mask_to_string(),
            DeckVariant::Manila32 => ManilaCardMask::from_raw(self.mask).mask_to_string(),
            DeckVariant::Joker53 => JokerDeckCardMask::from_raw(self.mask).mask_to_string(),
        }
    }
}

/// A deck implementation that can wrap any of the specific decks.
pub struct UniversalDeck {
    pub variant: DeckVariant,
}

impl UniversalDeck {
    pub fn new(variant: DeckVariant) -> Self {
        Self { variant }
    }
}

impl Deck for UniversalDeck {
    type Mask = UniversalCardMask;
    const N_CARDS: usize = 53; // Maximum possible cards

    fn card_to_string(card_index: usize) -> String {
        StdDeck::card_to_string(card_index)
    }

    fn string_to_card(card_str: &str) -> Option<usize> {
        StdDeck::string_to_card(card_str)
    }

    fn string_to_mask(cards_str: &str) -> Result<(Self::Mask, usize), String> {
        let (m, n) = StdDeck::string_to_mask(cards_str)?;
        Ok((
            UniversalCardMask {
                mask: m.as_raw(),
                variant: DeckVariant::Standard52,
            },
            n,
        ))
    }
}

impl UniversalDeck {
    pub fn variant_to_string(&self, card_index: usize) -> String {
        match self.variant {
            DeckVariant::Standard52 => StdDeck::card_to_string(card_index),
            DeckVariant::Short36 => ShortDeck::card_to_string(card_index),
            DeckVariant::Manila32 => ManilaDeck::card_to_string(card_index),
            DeckVariant::Joker53 => JokerDeck::card_to_string(card_index),
        }
    }

    pub fn variant_string_to_card(&self, card_str: &str) -> Option<usize> {
        match self.variant {
            DeckVariant::Standard52 => StdDeck::string_to_card(card_str),
            DeckVariant::Short36 => ShortDeck::string_to_card(card_str),
            DeckVariant::Manila32 => ManilaDeck::string_to_card(card_str),
            DeckVariant::Joker53 => JokerDeck::string_to_card(card_str),
        }
    }

    pub fn variant_string_to_mask(
        &self,
        cards_str: &str,
    ) -> Result<(UniversalCardMask, usize), String> {
        match self.variant {
            DeckVariant::Standard52 => {
                let (m, n) = StdDeck::string_to_mask(cards_str)?;
                Ok((
                    UniversalCardMask {
                        mask: m.as_raw(),
                        variant: DeckVariant::Standard52,
                    },
                    n,
                ))
            }
            DeckVariant::Short36 => {
                let (m, n) = ShortDeck::string_to_mask(cards_str)?;
                Ok((
                    UniversalCardMask {
                        mask: m.as_raw(),
                        variant: DeckVariant::Short36,
                    },
                    n,
                ))
            }
            DeckVariant::Manila32 => {
                let (m, n) = ManilaDeck::string_to_mask(cards_str)?;
                Ok((
                    UniversalCardMask {
                        mask: m.as_raw(),
                        variant: DeckVariant::Manila32,
                    },
                    n,
                ))
            }
            DeckVariant::Joker53 => {
                let (m, n) = JokerDeck::string_to_mask(cards_str)?;
                Ok((
                    UniversalCardMask {
                        mask: m.as_raw(),
                        variant: DeckVariant::Joker53,
                    },
                    n,
                ))
            }
        }
    }
}
