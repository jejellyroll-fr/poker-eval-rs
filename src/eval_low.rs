use crate::handval_low::LowHandVal;
use crate::rules_std::*;
use crate::t_botcard::BOTTOM_CARD_TABLE;
use crate::t_cardmasks::StdDeckCardMask;
use crate::t_nbits::NBITS_TABLE;

pub fn extract_top_five_cards_lowball(cards: u32) -> (u8, u8, u8, u8, u8) {
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

    (
        extracted_cards[0],
        extracted_cards[1],
        extracted_cards[2],
        extracted_cards[3],
        extracted_cards[4],
    )
}

pub fn get_trips(dups: u32, ranks: u32) -> (u8, u8, u8) {
    let trips_card = BOTTOM_CARD_TABLE[dups as usize];
    let mut kickers = Vec::new();

    // Identifier les kickers
    for i in 0..13 {
        if (ranks & (1 << i) != 0) && (i != trips_card) {
            kickers.push(i);
        }
    }

    // Assurer qu'il y a deux kickers
    if kickers.len() >= 2 {
        let kicker1 = kickers[0]; // Le plus bas kicker
        let kicker2 = kickers[1]; // Le deuxième plus bas kicker
        (trips_card as u8 + 1, kicker1 as u8 + 1, kicker2 as u8 + 1)
    } else {
        // Cas d'erreur ou situation inattendue
        panic!("Logic error in get_trips: insufficient kickers");
    }
}

pub fn get_two_pairs(dups: u32, ranks: u32) -> (u8, u8, u8) {
    let mut pairs = Vec::new();
    let mut kickers = Vec::new();

    // Identifier les paires
    for i in 0..13 {
        if dups & (1 << i) != 0 {
            pairs.push(i);
        } else if ranks & (1 << i) != 0 {
            kickers.push(i);
        }
    }

    // Assurer qu'on a deux paires et un kicker
    if pairs.len() >= 2 && !kickers.is_empty() {
        let pair1 = pairs[pairs.len() - 1]; // La plus haute paire
        let pair2 = pairs[pairs.len() - 2]; // La deuxième plus haute paire
        let kicker = kickers[kickers.len() - 1]; // Le plus haut kicker
        (pair1 as u8 + 1, pair2 as u8 + 1, kicker as u8 + 1)
    } else {
        // Cas d'erreur ou situation inattendue
        panic!("Logic error in get_two_pairs: insufficient pairs or kickers");
    }
}

pub fn get_full_house(dups: u32) -> (u8, u8) {
    let three_mask = (dups & (dups - 1)) & dups; // Masque pour trois cartes identiques
    let three_card = BOTTOM_CARD_TABLE[three_mask as usize];
    let pair_mask = dups ^ three_mask;
    let pair_card = BOTTOM_CARD_TABLE[pair_mask as usize];
    (three_card + 1, pair_card + 1)
}

pub fn bottom_n_cards(mut cards: u32, how_many: usize) -> Vec<u8> {
    let mut retval = Vec::new();
    for _ in 0..how_many {
        let t = BOTTOM_CARD_TABLE[cards as usize];
        retval.push(t + 1); // Ajoutez 2 car les cartes commencent à 2, pas à 0
        cards ^= 1 << t;
    }
    retval
}

pub fn std_deck_lowball_eval(cards: &StdDeckCardMask, _n_cards: usize) -> LowHandVal {
    let ss = LowHandVal::rotate_ranks(cards.spades().into());
    let sc = LowHandVal::rotate_ranks(cards.clubs().into());
    let sd = LowHandVal::rotate_ranks(cards.diamonds().into());
    let sh = LowHandVal::rotate_ranks(cards.hearts().into());

    let ranks = sc | ss | sd | sh;
    let _n_ranks = NBITS_TABLE[ranks as usize];
    let _dups = (sc & sd) | (sh & (sc | sd)) | (ss & (sh | sc | sd));

    //println!("Rangs combinés: {:b}, Nombre de rangs: {}", ranks, n_ranks);
    //println!("Nombre de duplicatas 1: {}", dups);
    //println!("Spades: {:b}, Clubs: {:b}, Diamonds: {:b}, Hearts: {:b}", ss, sc, sd, sh);

    if _n_ranks >= 5 {
        let (top, second, third, fourth, fifth) = extract_top_five_cards_lowball(ranks);
        return LowHandVal::new(HandType::NoPair as u8, top, second, third, fourth, fifth);
        // Soustrayez 2 pour revenir aux indices originaux
    }

    match _n_ranks {
        4 => {
            let pair_card = BOTTOM_CARD_TABLE[_dups as usize];
            let kickers = bottom_n_cards(ranks ^ (1 << pair_card), 3);
            // Assurez-vous d'ajouter 1 à chaque carte pour les rangs
            LowHandVal::new(
                HandType::OnePair as u8,
                pair_card + 1,
                kickers[0],
                kickers[1],
                kickers[2],
                0,
            )
        }
        3 => {
            if NBITS_TABLE[_dups as usize] == 2 {
                // Deux paires
                let (pair1, pair2, kicker) = get_two_pairs(_dups, ranks);
                LowHandVal::new(HandType::TwoPair as u8, pair1, pair2, kicker, 0, 0)
            } else {
                // Un brelan
                let (trips_card, kicker1, kicker2) = get_trips(_dups, ranks);
                LowHandVal::new(HandType::Trips as u8, trips_card, kicker1, kicker2, 0, 0)
            }
        }
        2 => {
            if NBITS_TABLE[_dups as usize] == 2 {
                // Full house
                let (three_of_a_kind, pair) = get_full_house(_dups);
                LowHandVal::new(HandType::FullHouse as u8, three_of_a_kind, pair, 0, 0, 0)
            } else {
                // Quads
                let quads_card = BOTTOM_CARD_TABLE[_dups as usize];
                let kicker = BOTTOM_CARD_TABLE[(ranks ^ (1 << quads_card)) as usize];
                LowHandVal::new(HandType::Quads as u8, quads_card + 1, kicker + 1, 0, 0, 0)
            }
        }
        _ => {
            panic!("Logic error in std_deck_lowball_eval")
        }
    }
}



    // Extract top five cards from a valid input with all five cards present
#[test]
fn test_extract_top_five_cards_all_present() {
    let cards = 0b11111;
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (1, 2, 3, 4, 5));
}

    // Extract top five cards from a valid input with less than five cards present
#[test]
fn test_extract_top_five_cards_less_than_five_present() {
    let cards = 0b1100;
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (1, 2, 0, 0, 0));
}

    // Extract top five cards from a valid input with more than five cards present
#[test]
fn test_extract_top_five_cards_more_than_five_present() {
    let cards = 0b1111111111;
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (1, 2, 3, 4, 5));
}

    // Extract top five cards from an input with no cards present
#[test]
fn test_extract_top_five_cards_no_cards_present() {
    let cards = 0b0;
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (0, 0, 0, 0, 0));
}

    // Extract top five cards from an input with all cards present
#[test]
fn test_extract_top_five_cards_all_cards_present() {
    let cards = 0b1111111111111;
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (1, 2, 3, 4, 5));
}

    // Extract top five cards from an input with only one card present
#[test]
fn test_extract_top_five_cards_one_card_present() {
    let cards = 0b10000;
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (1, 0, 0, 0, 0));
}
