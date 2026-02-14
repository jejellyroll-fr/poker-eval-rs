use super::lowball::extract_top_five_cards_lowball;
use crate::handval::{
    HandVal, CARD_MASK, CARD_WIDTH, FIFTH_CARD_MASK, FOURTH_CARD_SHIFT, SECOND_CARD_SHIFT,
    THIRD_CARD_SHIFT,
};
use crate::rules::HandType;
use crate::tables::t_cardmasks::StdDeckCardMask;

use crate::tables::t_straight::STRAIGHT_TABLE;
use crate::tables::t_topcard::TOP_CARD_TABLE;
use crate::tables::t_topfivecards::TOP_FIVE_CARDS_TABLE;

/// Evaluates a hand for 2-to-7 Lowball (Kansas City Lowball).
///
/// In 2-7 Lowball:
/// - Straights and Flushes count AGAINST you.
/// - Ace is high.
/// - Best hand is 7-5-4-3-2 offsuit.
pub fn std_deck_lowball27_eval(cards: &StdDeckCardMask, _n_cards: usize) -> HandVal {
    let ss: u32 = cards.spades().into(); // Ranks for Spades
    let sc: u32 = cards.clubs().into(); // Ranks for Clubs
    let sd: u32 = cards.diamonds().into(); // Ranks for Diamonds
    let sh: u32 = cards.hearts().into(); // Ranks for Hearts

    let ranks = sc | ss | sd | sh;
    let n_ranks = ranks.count_ones() as u8;
    let n_dups = _n_cards - n_ranks as usize;
    let retval: Option<HandVal> = None;

    if n_ranks >= 5 {
        for suit in [ss, sc, sd, sh].iter() {
            if suit.count_ones() >= 5 {
                let (top, second, third, fourth, fifth) = extract_top_five_cards_lowball(*suit);
                if STRAIGHT_TABLE[*suit as usize] != 0 {
                    let retval =
                        HandVal::new(HandType::StFlush as u8, top, second, third, fourth, fifth);
                    return retval;
                } else {
                    let retval =
                        HandVal::new(HandType::Flush as u8, top, second, third, fourth, fifth);
                    return retval;
                }
            }
        }
        let st = STRAIGHT_TABLE[ranks as usize];
        if st != 0 {
            // Straight found
            let top_card = st;
            let second_card = if top_card > 0 { top_card - 1 } else { 0 };
            let third_card = if top_card > 1 { top_card - 2 } else { 0 };
            let fourth_card = if top_card > 2 { top_card - 3 } else { 0 };
            let fifth_card = if top_card > 3 { top_card - 4 } else { 0 };

            let retval = HandVal::new(
                HandType::Straight as u8,
                top_card,
                second_card,
                third_card,
                fourth_card,
                fifth_card,
            );
            return retval;
        }
        // Check if hand_val is set and if n_dups < 3
        if let Some(val) = retval {
            if n_dups < 3 {
                return val;
            }
        }
    }

    match n_dups {
        0 => {
            // No pair hand

            HandVal {
                value: (HandType::NoPair as u32) + TOP_FIVE_CARDS_TABLE[ranks as usize],
            }
        }
        1 => {
            // One pair hand
            let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
            let pair_card = TOP_CARD_TABLE[two_mask as usize];
            let kickers_mask = ranks ^ two_mask;
            //let (kicker1, kicker2, kicker3, _, _) = extract_top_five_cards_lowball(kickers_mask);
            // Get the kickers using the topFiveCardsTable
            let kickers =
                TOP_FIVE_CARDS_TABLE[kickers_mask as usize] >> CARD_WIDTH & !FIFTH_CARD_MASK;

            let kicker1_mask = CARD_MASK << SECOND_CARD_SHIFT;
            let kicker2_mask = CARD_MASK << THIRD_CARD_SHIFT;
            let kicker3_mask = CARD_MASK << FOURTH_CARD_SHIFT;

            let kicker1 = (kickers & kicker1_mask) >> SECOND_CARD_SHIFT;
            let kicker2 = (kickers & kicker2_mask) >> THIRD_CARD_SHIFT;
            let kicker3 = (kickers & kicker3_mask) >> FOURTH_CARD_SHIFT;

            HandVal::new(
                HandType::OnePair as u8,
                pair_card,
                kicker1 as u8,
                kicker2 as u8,
                kicker3 as u8,
                0,
            )
        }
        2 => {
            let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
            let _t = ranks ^ two_mask;

            if two_mask == 0 {
                // Three of a kind
                let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                let trips_card = TOP_CARD_TABLE[three_mask as usize];
                let kickers_mask = ranks ^ three_mask;
                let (kicker1, kicker2, _, _, _) = extract_top_five_cards_lowball(kickers_mask);

                HandVal::new(HandType::Trips as u8, trips_card, kicker1, kicker2, 0, 0)
            } else {
                // Two pair
                let pair1 = TOP_CARD_TABLE[two_mask as usize];
                let pair2 = TOP_CARD_TABLE[(two_mask ^ (1 << pair1)) as usize];
                let kicker = TOP_CARD_TABLE[(ranks ^ (1 << pair1) ^ (1 << pair2)) as usize];

                HandVal::new(
                    HandType::TwoPair as u8,
                    pair1.max(pair2),
                    pair1.min(pair2),
                    kicker,
                    0,
                    0,
                )
            }
        }

        _ => {
            // Possible quads, fullhouse, straight or flush, or two pair
            let four_mask = sh & sd & sc & ss;
            if four_mask != 0 {
                // Four of a kind (Quads)
                let quads_card = TOP_CARD_TABLE[four_mask as usize];
                let kicker = TOP_CARD_TABLE[(ranks ^ (1 << quads_card)) as usize];
                HandVal::new(HandType::Quads as u8, quads_card, kicker, 0, 0, 0)
            } else {
                // Check for full house or two pair
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                if three_mask != 0 {
                    // Full house
                    let fullhouse_card = TOP_CARD_TABLE[three_mask as usize];
                    let pair_card = TOP_CARD_TABLE[(two_mask ^ (1 << fullhouse_card)) as usize];

                    HandVal::new(
                        HandType::FullHouse as u8,
                        fullhouse_card,
                        pair_card,
                        0,
                        0,
                        0,
                    )
                } else {
                    // Two pair
                    let pair1 = TOP_CARD_TABLE[two_mask as usize];
                    let pair2 = TOP_CARD_TABLE[(two_mask ^ (1 << pair1)) as usize];
                    let kicker = TOP_CARD_TABLE[(ranks ^ (1 << pair1) ^ (1 << pair2)) as usize];

                    HandVal::new(
                        HandType::TwoPair as u8,
                        pair1.max(pair2),
                        pair1.min(pair2),
                        kicker,
                        0,
                        0,
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::StdDeck;
    use crate::handval_low::HANDTYPE_SHIFT;

    fn create_mask_from_string(cards_str: &str) -> StdDeckCardMask {
        let (mask, _) = StdDeck::string_to_mask(cards_str).expect("Failed to create mask");
        mask
    }
    fn calculate_rank_mask(cards: &StdDeckCardMask) -> u32 {
        let ranks = [
            cards.spades(),
            cards.clubs(),
            cards.diamonds(),
            cards.hearts(),
        ];

        ranks
            .iter()
            .fold(0, |acc, &suit_ranks| acc | suit_ranks as u32)
    }

    #[test]
    fn test_no_pair() {
        let mask = create_mask_from_string("2s3d4c7h8s");
        let rank_mask = calculate_rank_mask(&mask);
        let result = std_deck_lowball27_eval(&mask, 5);
        let expected_value = TOP_FIVE_CARDS_TABLE[rank_mask as usize];
        assert_eq!(
            result.value, expected_value,
            "Failed to evaluate no pair hand"
        );
    }

    #[test]
    fn test_no_pair_withace() {
        let mask = create_mask_from_string("As3d4c7h8s");
        let rank_mask = calculate_rank_mask(&mask);
        let result = std_deck_lowball27_eval(&mask, 5);
        let expected_value = TOP_FIVE_CARDS_TABLE[rank_mask as usize];
        assert_eq!(
            result.value, expected_value,
            "Failed to evaluate no pair hand"
        );
    }

    #[test]
    fn test_one_pair() {
        let mask = create_mask_from_string("2s2d4c7h8s");
        let result = std_deck_lowball27_eval(&mask, 5);

        // Extract hand type from the result
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(
            hand_type,
            HandType::OnePair as u32,
            "Failed to detect one pair"
        );
    }

    #[test]
    fn test_two_pair() {
        let mask = create_mask_from_string("2s2d3c3h8s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(
            hand_type,
            HandType::TwoPair as u32,
            "Failed to detect two pair"
        );
    }

    #[test]
    fn test_three_of_a_kind() {
        let mask = create_mask_from_string("2s2d2c7h8s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        println!("hand_type: {}", hand_type);
        assert_eq!(
            hand_type,
            HandType::Trips as u32,
            "Failed to detect three of a kind"
        );
    }

    #[test]
    fn test_straight() {
        let mask = create_mask_from_string("2s3d4c5h6s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(
            hand_type,
            HandType::Straight as u32,
            "Failed to detect straight"
        );
    }

    #[test]
    fn test_flush() {
        let mask = create_mask_from_string("2s4s6s8sTs");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(hand_type, HandType::Flush as u32, "Failed to detect flush");
    }

    #[test]
    fn test_full_house() {
        let mask = create_mask_from_string("2s2d2c3h3s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(
            hand_type,
            HandType::FullHouse as u32,
            "Failed to detect full house"
        );
    }

    #[test]
    fn test_four_of_a_kind() {
        let mask = create_mask_from_string("2s2d2c2h8s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(
            hand_type,
            HandType::Quads as u32,
            "Failed to detect four of a kind"
        );
    }

    #[test]
    fn test_straight_flush() {
        let mask = create_mask_from_string("2s3s4s5s6s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(
            hand_type,
            HandType::StFlush as u32,
            "Failed to detect straight flush"
        );
    }

    #[test]
    fn test_2_3_4_5_7_is_best_27_lowball_hand() {
        // In 2-7 lowball, straights and flushes count against you.
        // 2-3-4-5-7 (different suits) is the best hand because it avoids a straight.
        let best = create_mask_from_string("2s3d4c5h7s");
        let best_val = std_deck_lowball27_eval(&best, 5);
        let best_type = best_val.value >> HANDTYPE_SHIFT;

        // 2-3-4-5-7 should be NoPair (no straight, no flush)
        assert_eq!(
            best_type,
            HandType::NoPair as u32,
            "2-3-4-5-7 offsuit should be NoPair in 2-7 lowball"
        );

        // 2-3-4-5-6 is a straight, which is worse in 2-7 lowball
        let straight = create_mask_from_string("2s3d4c5h6s");
        let straight_val = std_deck_lowball27_eval(&straight, 5);
        let straight_type = straight_val.value >> HANDTYPE_SHIFT;

        assert_eq!(
            straight_type,
            HandType::Straight as u32,
            "2-3-4-5-6 should be detected as a straight in 2-7 lowball"
        );

        // The NoPair hand (2-3-4-5-7) should be lower (better) than the straight (2-3-4-5-6)
        assert!(
            best_val < straight_val,
            "2-3-4-5-7 (NoPair) should beat 2-3-4-5-6 (Straight) in 2-7 lowball"
        );
    }

    #[test]
    fn test_ace_2_3_4_5_is_not_best_in_27_lowball() {
        // In 2-7 lowball, A-2-3-4-5 is a straight (Ace plays high as 5-high straight),
        // so it is NOT the best hand. 2-3-4-5-7 should beat it.
        let ace_wheel = create_mask_from_string("As2d3c4h5s");
        let ace_val = std_deck_lowball27_eval(&ace_wheel, 5);
        let ace_type = ace_val.value >> HANDTYPE_SHIFT;

        // A-2-3-4-5 should be a straight in 2-7 lowball (Ace is high, making A-5 straight)
        assert_eq!(
            ace_type,
            HandType::Straight as u32,
            "A-2-3-4-5 should be a straight in 2-7 lowball (straights count)"
        );

        // 2-3-4-5-7 offsuit is better (lower value)
        let best_27 = create_mask_from_string("2s3d4c5h7s");
        let best_val = std_deck_lowball27_eval(&best_27, 5);

        assert!(
            best_val < ace_val,
            "2-3-4-5-7 should beat A-2-3-4-5 in 2-7 lowball because A-2-3-4-5 is a straight"
        );
    }
}
