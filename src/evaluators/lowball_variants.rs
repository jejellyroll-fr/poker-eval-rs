use crate::handval_low::LowHandVal;

use super::lowball::extract_top_five_cards_lowball;
use crate::rules::HandType;
use crate::tables::t_cardmasks::StdDeckCardMask;
use crate::tables::t_straight::STRAIGHT_TABLE;

pub fn ace_to_five_lowball_eval(cards: &StdDeckCardMask) -> LowHandVal {
    // Convert the values returned by the spades, clubs, diamonds and hearts methods to u32
    let ss = cards.spades() as u32;
    let sc = cards.clubs() as u32;
    let sd = cards.diamonds() as u32;
    let sh = cards.hearts() as u32;

    let ranks = ss | sc | sd | sh;

    // Evaluate the hand ignoring straights and flushes
    let (top, second, third, fourth, fifth) = extract_top_five_cards_lowball(ranks);
    LowHandVal::new(
        HandType::NoPair as u8,
        top + 1,
        second + 1,
        third + 1,
        fourth + 1,
        fifth + 1,
    )
}

pub fn deuce_to_seven_lowball_eval(cards: &StdDeckCardMask) -> LowHandVal {
    // Convert the values returned by the spades, clubs, diamonds and hearts methods to u32
    let ss = cards.spades() as u32;
    let sc = cards.clubs() as u32;
    let sd = cards.diamonds() as u32;
    let sh = cards.hearts() as u32;

    let ranks = ss | sc | sd | sh;

    // Check for straights and flushes
    if ss == ranks || sc == ranks || sd == ranks || sh == ranks {
        // It's a flush
        return LowHandVal::new(HandType::Flush as u8, 0, 0, 0, 0, 0);
    }

    if let Some(top_card) = STRAIGHT_TABLE.get(ranks as usize) {
        // It's a straight
        return LowHandVal::new(HandType::Straight as u8, *top_card, 0, 0, 0, 0);
    }

    // Evaluate the hand with Aces being high
    let (top, second, third, fourth, fifth) = extract_top_five_cards_lowball(ranks);
    LowHandVal::new(
        HandType::NoPair as u8,
        top + 1,
        second + 1,
        third + 1,
        fourth + 1,
        fifth + 1,
    )
}
