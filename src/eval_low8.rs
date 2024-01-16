use crate::deck_std::{
    STD_DECK_SUIT_CLUBS, STD_DECK_SUIT_DIAMONDS, STD_DECK_SUIT_HEARTS, STD_DECK_SUIT_SPADES,
};
use crate::handval_low::{
    LowHandVal, FIFTH_CARD_MASK, FIFTH_CARD_SHIFT, FOURTH_CARD_MASK, FOURTH_CARD_SHIFT,
    LOW_HAND_VAL_NOTHING, LOW_HAND_VAL_WORST_EIGHT, SECOND_CARD_MASK, SECOND_CARD_SHIFT,
    THIRD_CARD_MASK, THIRD_CARD_SHIFT, TOP_CARD_MASK, TOP_CARD_SHIFT,
};
use crate::rules_std::HandType;
use crate::t_botfivecards::BOTTOM_FIVE_CARDS_TABLE;
use crate::t_cardmasks::{StdDeckCardMask};

// Function to extract the rank of the top card
pub fn extract_top_card_rank(val: u32) -> u8 {
    ((val & TOP_CARD_MASK) >> TOP_CARD_SHIFT) as u8
}

// Function to extract the rank of the second card
pub fn extract_second_card_rank(val: u32) -> u8 {
    ((val & SECOND_CARD_MASK) >> SECOND_CARD_SHIFT) as u8
}

// Function to extract the rank of the third card
pub fn extract_third_card_rank(val: u32) -> u8 {
    ((val & THIRD_CARD_MASK) >> THIRD_CARD_SHIFT) as u8
}

// Function to extract the rank of the fourth card
pub fn extract_fourth_card_rank(val: u32) -> u8 {
    ((val & FOURTH_CARD_MASK) >> FOURTH_CARD_SHIFT) as u8
}

// Function to extract the rank of the fifth card
pub fn extract_fifth_card_rank(val: u32) -> u8 {
    ((val & FIFTH_CARD_MASK) >> FIFTH_CARD_SHIFT) as u8
}

// La fonction `StdDeck_Lowball8_EVAL` en Rust
fn std_deck_lowball8_eval(cards: StdDeckCardMask, _n_cards: i32) -> LowHandVal {
    let card_mask = cards.mask;

    let ranks = card_mask
        & ((STD_DECK_SUIT_HEARTS as u64)
            | (STD_DECK_SUIT_DIAMONDS as u64)
            | (STD_DECK_SUIT_CLUBS as u64)
            | (STD_DECK_SUIT_SPADES as u64));

    // Convertir ranks de u64 à u32
    let ranks_u32 = ranks as u32;

    let ranks = LowHandVal::rotate_ranks(ranks_u32);
    let retval = BOTTOM_FIVE_CARDS_TABLE[ranks as usize];

    if retval > 0 && retval <= LOW_HAND_VAL_WORST_EIGHT {
        // Créez un LowHandVal avec la valeur appropriée

        let top_card = extract_top_card_rank(retval);
        let second_card = extract_second_card_rank(retval);
        let third_card = extract_third_card_rank(retval);
        let fourth_card = extract_fourth_card_rank(retval);
        let fifth_card = extract_fifth_card_rank(retval);

        LowHandVal::new(
            HandType::NoPair as u8,
            top_card,
            second_card,
            third_card,
            fourth_card,
            fifth_card,
        )
    } else {
        // Convertissez LOW_HAND_VAL_NOTHING en LowHandVal avant de retourner
        LowHandVal {
            value: LOW_HAND_VAL_NOTHING,
        }
    }
}
