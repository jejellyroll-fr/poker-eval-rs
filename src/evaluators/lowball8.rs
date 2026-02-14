use crate::handval_low::{
    LowHandVal, FIFTH_CARD_SHIFT, FOURTH_CARD_SHIFT, HANDTYPE_MASK, HANDTYPE_SHIFT,
    LOW_HAND_VAL_NOTHING, LOW_HAND_VAL_WORST_EIGHT, SECOND_CARD_SHIFT, THIRD_CARD_SHIFT,
    TOP_CARD_SHIFT,
};
use crate::rules::HandType;
use crate::tables::t_botfivecards::BOTTOM_FIVE_CARDS_TABLE;
use crate::tables::t_cardmasks::StdDeckCardMask;

/// Evaluates a hand for 8-or-better Lowball (e.g. for Omaha Hi/Lo or Stud8).
///
/// Returns `Some(LowHandVal)` if the hand qualifies (all cards <= 8),
/// or `Some(LOW_HAND_VAL_NOTHING)` / `None` based on implementation (here consistent with `LowHandVal` wrapper).
pub fn std_deck_lowball8_eval(cards: &StdDeckCardMask, _n_cards: usize) -> Option<LowHandVal> {
    let ss = LowHandVal::rotate_ranks(cards.spades().into());
    let sc = LowHandVal::rotate_ranks(cards.clubs().into());
    let sd = LowHandVal::rotate_ranks(cards.diamonds().into());
    let sh = LowHandVal::rotate_ranks(cards.hearts().into());

    let ranks = sc | ss | sd | sh;

    let retval = BOTTOM_FIVE_CARDS_TABLE[ranks as usize];

    if retval > 0 {
        // Table returns 0-based ranks (0..12). LowHandVal expects 1-based (1..13).
        // We must unpack, add 1, and repack.
        let t1 = ((retval >> TOP_CARD_SHIFT) & 0xF) + 1;
        let t2 = ((retval >> SECOND_CARD_SHIFT) & 0xF) + 1;
        let t3 = ((retval >> THIRD_CARD_SHIFT) & 0xF) + 1;
        let t4 = ((retval >> FOURTH_CARD_SHIFT) & 0xF) + 1;
        let t5 = ((retval >> FIFTH_CARD_SHIFT) & 0xF) + 1;

        // Reconstruct value
        let mut value = ((HandType::NoPair as u32) << HANDTYPE_SHIFT) & HANDTYPE_MASK;
        value |= t1 << TOP_CARD_SHIFT;
        value |= t2 << SECOND_CARD_SHIFT;
        value |= t3 << THIRD_CARD_SHIFT;
        value |= t4 << FOURTH_CARD_SHIFT;
        value |= t5 << FIFTH_CARD_SHIFT;

        // Compare constructed value against WORST_EIGHT
        if value <= LOW_HAND_VAL_WORST_EIGHT {
            Some(LowHandVal { value })
        } else {
            Some(LowHandVal {
                value: LOW_HAND_VAL_NOTHING,
            })
        }
    } else {
        Some(LowHandVal {
            value: LOW_HAND_VAL_NOTHING,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::StdDeck;
    use crate::handval_low::LOW_HAND_VAL_NOTHING;

    fn eval_str(cards_str: &str) -> Option<LowHandVal> {
        let (mask, _) = StdDeck::string_to_mask(cards_str).expect("failed to parse");
        std_deck_lowball8_eval(&mask, 5)
    }

    #[test]
    fn test_wheel_is_best() {
        // A-2-3-4-5 is the best low hand in 8-or-better (Straights/Flushes ignored for low)
        let val = eval_str("As2d3c4h5s").unwrap();
        assert_ne!(val.value, LOW_HAND_VAL_NOTHING);

        // Compare against a 6-low
        let val6 = eval_str("As2d3c4h6s").unwrap();
        assert!(val < val6, "Wheel should be better (lower val) than 6-low");
    }

    #[test]
    fn test_6_low() {
        let val = eval_str("As2d3c4h6s").unwrap();
        assert_ne!(val.value, LOW_HAND_VAL_NOTHING);
    }

    #[test]
    fn test_8_low_qualifies() {
        // 8-7-6-5-4 is the worst qualifying hand
        let val = eval_str("8s7d6c5h4s").unwrap();
        assert_ne!(val.value, LOW_HAND_VAL_NOTHING);
    }

    #[test]
    fn test_9_low_does_not_qualify() {
        // 9-high does not qualify
        let val = eval_str("9s2d3c4h5s").unwrap();
        assert_eq!(val.value, LOW_HAND_VAL_NOTHING);
    }

    #[test]
    fn test_pair_does_not_qualify() {
        let val = eval_str("AsAd3c4h5s").unwrap();
        assert_eq!(val.value, LOW_HAND_VAL_NOTHING);
    }
}
