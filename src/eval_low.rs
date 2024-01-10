use crate::handval_low::{LowHandVal, CARD_WIDTH};
use crate::rules_std::*;
use crate::t_botcard::BOTTOM_CARD_TABLE;
use crate::t_cardmasks::StdDeckCardMask;
use crate::t_nbits::NBITS_TABLE;


fn extract_top_five_cards_lowball(cards: u32) -> (u8, u8, u8, u8, u8) {
    let mut extracted_cards = [0u8; 5];
    let mut count = 0;

    for i in 0..13 {
        if cards & (1 << i) != 0 {
            extracted_cards[count] = i as u8 + 1; // Ajoutez 2 pour commencer à compter à partir de 2 au lieu de 0
            count += 1;
            if count == 5 {
                break;
            }
        }
    }

    (extracted_cards[0], extracted_cards[1], extracted_cards[2], extracted_cards[3], extracted_cards[4])
}

pub fn get_two_pairs(dups: u32) -> (u8, u8) {
    let pair1 = BOTTOM_CARD_TABLE[dups as usize];
    let pair2 = BOTTOM_CARD_TABLE[(dups ^ (1 << pair1)) as usize];
    (pair1+1, pair2+1)
}

pub fn get_full_house(dups: u32) -> (u8, u8) {
    let three_mask = (dups & (dups - 1)) & dups; // Masque pour trois cartes identiques
    let three_card = BOTTOM_CARD_TABLE[three_mask as usize];
    let pair_mask = dups ^ three_mask;
    let pair_card = BOTTOM_CARD_TABLE[pair_mask as usize];
    (three_card+1, pair_card+1)
}

pub fn bottom_n_cards(mut cards: u32, how_many: usize) -> u32 {
    let mut retval = 0;
    for i in 0..how_many {
        let t = BOTTOM_CARD_TABLE[cards as usize] as u32;  // Convertir t en u32
        retval |= t << (i * CARD_WIDTH as usize);
        cards ^= 1 << t;
    }
    retval+1
}

pub fn std_deck_lowball_eval(cards: &StdDeckCardMask, n_cards: usize) -> LowHandVal {
    let ss = LowHandVal::rotate_ranks(cards.spades().into());
    let sc = LowHandVal::rotate_ranks(cards.clubs().into());
    let sd = LowHandVal::rotate_ranks(cards.diamonds().into());
    let sh = LowHandVal::rotate_ranks(cards.hearts().into());

    let ranks = sc | ss | sd | sh;
    let n_ranks = NBITS_TABLE[ranks as usize];
    let dups = (sc & sd) | (sh & (sc | sd)) | (ss & (sh | sc | sd));

    if n_ranks >= 5 {
        let (top, second, third, fourth, fifth) = extract_top_five_cards_lowball(ranks);
        return LowHandVal::new(HandType::NoPair as u8, top , second , third , fourth , fifth ); // Soustrayez 2 pour revenir aux indices originaux
    }

    match n_ranks {
        4 => {
            let pair_card = BOTTOM_CARD_TABLE[dups as usize];
            let (kicker1, kicker2, kicker3, _, _) = extract_top_five_cards_lowball(ranks ^ (1 << pair_card));
            LowHandVal::new(HandType::OnePair as u8, pair_card+1, kicker1, kicker2, kicker3, 0)
        },
        3 => {
            if NBITS_TABLE[dups as usize] == 2 {
                // Deux paires
                let (pair1, pair2) = get_two_pairs(dups);
                let kicker = BOTTOM_CARD_TABLE[(ranks ^ (1 << pair1) ^ (1 << pair2)) as usize];
                LowHandVal::new(HandType::TwoPair as u8, pair1, pair2, kicker, 0, 0)
            } else {
                // Un brelan
                let trips_card = BOTTOM_CARD_TABLE[dups as usize];
                let (kicker1, kicker2, _, _, _) = extract_top_five_cards_lowball(ranks ^ (1 << trips_card));
                LowHandVal::new(HandType::Trips as u8, trips_card+1, kicker1, kicker2, 0, 0)
            }
        },
        2 => {
            if NBITS_TABLE[dups as usize] == 2 {
                // Full house
                let (three_of_a_kind, pair) = get_full_house(dups);
                LowHandVal::new(HandType::FullHouse as u8, three_of_a_kind, pair, 0, 0, 0)
            } else {
                // Quads
                let quads_card = BOTTOM_CARD_TABLE[dups as usize];
                let kicker = BOTTOM_CARD_TABLE[(ranks ^ (1 << quads_card)) as usize];
                LowHandVal::new(HandType::Quads as u8, quads_card+1, kicker, 0, 0, 0)
            }
        },
        _ => {
            panic!("Logic error in std_deck_lowball_eval")
        }
    }
}