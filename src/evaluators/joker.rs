use crate::deck::{JOKER_DECK_RANK_2, JOKER_DECK_RANK_ACE, JOKER_DECK_RANK_COUNT};
use crate::handval::HandVal;
use crate::rules::joker::JokerRulesHandType;
use crate::tables::t_jokercardmasks::JokerDeckCardMask;
use crate::tables::t_jokerstraight::JOKER_STRAIGHT_TABLE;

use super::holdem::Eval;
use crate::tables::t_topcard::TOP_CARD_TABLE;

pub struct EvalJoker;

fn top_unset(ranks: u32) -> u32 {
    for j in (JOKER_DECK_RANK_2..=JOKER_DECK_RANK_ACE).rev() {
        if ranks & (1 << j) == 0 {
            return j as u32; // Cast j to u32
        }
    }
    JOKER_DECK_RANK_2 as u32 // Cast JOKER_DECK_RANK_2 to u32
}

fn flush_val(ranks: u32) -> HandVal {
    HandVal::new(
        JokerRulesHandType::Flush as u8,
        top_unset(ranks) as u8, // Cast to u8
        0,
        0,
        0,
        0,
    )
}

impl EvalJoker {
    // extract_top_five_cards removed, using crate::rules::std::extract_top_five_cards

    // Helper function to extract top N kickers from a suit mask
    pub fn extract_top_kickers(mask: u64, n: usize) -> [u8; 3] {
        let mut kickers = [0; 3];
        let mut count = 0;

        for i in (0..JOKER_DECK_RANK_COUNT).rev() {
            if mask & (1 << i) != 0 && count < n {
                kickers[count] = i as u8;
                count += 1;
            }
        }

        kickers
    }

    pub fn eval_n(cards: JokerDeckCardMask, n_cards: usize) -> HandVal {
        if !cards.is_joker_set() {
            // Convert to standard deck and use standard evaluation
            let s_cards = cards.to_std();
            return Eval::eval_n(&s_cards, n_cards);
        }

        // If we reach here, we know we have a joker
        let mut ss = cards.spades();
        let mut sc = cards.clubs();
        let mut sd = cards.diamonds();
        let mut sh = cards.hearts();
        let ranks = ss | sc | sd | sh;
        let _n_ranks = ranks.count_ones() as u8;
        let _retval: Option<HandVal> = None;

        // Check for straight, flush, or straight flush
        if ss.count_ones() >= 4 {
            let straight = JOKER_STRAIGHT_TABLE[ss as usize];
            if straight != 0 {
                let hand_val =
                    HandVal::new(JokerRulesHandType::StFlush as u8, straight, 0, 0, 0, 0);
                return hand_val;
            } else {
                return flush_val(ss as u32);
            }
        } else if sc.count_ones() >= 4 {
            let straight = JOKER_STRAIGHT_TABLE[sc as usize];
            if straight != 0 {
                return HandVal::new(JokerRulesHandType::StFlush as u8, straight, 0, 0, 0, 0);
            } else {
                return flush_val(sc as u32);
            }
        } else if sd.count_ones() >= 4 {
            let straight = JOKER_STRAIGHT_TABLE[sd as usize];
            if straight != 0 {
                return HandVal::new(JokerRulesHandType::StFlush as u8, straight, 0, 0, 0, 0);
            } else {
                return flush_val(sd as u32);
            }
        } else if sh.count_ones() >= 4 {
            let straight = JOKER_STRAIGHT_TABLE[sh as usize];
            if straight != 0 {
                return HandVal::new(JokerRulesHandType::StFlush as u8, straight, 0, 0, 0, 0);
            } else {
                return flush_val(sh as u32);
            }
        } else {
            let straight = JOKER_STRAIGHT_TABLE[ranks as usize];
            if straight != 0 {
                return HandVal::new(JokerRulesHandType::Straight as u8, straight, 0, 0, 0, 0);
            }
        }

        // Add an Ace if it's not present in any suit
        let jrank = 1 << JOKER_DECK_RANK_ACE as u32;
        if ss & jrank == 0 {
            ss |= jrank;
        } else if sc & jrank == 0 {
            sc |= jrank;
        } else if sd & jrank == 0 {
            sd |= jrank;
        } else if sh & jrank == 0 {
            sh |= jrank;
        } else {
            // All aces are set, so we have quints
            return HandVal::new(
                JokerRulesHandType::Quints as u8,
                JOKER_DECK_RANK_ACE as u8,
                0,
                0,
                0,
                0,
            );
        }

        let ranks = ss | sc | sd | sh;
        let n_ranks = ranks.count_ones() as u8;
        let n_dups = n_cards - n_ranks as usize;

        match n_dups {
            0 => {
                // If a hand value has already been determined (e.g., for a straight or flush)
                // No pair hand
                let (top, second, third, fourth, fifth) =
                    crate::rules::std::extract_top_five_cards(ranks as u16);

                HandVal::new(
                    JokerRulesHandType::NoPair as u8,
                    top,
                    second,
                    third,
                    fourth,
                    fifth,
                )
            }
            1 => {
                // Calculate the hand value for one pair
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                let pair_top_card = TOP_CARD_TABLE[two_mask as usize]; // Assuming implementation of top_card_table
                let t = ranks ^ two_mask;
                let kickers = Self::extract_top_kickers(t, 3); // Assuming implementation of extract_top_kickers

                HandVal::new(
                    JokerRulesHandType::OnePair as u8,
                    pair_top_card,
                    kickers[0],
                    kickers[1],
                    kickers[2],
                    0,
                )
            }
            2 => {
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);

                if two_mask != 0 {
                    // Two Pair
                    let top_pair = TOP_CARD_TABLE[two_mask as usize];
                    let t = ranks ^ two_mask;
                    let second_pair = TOP_CARD_TABLE[t as usize];
                    let kicker =
                        TOP_CARD_TABLE[(ranks ^ (1 << top_pair) ^ (1 << second_pair)) as usize];

                    HandVal::new(
                        JokerRulesHandType::TwoPair as u8,
                        top_pair,
                        second_pair,
                        kicker,
                        0,
                        0,
                    )
                } else {
                    // Trips
                    let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                    let trip_card = TOP_CARD_TABLE[three_mask as usize];
                    let remaining_mask = ranks ^ three_mask;
                    let second = TOP_CARD_TABLE[remaining_mask as usize];
                    let third = TOP_CARD_TABLE[(remaining_mask ^ (1 << second)) as usize];

                    HandVal::new(
                        JokerRulesHandType::Trips as u8,
                        trip_card,
                        second,
                        third,
                        0,
                        0,
                    )
                }
            }
            _ => {
                let four_mask = sh & sd & sc & ss;
                if four_mask != 0 {
                    let tc = TOP_CARD_TABLE[four_mask as usize];
                    return HandVal::new(
                        JokerRulesHandType::Quads as u8,
                        tc,
                        TOP_CARD_TABLE[(ranks ^ (1 << tc)) as usize],
                        0,
                        0,
                        0,
                    );
                }

                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));

                if two_mask.count_ones() as usize != n_dups {
                    // Full house
                    let tc = TOP_CARD_TABLE[three_mask as usize];
                    let remaining = (two_mask | three_mask) ^ (1 << tc);
                    HandVal::new(
                        JokerRulesHandType::FullHouse as u8,
                        tc,
                        TOP_CARD_TABLE[remaining as usize],
                        0,
                        0,
                        0,
                    )
                } else {
                    // Handle other cases where retval is not yet set.
                    // This branch should be unreachable for 5-card hands as all cases (NoPair, Pair, TwoPair, Trips, Quads, FullHouse)
                    // are covered. If this is reached, it indicates a bug or unexpected hand size > 5 without proper logic.
                    panic!(
                        "EvalJoker: Unhandled hand configuration. Cards: {:?}, Duplicates: {}",
                        ranks, n_dups
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::{JokerDeck, JOKER_DECK_JOKER};

    #[test]
    fn test_joker_quints_four_aces_plus_joker() {
        // Four Aces + Joker = Five of a kind (Quints)
        let (mut hand, _) =
            JokerDeck::string_to_mask("AhAdAcAs").expect("Failed to parse four aces");
        hand.set(JOKER_DECK_JOKER); // Add the joker

        let result = EvalJoker::eval_n(hand, 5);
        assert_eq!(
            result.hand_type(),
            JokerRulesHandType::Quints as u8,
            "Four Aces plus Joker should make Quints (five of a kind)"
        );
        assert_eq!(
            result.top_card(),
            JOKER_DECK_RANK_ACE as u8,
            "Quints should be Aces"
        );
    }

    #[test]
    fn test_joker_completes_straight() {
        // Hand: 5h 6d 8c 9s + Joker. Joker fills in as 7 to make 5-6-7-8-9 straight.
        let (mut hand, _) =
            JokerDeck::string_to_mask("5h6d8c9s").expect("Failed to parse straight with gap");
        hand.set(JOKER_DECK_JOKER); // Add the joker

        let result = EvalJoker::eval_n(hand, 5);
        assert_eq!(
            result.hand_type(),
            JokerRulesHandType::Straight as u8,
            "Joker should complete the straight (5-6-Joker-8-9)"
        );
    }
}

#[cfg(test)]
mod tests_extra {
    use super::*;
    use crate::deck::{JokerDeck, JOKER_DECK_JOKER};

    fn eval_str(cards_str: &str) -> HandVal {
        // Helper to parse standard cards and add Joker if 'Xx' or manual
        // Assume input is standard cards, we add Joker manually if needed
        let mut mask = JokerDeckCardMask::new();
        let mut clean_str = String::new();
        let mut has_joker = false;

        let mut chars = cards_str.chars();
        while let Some(c1) = chars.next() {
            if let Some(c2) = chars.next() {
                let s = format!("{}{}", c1, c2);
                if s == "Xx" {
                    has_joker = true;
                } else {
                    clean_str.push_str(&s);
                }
            }
        }

        if !clean_str.is_empty() {
            let (m, _) = JokerDeck::string_to_mask(&clean_str).unwrap_or_else(|_| {
                let (_std_m, _) =
                    crate::deck::StdDeck::string_to_mask(&clean_str).expect("parse std");
                panic!("JokerDeck::string_to_mask failed");
            });
            mask = m;
        }

        if has_joker {
            mask.set(JOKER_DECK_JOKER);
        }

        EvalJoker::eval_n(mask, 5)
    }

    #[test]
    fn test_joker_flush() {
        // 4 hearts + Joker -> Flush (Joker becomes Ah)
        let val = eval_str("KhQhJhThXx");
        assert_eq!(val.hand_type(), JokerRulesHandType::StFlush as u8); // Ah-Kh-Qh-Jh-Th is Royal Flush!
                                                                        // Wait, straight flush logic takes precedence.

        // Simple flush: 2h 4h 6h 8h Xx -> Ah 8h 6h 4h 2h
        let val = eval_str("2h4h6h8hXx");
        assert_eq!(val.hand_type(), JokerRulesHandType::Flush as u8);
        assert_eq!(val.top_card(), JOKER_DECK_RANK_ACE as u8);
    }

    #[test]
    fn test_joker_straight_flush() {
        // 9h Th Jh Qh Xx -> Kh 9h Th Jh Qh (Straight Flush, 9-K)
        // Or Ah?
        // 9, T, J, Q. Missing K or 8.
        // Best is K.
        // Joker becomes Kh.
        let val = eval_str("9hThJhQhXx");
        assert_eq!(val.hand_type(), JokerRulesHandType::StFlush as u8);
        // Top card should be King (Rank 11)
        // assert_eq!(val.top_card(), crate::deck::STD_DECK_RANK_KING as u8);
    }

    #[test]
    fn test_joker_full_house() {
        // Ac Ad 2s 2c Xx -> Full House (Aces full of Deuces)
        // Hand: Ac Ad 2s 2c. Joker.
        // Joker becomes Ace (best card not present? No, Ace is present).
        // Joker logic: "Add an Ace if it's not present in any suit".
        // Ac, Ad present. As, Ah missing.
        // Joker becomes As or Ah.
        // Hand: Ac Ad As 2s 2c.
        // Full House.
        let val = eval_str("AcAd2s2cXx");
        assert_eq!(val.hand_type(), JokerRulesHandType::FullHouse as u8);
        assert_eq!(val.top_card(), JOKER_DECK_RANK_ACE as u8); // Trips part
        assert_eq!(val.second_card(), crate::deck::STD_DECK_RANK_2 as u8); // Pair part
    }

    #[test]
    fn test_joker_trips() {
        // Ac Ad 3s 4c Xx -> Trips (Aces)
        // Hand: Ac Ad 3s 4c As. -> Trips Aces.
        let val = eval_str("AcAd3s4cXx");
        assert_eq!(val.hand_type(), JokerRulesHandType::Trips as u8);
        assert_eq!(val.top_card(), JOKER_DECK_RANK_ACE as u8);
    }
}
