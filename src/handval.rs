//! Hand value representations and utilities.
//!
//! This module defines the `HandVal` struct, which encodes poker hand values for efficient comparison.

// Constants for shifts and masks
// Constants for shifts and masks
/// Bit shift for hand type (bits 24-27).
pub(crate) const HANDTYPE_SHIFT: u32 = 24;
/// Bit mask for hand type (0x0F000000).
pub(crate) const HANDTYPE_MASK: u32 = 0x0F000000;
/// Bit shift for top card (bits 16-19).
pub(crate) const TOP_CARD_SHIFT: u32 = 16;
/// Bit mask for top card (0x000F0000).
pub(crate) const TOP_CARD_MASK: u32 = 0x000F0000;
/// Bit shift for second card (bits 12-15).
pub(crate) const SECOND_CARD_SHIFT: u32 = 12;
/// Bit mask for second card (0x0000F000).
pub(crate) const SECOND_CARD_MASK: u32 = 0x0000F000;
/// Bit shift for third card (bits 8-11).
pub(crate) const THIRD_CARD_SHIFT: u32 = 8;
/// Bit mask for third card (0x00000F00).
pub(crate) const THIRD_CARD_MASK: u32 = 0x00000F00;
/// Bit shift for fourth card (bits 4-7).
pub(crate) const FOURTH_CARD_SHIFT: u32 = 4;
/// Bit mask for fourth card (0x000000F0).
pub(crate) const FOURTH_CARD_MASK: u32 = 0x000000F0;
/// Bit shift for fifth card (bits 0-3).
pub(crate) const FIFTH_CARD_SHIFT: u32 = 0;
/// Bit mask for fifth card (0x0000000F).
pub(crate) const FIFTH_CARD_MASK: u32 = 0x0000000F;
/// Width in bits of a single card rank (4).
pub(crate) const CARD_WIDTH: u32 = 4;
/// Mask for a single card rank (0x0F).
pub(crate) const CARD_MASK: u32 = 0x0F;

use serde::{Deserialize, Serialize};

/// Represents the value of a poker hand.
/// Encodes the hand type and significant kickers into a single u32 for efficient comparison.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct HandVal {
    /// The raw integer value of the hand. Higher is better.
    pub value: u32,
}

impl HandVal {
    /// Creates a new `HandVal` from its components.
    #[inline]
    pub const fn new(hand_type: u8, top: u8, second: u8, third: u8, fourth: u8, fifth: u8) -> Self {
        let mut value = ((hand_type as u32) << HANDTYPE_SHIFT) & HANDTYPE_MASK;
        value |= ((top as u32) << TOP_CARD_SHIFT) & TOP_CARD_MASK;
        value |= ((second as u32) << SECOND_CARD_SHIFT) & SECOND_CARD_MASK;
        value |= ((third as u32) << THIRD_CARD_SHIFT) & THIRD_CARD_MASK;
        value |= ((fourth as u32) << FOURTH_CARD_SHIFT) & FOURTH_CARD_MASK;
        value |= ((fifth as u32) << FIFTH_CARD_SHIFT) & FIFTH_CARD_MASK;

        HandVal { value }
    }

    /// Returns the hand type (0-8 for standard rules) as a u8.
    #[inline]
    pub const fn hand_type(&self) -> u8 {
        ((self.value & HANDTYPE_MASK) >> HANDTYPE_SHIFT) as u8
    }

    /// Returns the rank of the top significant card.
    #[inline]
    pub const fn top_card(&self) -> u8 {
        ((self.value & TOP_CARD_MASK) >> TOP_CARD_SHIFT) as u8
    }

    /// Returns the rank of the second significant card.
    pub const fn second_card(&self) -> u8 {
        ((self.value & SECOND_CARD_MASK) >> SECOND_CARD_SHIFT) as u8
    }

    /// Returns the rank of the third significant card.
    pub const fn third_card(&self) -> u8 {
        ((self.value & THIRD_CARD_MASK) >> THIRD_CARD_SHIFT) as u8
    }

    /// Returns the rank of the fourth significant card.
    pub const fn fourth_card(&self) -> u8 {
        ((self.value & FOURTH_CARD_MASK) >> FOURTH_CARD_SHIFT) as u8
    }

    /// Returns the rank of the fifth significant card.
    pub const fn fifth_card(&self) -> u8 {
        ((self.value & FIFTH_CARD_MASK) >> FIFTH_CARD_SHIFT) as u8
    }

    // ... other methods, including card extraction ...
}

impl std::fmt::Display for HandVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_names = [
            "NoPair",
            "OnePair",
            "TwoPair",
            "Trips",
            "Straight",
            "Flush",
            "FullHouse",
            "Quads",
            "StFlush",
        ];
        let ht = self.hand_type() as usize;
        let name = if ht < type_names.len() {
            type_names[ht]
        } else {
            "Unknown"
        };

        write!(f, "{} (", name)?;

        let n_sig = match ht {
            0 => 5, // NoPair
            1 => 4, // OnePair
            2 => 3, // TwoPair
            3 => 3, // Trips
            4 => 1, // Straight
            5 => 5, // Flush
            6 => 2, // FullHouse
            7 => 2, // Quads
            8 => 1, // StFlush
            _ => 0,
        };

        for i in 0..n_sig {
            if i > 0 {
                write!(f, " ")?;
            }
            let rank = match i {
                0 => self.top_card(),
                1 => self.second_card(),
                2 => self.third_card(),
                3 => self.fourth_card(),
                4 => self.fifth_card(),
                _ => 0,
            } as usize;
            let rank_char = "23456789TJQKA".chars().nth(rank).unwrap_or('?');
            write!(f, "{}", rank_char)?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handval_new_and_getters() {
        // Create a hand: type=5 (Flush), top=12 (A), second=11 (K), third=10 (Q), fourth=9 (J), fifth=7 (9)
        let hv = HandVal::new(5, 12, 11, 10, 9, 7);

        assert_eq!(hv.hand_type(), 5);
        assert_eq!(hv.top_card(), 12);
        assert_eq!(hv.second_card(), 11);
        assert_eq!(hv.third_card(), 10);
        assert_eq!(hv.fourth_card(), 9);
        assert_eq!(hv.fifth_card(), 7);
    }

    #[test]
    fn test_handval_comparison_same_type() {
        // Two flushes, first one higher
        let flush_ak = HandVal::new(5, 12, 11, 10, 9, 7); // A-K-Q-J-9
        let flush_kq = HandVal::new(5, 11, 10, 9, 8, 6); // K-Q-J-T-8

        assert!(flush_ak > flush_kq);
        assert!(flush_kq < flush_ak);
    }

    #[test]
    fn test_handval_comparison_different_types() {
        // Flush (5) should beat Straight (4)
        let flush = HandVal::new(5, 10, 9, 8, 7, 5);
        let straight = HandVal::new(4, 12, 0, 0, 0, 0);

        assert!(flush > straight);
    }

    #[test]
    fn test_handval_equality() {
        let hv1 = HandVal::new(3, 10, 8, 0, 0, 0); // Trips
        let hv2 = HandVal::new(3, 10, 8, 0, 0, 0); // Same trips

        assert_eq!(hv1, hv2);
    }

    #[test]
    fn test_handval_zero_cards() {
        // Edge case: all zeros
        let hv = HandVal::new(0, 0, 0, 0, 0, 0);

        assert_eq!(hv.hand_type(), 0);
        assert_eq!(hv.top_card(), 0);
        assert_eq!(hv.value, 0);
    }

    #[test]
    fn test_display_trait() {
        // Full House: Aces full of Kings
        // HandType::FullHouse = 6
        // Top=12 (A), Second=11 (K)
        let hv = HandVal::new(6, 12, 11, 0, 0, 0);

        assert_eq!(hv.to_string(), "FullHouse (A K)");

        // Flush: As Ks Qs Js 9s
        // HandType::Flush = 5
        let hv_flush = HandVal::new(5, 12, 11, 10, 9, 7);
        assert_eq!(hv_flush.to_string(), "Flush (A K Q J 9)");
    }

    #[test]
    fn test_handval_max_values() {
        // Edge case: maximum valid values (hand_type=8, cards=12)
        let hv = HandVal::new(8, 12, 12, 12, 12, 12);

        assert_eq!(hv.hand_type(), 8);
        assert_eq!(hv.top_card(), 12);
    }

    #[test]
    fn test_flush_beats_straight() {
        // HandType::Straight = 4, HandType::Flush = 5
        let straight = HandVal::new(4, 12, 11, 10, 9, 8); // A-high straight
        let flush = HandVal::new(5, 8, 6, 4, 3, 2); // Worst possible flush

        assert!(flush > straight, "Any flush should beat any straight");
    }

    #[test]
    fn test_full_house_beats_flush() {
        // HandType::Flush = 5, HandType::FullHouse = 6
        let flush = HandVal::new(5, 12, 11, 10, 9, 7); // A-K-Q-J-9 flush
        let full_house = HandVal::new(6, 0, 0, 0, 0, 0); // Worst possible full house (2s full)

        assert!(full_house > flush, "Any full house should beat any flush");
    }

    #[test]
    fn test_straight_flush_beats_quads() {
        // HandType::Quads = 7, HandType::StFlush = 8
        let quads = HandVal::new(7, 12, 11, 0, 0, 0); // Quad Aces with K kicker
        let straight_flush = HandVal::new(8, 0, 0, 0, 0, 0); // Worst straight flush

        assert!(
            straight_flush > quads,
            "Any straight flush should beat any four of a kind"
        );
    }
}
