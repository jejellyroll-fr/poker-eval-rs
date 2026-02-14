use super::holdem::Eval;
use super::lowball8::std_deck_lowball8_eval;
use crate::enumerate::PokerError;
use crate::handval::HandVal;
use crate::handval_low::{LowHandVal, LOW_HAND_VAL_NOTHING};
use crate::tables::t_cardmasks::StdDeckCardMask;

/// Minimum hole cards required (2).
pub const OMAHA_MINHOLE: usize = 4;
/// Maximum hole cards (unused).
pub const OMAHA_MAXHOLE: usize = 6;
/// Minimum board cards (3).
pub const OMAHA_MINBOARD: usize = 3;
/// Maximum board cards (5).
pub const OMAHA_MAXBOARD: usize = 5;

#[derive(Clone, Copy)]
struct PartialHand {
    ranks: u16,
    ss: u16,
    sc: u16,
    sd: u16,
    sh: u16,
}

impl PartialHand {
    #[inline]
    fn from_mask(mask: StdDeckCardMask) -> Self {
        let ss = mask.spades();
        let sc = mask.clubs();
        let sd = mask.diamonds();
        let sh = mask.hearts();
        PartialHand {
            ranks: ss | sc | sd | sh,
            ss,
            sc,
            sd,
            sh,
        }
    }

    #[inline]
    fn combine(self, other: Self) -> Self {
        PartialHand {
            ranks: self.ranks | other.ranks,
            ss: self.ss | other.ss,
            sc: self.sc | other.sc,
            sd: self.sd | other.sd,
            sh: self.sh | other.sh,
        }
    }
}

/// Evaluates a hand for Omaha Hi/Lo (8-or-better).
///
/// Ensures exactly 2 cards from hole and 3 cards from board are used.
#[inline]
pub fn std_deck_omaha_hi_low8_eval(
    hole: StdDeckCardMask,
    board: StdDeckCardMask,
    hival: &mut Option<HandVal>,
    loval: &mut Option<LowHandVal>,
) -> Result<(), PokerError> {
    let allcards = hole | board;
    let n_cards = allcards.num_cards();
    let allval = std_deck_lowball8_eval(&allcards, n_cards);

    let has_low_potential = if let Some(val) = allval {
        val.value != LOW_HAND_VAL_NOTHING
    } else {
        false
    };

    if !has_low_potential {
        *loval = None;
    } else {
        *loval = allval;
    }

    let mut hole_cards = [StdDeckCardMask::from_raw(0); 6];
    let mut hole_count = 0;
    let mut temp_hole = hole.as_raw();
    while temp_hole != 0 {
        let bit = temp_hole & (!temp_hole + 1);
        if hole_count < 6 {
            hole_cards[hole_count] = StdDeckCardMask::from_raw(bit);
            hole_count += 1;
        }
        temp_hole ^= bit;
    }

    let mut board_cards = [StdDeckCardMask::from_raw(0); 5];
    let mut board_count = 0;
    let mut temp_board = board.as_raw();
    while temp_board != 0 {
        let bit = temp_board & (!temp_board + 1);
        if board_count < 5 {
            board_cards[board_count] = StdDeckCardMask::from_raw(bit);
            board_count += 1;
        }
        temp_board ^= bit;
    }

    let mut hole_pairs_mask = [StdDeckCardMask::from_raw(0); 15];
    let mut hole_pairs = [PartialHand {
        ranks: 0,
        ss: 0,
        sc: 0,
        sd: 0,
        sh: 0,
    }; 15];
    let mut hp_count = 0;
    for i in 0..hole_count {
        for j in i + 1..hole_count {
            let mask = hole_cards[i] | hole_cards[j];
            hole_pairs_mask[hp_count] = mask;
            hole_pairs[hp_count] = PartialHand::from_mask(mask);
            hp_count += 1;
        }
    }

    let mut best_hi_val = 0u32;

    for k in 0..board_count {
        for l in k + 1..board_count {
            for m in l + 1..board_count {
                let triplet_mask = board_cards[k] | board_cards[l] | board_cards[m];
                let tri = PartialHand::from_mask(triplet_mask);

                let tri_ss_count = tri.ss.count_ones();
                let tri_sc_count = tri.sc.count_ones();
                let tri_sd_count = tri.sd.count_ones();
                let tri_sh_count = tri.sh.count_ones();

                let monotone_suit_idx = if tri_ss_count == 3 {
                    Some(0)
                } else if tri_sc_count == 3 {
                    Some(1)
                } else if tri_sd_count == 3 {
                    Some(2)
                } else if tri_sh_count == 3 {
                    Some(3)
                } else {
                    None
                };

                for i in 0..hp_count {
                    let hp = hole_pairs[i];
                    let combined = tri.combine(hp);

                    let hi = if let Some(suit_idx) = monotone_suit_idx {
                        let hp_suit_mask = match suit_idx {
                            0 => hp.ss,
                            1 => hp.sc,
                            2 => hp.sd,
                            _ => hp.sh,
                        };
                        if hp_suit_mask.count_ones() == 2 {
                            let mut v = Eval::eval_5_non_flush(
                                combined.ranks,
                                combined.ss,
                                combined.sc,
                                combined.sd,
                                combined.sh,
                            );
                            if let Some(flush_val) = Eval::find_flush_or_straight_flush(
                                combined.ss,
                                combined.sc,
                                combined.sd,
                                combined.sh,
                            ) {
                                if flush_val.value > v.value {
                                    v = flush_val;
                                }
                            }
                            v
                        } else {
                            Eval::eval_5_non_flush(
                                combined.ranks,
                                combined.ss,
                                combined.sc,
                                combined.sd,
                                combined.sh,
                            )
                        }
                    } else {
                        Eval::eval_5_non_flush(
                            combined.ranks,
                            combined.ss,
                            combined.sc,
                            combined.sd,
                            combined.sh,
                        )
                    };

                    if hi.value > best_hi_val {
                        best_hi_val = hi.value;
                    }

                    if has_low_potential {
                        let potential_hand = triplet_mask | hole_pairs_mask[i];
                        let cur_lo = std_deck_lowball8_eval(&potential_hand, 5);
                        if let Some(cur_lo_val) = cur_lo {
                            if let Some(best_lo_val) = *loval {
                                if cur_lo_val < best_lo_val {
                                    *loval = Some(cur_lo_val);
                                }
                            } else {
                                *loval = Some(cur_lo_val);
                            }
                        }
                    }
                }
            }
        }
    }

    *hival = Some(HandVal { value: best_hi_val });
    Ok(())
}

/// Evaluates a hand for Omaha High only.
///
/// Optimized: pre-computes SUIT_HASH values for hole pairs and board triplets,
/// then combines them in the inner loop with simple addition + one table lookup.
#[inline]
pub fn std_deck_omaha_hi_eval(
    hole: StdDeckCardMask,
    board: StdDeckCardMask,
    hival: &mut Option<HandVal>,
) -> Result<(), PokerError> {
    #[cfg(feature = "compact-table")]
    use crate::tables::rank_lookup::{
        FLUSH_LOOKUP, NOFLUSH_LOOKUP, PERF_HASH_ROW_OFFSETS, PERF_HASH_ROW_SHIFT, SUIT_HASH,
    };
    #[cfg(all(feature = "large-table", not(feature = "compact-table")))]
    use crate::tables::rank_lookup::{FLUSH_LOOKUP, NOFLUSH_LOOKUP, SUIT_HASH};

    let mut hole_cards = [StdDeckCardMask::from_raw(0); 6];
    let mut hole_count = 0;
    let mut temp_hole = hole.as_raw();
    while temp_hole != 0 {
        let bit = temp_hole & (!temp_hole + 1);
        if hole_count < 6 {
            hole_cards[hole_count] = StdDeckCardMask::from_raw(bit);
            hole_count += 1;
        }
        temp_hole ^= bit;
    }

    let mut board_cards = [StdDeckCardMask::from_raw(0); 5];
    let mut board_count = 0;
    let mut temp_board = board.as_raw();
    while temp_board != 0 {
        let bit = temp_board & (!temp_board + 1);
        if board_count < 5 {
            board_cards[board_count] = StdDeckCardMask::from_raw(bit);
            board_count += 1;
        }
        temp_board ^= bit;
    }

    // Pre-compute hole pair partial data: suit masks + SUIT_HASH per suit
    #[derive(Clone, Copy)]
    struct HolePairData {
        ss: u16,
        sc: u16,
        sd: u16,
        sh: u16,
        hash_s: u32,
        hash_c: u32,
        hash_d: u32,
        hash_h: u32,
    }

    let mut hole_pairs = [HolePairData {
        ss: 0,
        sc: 0,
        sd: 0,
        sh: 0,
        hash_s: 0,
        hash_c: 0,
        hash_d: 0,
        hash_h: 0,
    }; 15];
    let mut hp_count = 0;
    for i in 0..hole_count {
        for j in i + 1..hole_count {
            let m = hole_cards[i] | hole_cards[j];
            let ss = m.spades();
            let sc = m.clubs();
            let sd = m.diamonds();
            let sh = m.hearts();
            hole_pairs[hp_count] = HolePairData {
                ss,
                sc,
                sd,
                sh,
                hash_s: SUIT_HASH[ss as usize],
                hash_c: SUIT_HASH[sc as usize],
                hash_d: SUIT_HASH[sd as usize],
                hash_h: SUIT_HASH[sh as usize],
            };
            hp_count += 1;
        }
    }

    let mut best_hi_val = 0u32;

    for k in 0..board_count {
        for l in k + 1..board_count {
            for m in l + 1..board_count {
                let tri_mask = board_cards[k] | board_cards[l] | board_cards[m];
                let tri_ss = tri_mask.spades();
                let tri_sc = tri_mask.clubs();
                let tri_sd = tri_mask.diamonds();
                let tri_sh = tri_mask.hearts();

                // Pre-compute board triplet SUIT_HASH values
                let tri_hash_s = SUIT_HASH[tri_ss as usize];
                let tri_hash_c = SUIT_HASH[tri_sc as usize];
                let tri_hash_d = SUIT_HASH[tri_sd as usize];
                let tri_hash_h = SUIT_HASH[tri_sh as usize];

                // Check if board triplet is monotone (3 cards same suit → flush possible)
                let flush_suit: i8 = if tri_ss.count_ones() == 3 {
                    0
                } else if tri_sc.count_ones() == 3 {
                    1
                } else if tri_sd.count_ones() == 3 {
                    2
                } else if tri_sh.count_ones() == 3 {
                    3
                } else {
                    -1
                };

                for hp in hole_pairs.iter().take(hp_count) {
                    // Fast non-flush eval: add pre-computed SUIT_HASH values
                    let key = tri_hash_s
                        .wrapping_add(hp.hash_s)
                        .wrapping_add(tri_hash_c.wrapping_add(hp.hash_c))
                        .wrapping_add(tri_hash_d.wrapping_add(hp.hash_d))
                        .wrapping_add(tri_hash_h.wrapping_add(hp.hash_h));

                    #[cfg(all(feature = "large-table", not(feature = "compact-table")))]
                    let mut v = NOFLUSH_LOOKUP[key as usize];

                    #[cfg(feature = "compact-table")]
                    let mut v = {
                        let index = key.wrapping_add(
                            PERF_HASH_ROW_OFFSETS[(key >> PERF_HASH_ROW_SHIFT) as usize],
                        );
                        NOFLUSH_LOOKUP[index as usize]
                    };

                    // Check flush only if board triplet is monotone AND hole pair has 2 of that suit
                    if flush_suit >= 0 {
                        let hp_suit = match flush_suit {
                            0 => hp.ss,
                            1 => hp.sc,
                            2 => hp.sd,
                            _ => hp.sh,
                        };
                        if hp_suit.count_ones() == 2 {
                            let combined_suit = match flush_suit {
                                0 => tri_ss | hp.ss,
                                1 => tri_sc | hp.sc,
                                2 => tri_sd | hp.sd,
                                _ => tri_sh | hp.sh,
                            };
                            let flush_v = FLUSH_LOOKUP[combined_suit as usize];
                            if flush_v > v {
                                v = flush_v;
                            }
                        }
                    }

                    if v > best_hi_val {
                        best_hi_val = v;
                    }
                }
            }
        }
    }

    *hival = Some(HandVal { value: best_hi_val });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::StdDeck;

    #[test]
    fn test_omaha_high_hand_evaluation() {
        let hole_str = "As2dKhQh";
        let board_str = "JhTh4h3c5d";

        let (hole, _) =
            StdDeck::string_to_mask(hole_str).expect("Failed to convert hole string to mask");
        let (board, _) =
            StdDeck::string_to_mask(board_str).expect("Failed to convert board string to mask");

        let mut hival = None;
        std_deck_omaha_hi_eval(hole, board, &mut hival).expect("High hand evaluation failed");

        // Best hand: KhQh (hole) + JhTh4h (board) = King-high flush in hearts
        let hi = hival.expect("Should have a high hand");
        assert_eq!(
            hi.hand_type(),
            crate::rules::HandType::Flush as u8,
            "Expected Flush, got hand_type={}",
            hi.hand_type()
        );
    }

    #[test]
    fn test_omaha_low_hand_evaluation() {
        let hole_str = "As2dKhQh";
        let board_str = "JhTh4h3c5d";

        let (hole, _) =
            StdDeck::string_to_mask(hole_str).expect("Failed to convert hole string to mask");
        let (board, _) =
            StdDeck::string_to_mask(board_str).expect("Failed to convert board string to mask");

        let mut loval = None;
        std_deck_omaha_hi_low8_eval(hole, board, &mut None, &mut loval)
            .expect("Low hand evaluation failed");

        let expected_low_value = LowHandVal { value: 344865 }; // Value representing As2d4h3c5d (A->1, 5->5) -> 0x54321

        assert_eq!(
            loval,
            Some(expected_low_value),
            "Low hand value did not match expected value"
        );
    }

    #[test]
    fn test_omaha_must_use_exactly_two_hole_cards() {
        // Player has AhKhQhJh (4 hole cards), board has 2h3h4h5h6d
        // Board has a heart flush, but player must use exactly 2 hole cards.
        // With 2 heart hole cards + 3 heart board cards, player can make a flush.
        // But the best hand must be constructed from exactly 2 hole + 3 board.
        let hole_str = "AhKh9d8c";
        let board_str = "2h3h4hTsJs";

        let (hole, _) =
            StdDeck::string_to_mask(hole_str).expect("Failed to convert hole string to mask");
        let (board, _) =
            StdDeck::string_to_mask(board_str).expect("Failed to convert board string to mask");

        let mut hival = None;
        std_deck_omaha_hi_eval(hole, board, &mut hival).expect("High hand evaluation failed");

        let hi = hival.expect("Should have a high hand value");
        let hand_type = hi.hand_type();
        // With Ah Kh from hole and 2h 3h 4h from board, player has a flush
        assert_eq!(
            hand_type,
            crate::rules::HandType::Flush as u8,
            "Omaha should form a flush using exactly 2 hole cards (Ah Kh) and 3 board cards (2h 3h 4h)"
        );
    }

    #[test]
    fn test_omaha_flush_on_board_player_needs_flush_cards() {
        // Board has 5 hearts, but player has no hearts in hole cards.
        // In Omaha, player MUST use 2 hole cards, so player cannot use the board flush.
        let hole_str = "AsKdQcJc";
        let board_str = "2h3h4h5h6h";

        let (hole, _) =
            StdDeck::string_to_mask(hole_str).expect("Failed to convert hole string to mask");
        let (board, _) =
            StdDeck::string_to_mask(board_str).expect("Failed to convert board string to mask");

        let mut hival = None;
        std_deck_omaha_hi_eval(hole, board, &mut hival).expect("High hand evaluation failed");

        let hi = hival.expect("Should have a high hand value");
        let hand_type = hi.hand_type();
        // Player has no hearts, so cannot make a flush despite board being all hearts.
        // Best hand should NOT be a flush.
        assert_ne!(
            hand_type,
            crate::rules::HandType::Flush as u8,
            "Player without heart hole cards should not make a flush even with all-heart board"
        );
        assert_ne!(
            hand_type,
            crate::rules::HandType::StFlush as u8,
            "Player without heart hole cards should not make a straight flush"
        );
    }

    #[test]
    fn test_omaha_hi_lo_no_qualifying_low() {
        // All cards are 9 or higher - no low hand should qualify (8-or-better)
        let hole_str = "AsKdQcJc";
        let board_str = "Th9hKhQhJs";

        let (hole, _) =
            StdDeck::string_to_mask(hole_str).expect("Failed to convert hole string to mask");
        let (board, _) =
            StdDeck::string_to_mask(board_str).expect("Failed to convert board string to mask");

        let mut hival = None;
        let mut loval = None;
        std_deck_omaha_hi_low8_eval(hole, board, &mut hival, &mut loval)
            .expect("Hi-lo evaluation failed");

        // High hand should exist
        assert!(hival.is_some(), "High hand should be evaluated");

        // Low hand should not qualify (all cards are 9+, no 8-or-better low possible)
        let low_is_nothing = loval.is_none()
            || loval
                == Some(LowHandVal {
                    value: LOW_HAND_VAL_NOTHING,
                });
        assert!(
            low_is_nothing,
            "Low hand should not qualify when all cards are 9 or higher"
        );
    }

    #[test]
    fn test_omaha_hi_lo_qualifying_low() {
        // Cards that can make an 8-or-better low
        let hole_str = "As2dKhQh";
        let board_str = "3c4c5dTsJs";

        let (hole, _) =
            StdDeck::string_to_mask(hole_str).expect("Failed to convert hole string to mask");
        let (board, _) =
            StdDeck::string_to_mask(board_str).expect("Failed to convert board string to mask");

        let mut hival = None;
        let mut loval = None;
        std_deck_omaha_hi_low8_eval(hole, board, &mut hival, &mut loval)
            .expect("Hi-lo evaluation failed");

        // Both high and low should exist
        assert!(hival.is_some(), "High hand should be evaluated");

        // Low hand should qualify (A-2 from hole + 3-4-5 from board = A-2-3-4-5)
        assert!(loval.is_some(), "Low hand should be present");
        let lo = loval.unwrap();
        assert_ne!(
            lo.value, LOW_HAND_VAL_NOTHING,
            "Low hand should qualify with A-2-3-4-5 available"
        );
    }
}

#[cfg(test)]
mod tests_extra {
    use super::*;
    use crate::deck::StdDeck;
    use crate::handval_low::LOW_HAND_VAL_NOTHING;
    use crate::rules::HandType;

    fn eval_omaha(hole_str: &str, board_str: &str) -> (Option<HandVal>, Option<LowHandVal>) {
        let (hole, _) = StdDeck::string_to_mask(hole_str).expect("failed to parse hole");
        let (board, _) = StdDeck::string_to_mask(board_str).expect("failed to parse board");
        let mut hival = None;
        let mut loval = None;
        std_deck_omaha_hi_low8_eval(hole, board, &mut hival, &mut loval).unwrap();
        (hival, loval)
    }

    #[test]
    fn test_omaha_scoop() {
        let (hi, lo) = eval_omaha("As2s3d4d", "5s6s7s8d9d");
        assert!(hi.is_some());
        assert!(lo.is_some());

        assert_eq!(hi.unwrap().hand_type(), HandType::Flush as u8);
        assert_ne!(lo.unwrap().value, LOW_HAND_VAL_NOTHING);
    }

    #[test]
    fn test_omaha_no_low() {
        let (hi, lo) = eval_omaha("AsKsQsJs", "Ts9s8s7d6d");
        assert!(hi.is_some());

        if let Some(l) = lo {
            assert_eq!(l.value, LOW_HAND_VAL_NOTHING);
        }
    }

    #[test]
    fn test_omaha_high_quads() {
        // Need 2 hole cards and 3 board cards to make Quads.
        // Hole: As Ad Kh Qh
        // Board: Ac Ah 2d 3d 4d
        // Use As Ad (hole) + Ac Ah 2d (board) -> Quads
        let (hi, _) = eval_omaha("AsAdKhQh", "AcAh2d3d4d");
        assert!(hi.is_some());
        assert_eq!(hi.unwrap().hand_type(), HandType::Quads as u8);
    }

    #[test]
    fn test_omaha_low_nut() {
        let (_, lo) = eval_omaha("As2dKhQd", "3c4s5hJdTd");
        assert!(lo.is_some());
        let l = lo.unwrap();
        assert_ne!(l.value, LOW_HAND_VAL_NOTHING);
    }
}
