use crate::handval_low::{
    LowHandVal, FIFTH_CARD_SHIFT, FOURTH_CARD_SHIFT, HANDTYPE_MASK, HANDTYPE_SHIFT,
    LOW_HAND_VAL_NOTHING, LOW_HAND_VAL_WORST_EIGHT, SECOND_CARD_SHIFT, THIRD_CARD_SHIFT,
    TOP_CARD_SHIFT,
};
use crate::rules_std::HandType;
use crate::t_botfivecards::BOTTOM_FIVE_CARDS_TABLE;
use crate::t_cardmasks::StdDeckCardMask;

pub fn std_deck_lowball8_eval(cards: &StdDeckCardMask, _n_cards: usize) -> Option<LowHandVal> {
    let ss = LowHandVal::rotate_ranks(cards.spades().into());
    let sc = LowHandVal::rotate_ranks(cards.clubs().into());
    let sd = LowHandVal::rotate_ranks(cards.diamonds().into());
    let sh = LowHandVal::rotate_ranks(cards.hearts().into());

    let ranks = sc | ss | sd | sh;

    let retval = BOTTOM_FIVE_CARDS_TABLE[ranks as usize];

    if retval > 0 && retval <= LOW_HAND_VAL_WORST_EIGHT {
        let mut value = ((HandType::NoPair as u32) << HANDTYPE_SHIFT) & HANDTYPE_MASK;
        value |= ((retval >> TOP_CARD_SHIFT) & 0xF) << TOP_CARD_SHIFT;
        value |= ((retval >> SECOND_CARD_SHIFT) & 0xF) << SECOND_CARD_SHIFT;
        value |= ((retval >> THIRD_CARD_SHIFT) & 0xF) << THIRD_CARD_SHIFT;
        value |= ((retval >> FOURTH_CARD_SHIFT) & 0xF) << FOURTH_CARD_SHIFT;
        value |= ((retval >> FIFTH_CARD_SHIFT) & 0xF) << FIFTH_CARD_SHIFT;

        Some(LowHandVal { value })
    } else {
        Some(LowHandVal {
            value: LOW_HAND_VAL_NOTHING,
        })
    }
}
