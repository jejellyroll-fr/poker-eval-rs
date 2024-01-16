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

#[test]
fn test_extract_top_five_cards_with_full_set() {
    // Test avec toutes les cartes présentes
    let cards = 0b1111111111111; // Représente un ensemble complet de cartes
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (1, 2, 3, 4, 5));
}

#[test]
fn test_extract_top_five_cards_with_partial_set() {
    // Test avec un ensemble partiel de cartes
    let cards = 0b11011; // Représente un ensemble partiel de cartes
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (1, 2, 4, 5, 0)); // Les dernières cartes devraient être à zéro si moins de 5 cartes
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




    // Returns a tuple with three elements when given valid input
#[test]
fn test_get_trips_valid_input() {
    let dups = 0b1100;
    let ranks = 0b1111111111;
    let result = get_trips(dups, ranks);
    assert_eq!(result, (3, 1, 2));
}

    // Includes the trips card and two kickers in the output tuple
#[test]
fn test_get_trips_includes_trips_and_kickers() {
    let dups = 0b1100;
    let ranks = 0b1111111111;
    let result = get_trips(dups, ranks);
    assert_eq!(result.0, 3);
    assert_eq!(result.1, 1);
    assert_eq!(result.2, 2);
}

    // Handles input with multiple kickers correctly
#[test]
fn test_get_trips_multiple_kickers() {
    let dups = 0b1100;
    let ranks = 0b1111111111;
    let result = get_trips(dups, ranks);
    assert_eq!(result.1, 1);
    assert_eq!(result.2, 2);
}

    // Throws an error when given input with no kickers
#[test]
#[should_panic(expected = "Logic error in get_trips: insufficient kickers")]
fn test_get_trips_no_kickers() {
    let dups = 0b1100;
    let ranks = 0b1100;
    get_trips(dups, ranks);
}

    // Handles input with only one kicker correctly
#[test]
fn test_get_trips_one_kicker() {
    let dups = 0b1100;
    let ranks = 0b1111111111;
    let result = get_trips(dups, ranks);
    assert_eq!(result.1, 1);
    assert_eq!(result.2, 2);
}




    // Should raise an exception when given valid input with no pairs and at least one kicker.
#[test]
#[should_panic(expected = "Logic error in get_two_pairs: insufficient pairs or kickers")]
fn test_get_two_pairs_valid_input_with_no_pairs_and_at_least_one_kicker() {
    let dups = 0b0;
    let ranks = 0b1111111111;
    get_two_pairs(dups, ranks);
}

    // Should raise an exception when given valid input with no pairs and no kickers.
#[test]
#[should_panic(expected = "Logic error in get_two_pairs: insufficient pairs or kickers")]
fn test_get_two_pairs_valid_input_with_no_pairs_and_no_kickers() {
    let dups = 0b0;
    let ranks = 0b0;
    get_two_pairs(dups, ranks);
}


#[test]
fn test_extract_top_five_cards_few_cards_present() {
    let cards = 0b101; // Example: Only 2 cards present
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (1, 3, 0, 0, 0)); // Expect the first two cards and zeros for the rest
}

#[test]
fn test_extract_top_five_cards_non_sequential() {
    let cards = 0b1010101; // Example: Non-sequential cards
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (1, 3, 5, 7, 0)); // Expect the correct five cards
}

#[test]
#[should_panic(expected = "Logic error in get_two_pairs: insufficient pairs")]
fn test_get_two_pairs_only_one_pair() {
    let dups = 0b1000; // Only one pair
    let ranks = 0b1111111;
    get_two_pairs(dups, ranks);
}


 