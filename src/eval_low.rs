use crate::handval_low::{LowHandVal, CARD_WIDTH};
use crate::rules_std::*;
use crate::t_botcard::BOTTOM_CARD_TABLE;
use crate::t_cardmasks::StdDeckCardMask;
use crate::t_nbits::NBITS_TABLE;


pub fn bottom_n_cards(mut cards: u32, how_many: usize) -> u32 {
    let mut retval = 0;
    for i in 0..how_many {
        let t = BOTTOM_CARD_TABLE[cards as usize] as u32;  // Convertir t en u32
        retval |= t << (i * CARD_WIDTH as usize);
        cards ^= 1 << t;
    }
    retval
}

pub fn std_deck_lowball_eval(cards: &StdDeckCardMask, n_cards: usize) -> LowHandVal {
    // Convertissez les valeurs retournées par les méthodes spades, clubs, diamonds et hearts en u32
    let ss = LowHandVal::rotate_ranks(cards.spades().into());
    let sc = LowHandVal::rotate_ranks(cards.clubs().into());
    let sd = LowHandVal::rotate_ranks(cards.diamonds().into());
    let sh = LowHandVal::rotate_ranks(cards.hearts().into());

    let ranks = sc | ss | sd | sh;
    let n_ranks = NBITS_TABLE[ranks as usize];// Implémentez la logique pour n_bits_table
    let dups = (sc & sd) | (sh & (sc | sd)) | (ss & (sh | sc | sd));

    if n_ranks >= 5 {
        LowHandVal::new(HandType::NoPair as u8, bottom_five_cards_table(ranks), 0, 0, 0, 0)
    } else {
        // Suite du code pour gérer les cas avec des paires, brelans, etc.
        // ...
    }
    match n_ranks {
        4 => {
            let pair_card = bottom_card_table(dups);
            let kickers = bottom_n_cards(ranks ^ (1 << pair_card), 3);
            LowHandVal::new(HandType::OnePair as u8, pair_card, kickers, 0, 0, 0)
        },
        3 => {
            if n_bits_table(dups) == 2 {
                // Deux paires
                // ...
            } else {
                // Un brelan
                let trips_card = bottom_card_table(dups);
                let kickers = bottom_n_cards(ranks ^ (1 << trips_card), 2);
                LowHandVal::new(HandType::Trips as u8, trips_card, kickers, 0, 0, 0)
            }
        },
        2 => {
            if n_bits_table(dups) == 2 {
                // Full house
                // ...
            } else {
                // Quads
                let quads_card = bottom_card_table(dups);
                let kicker = bottom_card_table(ranks ^ (1 << quads_card));
                LowHandVal::new(HandType::Quads as u8, quads_card, kicker, 0, 0, 0)
            }
        },
        _ => {
            // Autres cas ou erreurs
            panic!("Logic error in std_deck_lowball_eval")
        }
    }   
}