//! Lowball hand value representations and constants.
//!
//! This module defines the `LowHandVal` struct used for storing evaluation results
//! in Lowball variants (2-7, A-5, Razz, Stud8 Low).

use crate::deck::STD_DECK_RANK_ACE;
use crate::rules::HandType;

// Constants for shifts and masks
//
// The LowHandVal packs the HandType and the top 5 cards into a single u32 integer for efficient comparison.
// Structure (MSB to LSB):
// [31..28]: Unused/Reserved
// [27..24]: HandType (4 bits) - e.g., LowNoPair, LowFlush
// [23..20]: Unused/Reserved
// [19..16]: Top Card Rank (4 bits)
// [15..12]: Second Card Rank (4 bits)
// [11..8] : Third Card Rank (4 bits)
// [7..4]  : Fourth Card Rank (4 bits)
// [3..0]  : Fifth Card Rank (4 bits)
pub(crate) const HANDTYPE_SHIFT: u32 = 24;
pub(crate) const HANDTYPE_MASK: u32 = 0x0F000000;
pub(crate) const TOP_CARD_SHIFT: u32 = 16;
pub(crate) const TOP_CARD_MASK: u32 = 0x000F0000;
pub(crate) const SECOND_CARD_SHIFT: u32 = 12;
pub(crate) const SECOND_CARD_MASK: u32 = 0x0000F000;
pub(crate) const THIRD_CARD_SHIFT: u32 = 8;
pub(crate) const THIRD_CARD_MASK: u32 = 0x00000F00;
pub(crate) const FOURTH_CARD_SHIFT: u32 = 4;
pub(crate) const FOURTH_CARD_MASK: u32 = 0x000000F0;
pub(crate) const FIFTH_CARD_SHIFT: u32 = 0;
pub(crate) const FIFTH_CARD_MASK: u32 = 0x0000000F;
// Constants defined directly without using functions
pub const LOW_HAND_VAL_NOTHING: u32 =
    (HandType::StFlush as u32) << HANDTYPE_SHIFT | (STD_DECK_RANK_ACE as u32 + 1) << TOP_CARD_SHIFT;
// In Lowball eval, Ranks are rotated so A=0 (Value 1), 2=1 (Value 2)... 8=7 (Value 8).
// So 8-high means Top Card Value = 8.
// WORST_EIGHT is 8-7-6-5-4. Values: 8, 7, 6, 5, 4.
pub const LOW_HAND_VAL_WORST_EIGHT: u32 = (HandType::NoPair as u32) << HANDTYPE_SHIFT
    | 8 << TOP_CARD_SHIFT
    | 7 << SECOND_CARD_SHIFT
    | 6 << THIRD_CARD_SHIFT
    | 5 << FOURTH_CARD_SHIFT
    | 4 << FIFTH_CARD_SHIFT;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LowHandVal {
    pub value: u32,
}

impl LowHandVal {
    pub fn new(hand_type: u8, top: u8, second: u8, third: u8, fourth: u8, fifth: u8) -> Self {
        let mut value = ((hand_type as u32) << HANDTYPE_SHIFT) & HANDTYPE_MASK;
        value |= ((top as u32) << TOP_CARD_SHIFT) & TOP_CARD_MASK;
        value |= ((second as u32) << SECOND_CARD_SHIFT) & SECOND_CARD_MASK;
        value |= ((third as u32) << THIRD_CARD_SHIFT) & THIRD_CARD_MASK;
        value |= ((fourth as u32) << FOURTH_CARD_SHIFT) & FOURTH_CARD_MASK;
        value |= ((fifth as u32) << FIFTH_CARD_SHIFT) & FIFTH_CARD_MASK;

        LowHandVal { value }
    }

    // Extraction methods
    pub fn hand_type(&self) -> u8 {
        ((self.value & HANDTYPE_MASK) >> HANDTYPE_SHIFT) as u8
    }

    pub fn top_card(&self) -> u8 {
        ((self.value & TOP_CARD_MASK) >> TOP_CARD_SHIFT) as u8
    }
    pub fn second_card(&self) -> u8 {
        ((self.value & SECOND_CARD_MASK) >> SECOND_CARD_SHIFT) as u8
    }

    pub fn third_card(&self) -> u8 {
        ((self.value & THIRD_CARD_MASK) >> THIRD_CARD_SHIFT) as u8
    }

    pub fn fourth_card(&self) -> u8 {
        ((self.value & FOURTH_CARD_MASK) >> FOURTH_CARD_SHIFT) as u8
    }

    pub fn fifth_card(&self) -> u8 {
        ((self.value & FIFTH_CARD_MASK) >> FIFTH_CARD_SHIFT) as u8
    }

    // Implement Display trait instead of to_string
}

impl std::fmt::Display for LowHandVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hand_type_str = match self.hand_type() {
            0 => "Low NoPair",
            1 => "No Low OnePair",
            2 => "No Low TwoPair",
            3 => "No Low Trips",
            4 => "No Low Straight",
            5 => "No Low Flush",
            6 => "No Low FullHouse",
            7 => "No Low Quads",
            8 => "No Low StFlush",
            _ => "Unknown",
        };

        write!(
            f,
            "{} ( {} {} {} {} {})",
            hand_type_str,
            self.top_card(),
            self.second_card(),
            self.third_card(),
            self.fourth_card(),
            self.fifth_card()
        )
    }
}

impl LowHandVal {
    // Method to rotate ranks (for Ace handling in Omaha Hi/Lo)
    pub fn rotate_ranks(ranks: u32) -> u32 {
        let ace_bit = (ranks >> STD_DECK_RANK_ACE) & 0x01; // Isolate the Ace bit and shift it to the right
        let without_ace = ranks & !(1 << STD_DECK_RANK_ACE); // Remove the Ace bit from the original mask
        let shifted = without_ace << 1; // Shift all other bits up by one position
        shifted | ace_bit // Combine the shifted Ace bit with the other shifted bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_hand_val_new_and_getters() {
        // Create a 7-5-4-3-2 NoPair hand
        // HandType 0 (Low NoPair)
        // Ranks: 2=1, 3=2, 4=3, 5=4, 7=6. (Assuming 0-indexed internal logic?
        // Wait, comments say "Ranks are rotated so A=0". 2=1... 7=6.
        // Let's use arbitrary values to test bit packing.
        let val = LowHandVal::new(0, 6, 4, 3, 2, 1);

        assert_eq!(val.hand_type(), 0);
        assert_eq!(val.top_card(), 6);
        assert_eq!(val.second_card(), 4);
        assert_eq!(val.third_card(), 3);
        assert_eq!(val.fourth_card(), 2);
        assert_eq!(val.fifth_card(), 1);
    }

    #[test]
    fn test_low_hand_val_display() {
        // HandType 1 = "No Low OnePair"
        let val = LowHandVal::new(1, 10, 8, 6, 4, 2);
        let s = format!("{}", val);
        assert!(s.contains("No Low OnePair"));
        assert!(s.contains("10 8 6 4 2"));
    }

    #[test]
    fn test_low_hand_val_ordering() {
        // Lower value is BETTER in valid lowball usually, but here `value` is u32.
        // The implementation packs it.
        // Typically for "High" evaluation, larger u32 = better hand.
        // For Lowball:
        // HandType 0 (NoPair) is better than HandType 1 (OnePair).
        // Since we want standard behavior (impl PartialOrd), distinct values will compare naturally.
        // IF the library treats "Value" as "Score" where higher is better, then HandType 0 should map to a high score?
        // OR the evaluator returns an inverted score.
        // LowHandVal struct just wraps u32. comparisons verify the u32 order.
        // HandType is stored in MSB.
        // 0 at MSB < 1 at MSB.

        let v1 = LowHandVal::new(0, 5, 4, 3, 2, 1); // "Better" hand logic wise (Wheel)
        let v2 = LowHandVal::new(1, 5, 4, 3, 2, 1); // Pair

        // v1 has type 0, v2 has type 1.
        assert!(v1.value < v2.value);
    }

    #[test]
    fn test_rotate_ranks() {
        // Ace is bit 12 (STD_DECK_RANK_ACE = 12).
        // A (bit 12) -> should become LSB (bit 0)
        // 2 (bit 0) -> should become bit 1

        let ace_mask = 1 << 12;
        let two_mask = 1 << 0;

        let rotated_ace = LowHandVal::rotate_ranks(ace_mask);
        assert_eq!(rotated_ace, 1); // Bit 0 set

        let rotated_two = LowHandVal::rotate_ranks(two_mask);
        assert_eq!(rotated_two, 2); // Bit 1 set

        let both = ace_mask | two_mask;
        let rotated_both = LowHandVal::rotate_ranks(both);
        assert_eq!(rotated_both, 1 | 2);
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_constants() {
        assert!(LOW_HAND_VAL_NOTHING > 0);
        assert!(LOW_HAND_VAL_WORST_EIGHT > 0);
    }
}
