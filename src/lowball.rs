use crate::handval_low::{LowHandVal, LOW_HAND_VAL_NOTHING};
use crate::deck_std::{StdDeck, STD_DECK_RANK_ACE, STD_DECK_RANK_COUNT, STD_DECK_RANK_8, STD_DECK_RANK_7, STD_DECK_RANK_6, STD_DECK_RANK_5, STD_DECK_RANK_4, STD_DECK_RANK_2, STD_DECK_RANK_CHARS};
use crate::rules_std::{HandType, HAND_TYPE_NAMES, N_SIG_CARDS};
use crate::t_cardmasks::StdDeckCardMask; // Importez StdDeckCardMask
use crate::t_straight::STRAIGHT_TABLE; // Importez STRAIGHT_TABLE
use crate::eval_low::extract_top_five_cards_lowball;

pub fn ace_to_five_lowball_eval(cards: &StdDeckCardMask) -> LowHandVal {
    // Convertissez les valeurs retournées par les méthodes spades, clubs, diamonds et hearts en u32
    let ss = cards.spades() as u32;
    let sc = cards.clubs() as u32;
    let sd = cards.diamonds() as u32;
    let sh = cards.hearts() as u32;

    let ranks = ss | sc | sd | sh;

    // Évaluez la main en ignorant les suites et les flushes
    let (top, second, third, fourth, fifth) = extract_top_five_cards_lowball(ranks as u32);
    LowHandVal::new(HandType::NoPair as u8, top+1, second+1, third+1, fourth+1, fifth+1)
}

pub fn deuce_to_seven_lowball_eval(cards: &StdDeckCardMask) -> LowHandVal {
    // Convertissez les valeurs retournées par les méthodes spades, clubs, diamonds et hearts en u32
    let ss = cards.spades() as u32;
    let sc = cards.clubs() as u32;
    let sd = cards.diamonds() as u32;
    let sh = cards.hearts() as u32;

    let ranks = ss | sc | sd | sh;

    // Vérifiez les suites et les flushes
    if ss == ranks || sc == ranks || sd == ranks || sh == ranks {
        // C'est une flush
        return LowHandVal::new(HandType::Flush as u8, 0, 0, 0, 0, 0);
    }

    if let Some(top_card) = STRAIGHT_TABLE.get(ranks as usize) {
        // C'est une suite
        return LowHandVal::new(HandType::Straight as u8, *top_card, 0, 0, 0, 0);
    }

    // Évaluez la main avec les As étant haut
    let (top, second, third, fourth, fifth) = extract_top_five_cards_lowball(ranks as u32);
    LowHandVal::new(HandType::NoPair as u8, top+1, second, third, fourth, fifth)
}    


impl LowHandVal {



    // Cette méthode imprime la représentation de HandVal pour Ace-to-Five Lowball
    pub fn print_ace_to_five_lowball(&self) {
        println!("{}", self.to_string());
    }

    // Cette méthode imprime la représentation de HandVal pour Deuce-to-Seven Lowball
    pub fn print_deuce_to_seven_lowball(&self) {
        println!("{}", self.to_string());
    }
}

