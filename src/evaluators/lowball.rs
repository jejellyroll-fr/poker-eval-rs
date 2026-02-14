use crate::deck::STD_DECK_RANK_COUNT;
use crate::handval_low::LowHandVal;
use crate::rules::*;
use crate::tables::t_botcard::BOTTOM_CARD_TABLE;
use crate::tables::t_cardmasks::StdDeckCardMask;

/// Extracts the top five unique ranks from a bitmask of ranks.
pub fn extract_top_five_cards_lowball(cards: u32) -> (u8, u8, u8, u8, u8) {
    let mut extracted_cards = [0u8; 5];
    let mut count = 0;

    for i in 0..STD_DECK_RANK_COUNT {
        if cards & (1 << i) != 0 {
            extracted_cards[count] = i as u8 + 1; // Add 1 to start counting from 1 instead of 0
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

/// Extracts trips card and two kickers from the given duplicates and ranks bitmasks.
/// Returns `None` if insufficient kickers are available.
pub fn get_trips(dups: u32, ranks: u32) -> Option<(u8, u8, u8)> {
    let trips_card = BOTTOM_CARD_TABLE[(dups & 0x1FFF) as usize];
    let mut kickers = [0u8; 2];
    let mut count = 0;

    for i in 0..STD_DECK_RANK_COUNT {
        if (ranks & (1 << i) != 0) && (i as u8 != trips_card) {
            kickers[count] = i as u8;
            count += 1;
            if count == 2 {
                break;
            }
        }
    }

    if count >= 2 {
        Some((trips_card + 1, kickers[0] + 1, kickers[1] + 1))
    } else {
        None
    }
}

/// Extracts two pairs and a kicker from the given duplicates and ranks bitmasks.
pub fn get_two_pairs(dups: u32, ranks: u32) -> Option<(u8, u8, u8)> {
    // We need the two highest pairs and the highest kicker.
    // Iterating in reverse (12 down to 0) allows us to find them immediately.
    let mut pair1 = None;
    let mut pair2 = None;
    let mut kicker = None;

    for i in (0..STD_DECK_RANK_COUNT).rev() {
        if dups & (1 << i) != 0 {
            if pair1.is_none() {
                pair1 = Some(i);
            } else if pair2.is_none() {
                pair2 = Some(i);
            }
        } else if ranks & (1 << i) != 0 && kicker.is_none() {
            kicker = Some(i);
        }

        if pair1.is_some() && pair2.is_some() && kicker.is_some() {
            break;
        }
    }

    match (pair1, pair2, kicker) {
        (Some(p1), Some(p2), Some(k)) => Some((p1 as u8 + 1, p2 as u8 + 1, k as u8 + 1)),
        _ => None,
    }
}

/// Extracts a full house (trips rank, pair rank) from the duplicates bitmask.
pub fn get_full_house(dups: u32) -> (u8, u8) {
    let three_mask = (dups & (dups - 1)) & dups; // Mask for three matching cards
    let three_card = BOTTOM_CARD_TABLE[(three_mask & 0x1FFF) as usize];
    let pair_mask = dups ^ three_mask;
    let pair_card = BOTTOM_CARD_TABLE[(pair_mask & 0x1FFF) as usize];
    (three_card + 1, pair_card + 1)
}

/// Evaluates a hand for Ace-to-Five Lowball (California Lowball).
///
/// In A-5 Lowball:
/// - Straights and Flushes are ignored.
/// - Ace is low (1).
/// - Best hand is 5-4-3-2-A (Wheel).
pub fn std_deck_lowball_eval(cards: &StdDeckCardMask, _n_cards: usize) -> LowHandVal {
    let ss = LowHandVal::rotate_ranks(cards.spades().into());
    let sc = LowHandVal::rotate_ranks(cards.clubs().into());
    let sd = LowHandVal::rotate_ranks(cards.diamonds().into());
    let sh = LowHandVal::rotate_ranks(cards.hearts().into());

    let ranks = sc | ss | sd | sh;
    let _n_ranks = ranks.count_ones() as u8;
    let _dups = (sc & sd) | (sh & (sc | sd)) | (ss & (sh | sc | sd));

    if _n_ranks >= 5 {
        let (top, second, third, fourth, fifth) = extract_top_five_cards_lowball(ranks);
        // We must pass the highest rank (fifth) first as the "top" card for correct value comparison
        // (Worst card in MSB).
        return LowHandVal::new(HandType::NoPair as u8, fifth, fourth, third, second, top);
    }
    // Subtract to return to the original indices

    match _n_ranks {
        4 => {
            let dups_masked = _dups & 0x1FFF;
            if dups_masked == 0 {
                return LowHandVal::new(HandType::NoPair as u8, 0, 0, 0, 0, 0);
            }
            let pair_card = BOTTOM_CARD_TABLE[dups_masked as usize];
            if pair_card >= 32 {
                return LowHandVal::new(HandType::NoPair as u8, 0, 0, 0, 0, 0);
            }
            let mut temp_ranks = ranks ^ (1 << pair_card);
            let mut kickers = [0u8; 3];
            for k in &mut kickers {
                let t_idx = temp_ranks & 0x1FFF;
                if t_idx == 0 {
                    break;
                }
                let t = BOTTOM_CARD_TABLE[t_idx as usize];
                if t >= 32 {
                    break;
                }
                *k = t + 1;
                temp_ranks ^= 1 << t;
            }
            LowHandVal::new(
                HandType::OnePair as u8,
                pair_card + 1,
                kickers[2],
                kickers[1],
                kickers[0],
                0,
            )
        }
        3 => {
            let dups_masked = _dups & 0x1FFF;
            if dups_masked == 0 {
                return LowHandVal::new(HandType::NoPair as u8, 0, 0, 0, 0, 0);
            }
            if dups_masked.count_ones() == 2 {
                if let Some((pair1, pair2, kicker)) = get_two_pairs(dups_masked, ranks) {
                    LowHandVal::new(HandType::TwoPair as u8, pair1, pair2, kicker, 0, 0)
                } else {
                    LowHandVal::new(HandType::NoPair as u8, 0, 0, 0, 0, 0)
                }
            } else if let Some((trips_card, kicker1, kicker2)) = get_trips(dups_masked, ranks) {
                LowHandVal::new(HandType::Trips as u8, trips_card, kicker1, kicker2, 0, 0)
            } else {
                LowHandVal::new(HandType::NoPair as u8, 0, 0, 0, 0, 0)
            }
        }
        2 => {
            let dups_masked = _dups & 0x1FFF;
            if dups_masked == 0 {
                return LowHandVal::new(HandType::NoPair as u8, 0, 0, 0, 0, 0);
            }
            if dups_masked.count_ones() == 2 {
                let (three_of_a_kind, pair) = get_full_house(dups_masked);

                LowHandVal::new(HandType::FullHouse as u8, three_of_a_kind, pair, 0, 0, 0)
            } else {
                let quads_card = BOTTOM_CARD_TABLE[dups_masked as usize];
                if quads_card >= 32 {
                    return LowHandVal::new(HandType::NoPair as u8, 0, 0, 0, 0, 0);
                }
                let kicker_idx = (ranks ^ (1 << quads_card)) & 0x1FFF;
                let kicker = BOTTOM_CARD_TABLE[kicker_idx as usize];
                LowHandVal::new(HandType::Quads as u8, quads_card + 1, kicker + 1, 0, 0, 0)
            }
        }
        _ => LowHandVal::new(HandType::NoPair as u8, 0, 0, 0, 0, 0),
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
    // Test with all cards present
    let cards = 0b1111111111111; // Represents a complete set of cards
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (1, 2, 3, 4, 5));
}

#[test]
fn test_extract_top_five_cards_with_partial_set() {
    // Test with a partial set of cards
    let cards = 0b11011; // Represents a partial set of cards
    let result = extract_top_five_cards_lowball(cards);
    assert_eq!(result, (1, 2, 4, 5, 0)); // Remaining cards should be zero if fewer than 5 cards
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
    assert_eq!(result, Some((3, 1, 2)));
}

// Includes the trips card and two kickers in the output tuple
#[test]
fn test_get_trips_includes_trips_and_kickers() {
    let dups = 0b1100;
    let ranks = 0b1111111111;
    let result = get_trips(dups, ranks).unwrap();
    assert_eq!(result.0, 3);
    assert_eq!(result.1, 1);
    assert_eq!(result.2, 2);
}

// Handles input with multiple kickers correctly
#[test]
fn test_get_trips_multiple_kickers() {
    let dups = 0b1100;
    let ranks = 0b1111111111;
    let result = get_trips(dups, ranks).unwrap();
    assert_eq!(result.1, 1);
    assert_eq!(result.2, 2);
}

// Returns None when given input with no kickers
#[test]
fn test_get_trips_no_kickers() {
    let dups = 0b1100;
    let ranks = 0b1100;
    assert_eq!(get_trips(dups, ranks), None);
}

// Handles input with only one kicker correctly
#[test]
fn test_get_trips_one_kicker() {
    let dups = 0b1100;
    let ranks = 0b1111111111;
    let result = get_trips(dups, ranks).unwrap();
    assert_eq!(result.1, 1);
    assert_eq!(result.2, 2);
}

// Returns None when given input with no pairs and at least one kicker.
#[test]
fn test_get_two_pairs_valid_input_with_no_pairs_and_at_least_one_kicker() {
    let dups = 0b0;
    let ranks = 0b1111111111;
    assert_eq!(get_two_pairs(dups, ranks), None);
}

// Returns None when given input with no pairs and no kickers.
#[test]
fn test_get_two_pairs_valid_input_with_no_pairs_and_no_kickers() {
    let dups = 0b0;
    let ranks = 0b0;
    assert_eq!(get_two_pairs(dups, ranks), None);
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
fn test_get_two_pairs_only_one_pair() {
    let dups = 0b1000; // Only one pair
    let ranks = 0b1111111;
    assert_eq!(get_two_pairs(dups, ranks), None);
}

#[cfg(test)]
mod lowball_eval_tests {
    use super::*;
    use crate::deck::StdDeck;
    use crate::rules::HandType;

    #[test]
    fn test_wheel_is_best_low_hand() {
        // A-2-3-4-5 (the wheel) should be the best possible low hand
        let (wheel, _) = StdDeck::string_to_mask("Ah2d3c4s5h").expect("Failed to parse wheel");
        let wheel_val = std_deck_lowball_eval(&wheel, 5);

        // The wheel should be a NoPair hand type in lowball
        assert_eq!(
            wheel_val.hand_type(),
            HandType::NoPair as u8,
            "Wheel (A-2-3-4-5) should be NoPair in lowball"
        );

        // Compare against other low hands - wheel should be lower (better)
        let (second_best, _) = StdDeck::string_to_mask("Ah2d3c4s6h").expect("Failed to parse");
        let second_val = std_deck_lowball_eval(&second_best, 5);

        assert!(
            wheel_val < second_val,
            "Wheel (A-2-3-4-5) should be better (lower) than A-2-3-4-6"
        );

        let (eight_low, _) = StdDeck::string_to_mask("Ah2d3c4s8h").expect("Failed to parse");
        let eight_val = std_deck_lowball_eval(&eight_low, 5);

        assert!(
            wheel_val < eight_val,
            "Wheel (A-2-3-4-5) should be better (lower) than A-2-3-4-8"
        );
    }

    #[test]
    fn test_pairs_are_worse_than_no_pair_lows() {
        // A no-pair hand should always beat a one-pair hand in lowball
        let (no_pair, _) = StdDeck::string_to_mask("2d3c4s5h8d").expect("Failed to parse no pair");
        let no_pair_val = std_deck_lowball_eval(&no_pair, 5);

        let (one_pair, _) = StdDeck::string_to_mask("2d2c3s4h5d").expect("Failed to parse pair");
        let one_pair_val = std_deck_lowball_eval(&one_pair, 5);

        assert_eq!(
            no_pair_val.hand_type(),
            HandType::NoPair as u8,
            "No pair hand should have NoPair type"
        );
        assert_eq!(
            one_pair_val.hand_type(),
            HandType::OnePair as u8,
            "Pair hand should have OnePair type"
        );
        assert!(
            no_pair_val < one_pair_val,
            "No-pair low should beat (be lower than) a paired hand in lowball"
        );
    }

    #[test]
    fn test_lowball_eval_seven_card_hand() {
        // A 7-card hand: the evaluator should pick the best 5-card low
        // Hand: Ah 2d 3c 4s 5h Kd Qc
        // Best low should be A-2-3-4-5 (the wheel)
        let (seven_cards, _) =
            StdDeck::string_to_mask("Ah2d3c4s5hKdQc").expect("Failed to parse 7-card hand");
        let result = std_deck_lowball_eval(&seven_cards, 7);

        // Should be a NoPair (best 5 low ranks from the 7 cards)
        assert_eq!(
            result.hand_type(),
            HandType::NoPair as u8,
            "7-card lowball eval should produce NoPair when wheel is available"
        );

        // The result should be equal to a pure wheel evaluation
        let (wheel, _) = StdDeck::string_to_mask("Ah2d3c4s5h").expect("Failed to parse wheel");
        let wheel_val = std_deck_lowball_eval(&wheel, 5);

        assert_eq!(
            result.value, wheel_val.value,
            "7-card hand with A-2-3-4-5-K-Q should evaluate the same as a 5-card wheel"
        );
    }
}
