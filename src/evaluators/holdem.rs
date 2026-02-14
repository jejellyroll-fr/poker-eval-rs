use crate::handval::HandVal;
use crate::rules::HandType;
use crate::tables::t_cardmasks::StdDeckCardMask;

use crate::tables::t_straight::STRAIGHT_TABLE;
use crate::tables::t_topcard::TOP_CARD_TABLE;

/// Standard Texas Hold'em (and similar 5+ card games) evaluator.
pub struct Eval;

impl Eval {
    // extract_top_five_cards removed, using crate::rules::std::extract_top_five_cards

    /// Evaluates a hand of `n_cards` (5 to 7) for standard High poker rules.
    ///
    /// # Arguments
    ///
    /// * `cards` - A mask describing the cards to evaluate.
    /// * `n_cards` - The number of cards in the mask (must be >= 5).
    ///
    /// # Returns
    ///
    /// A `HandVal` representing the best 5-card hand found.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker_eval_rs::evaluators::HoldemEvaluator; // Access via typedef if strictly needed, or direct path
    /// // Better use the re-export if available, or imports:
    /// use poker_eval_rs::evaluators::Eval;
    /// use poker_eval_rs::deck::StdDeck;
    /// use poker_eval_rs::rules::HandType;
    ///
    /// let (mask, count) = StdDeck::string_to_mask("As Ks Qs Js Ts").unwrap();
    /// let val = Eval::eval_n(&mask, count);
    /// assert_eq!(val.hand_type(), HandType::StFlush as u8);
    /// ```
    /// O(1) non-flush hand evaluation.
    ///
    /// Computes: key = SUIT_HASH[ss] + SUIT_HASH[sc] + SUIT_HASH[sd] + SUIT_HASH[sh]
    ///
    /// If `large-table` is enabled:
    ///   Uses a single memory access: NOFLUSH_LOOKUP[key]
    /// If `compact-table` is enabled:
    ///   Uses zero-cost perfect hash: NOFLUSH_LOOKUP[key + ROW_OFFSETS[key >> 12]]
    #[cfg(all(feature = "large-table", not(feature = "compact-table")))]
    #[inline]
    pub fn get_non_flush_val(ss: u16, sc: u16, sd: u16, sh: u16) -> HandVal {
        use crate::tables::rank_lookup::{NOFLUSH_LOOKUP, SUIT_HASH};
        let key = SUIT_HASH[ss as usize]
            .wrapping_add(SUIT_HASH[sc as usize])
            .wrapping_add(SUIT_HASH[sd as usize])
            .wrapping_add(SUIT_HASH[sh as usize]);
        HandVal {
            value: NOFLUSH_LOOKUP[key as usize],
        }
    }

    #[cfg(feature = "compact-table")]
    #[inline]
    pub fn get_non_flush_val(ss: u16, sc: u16, sd: u16, sh: u16) -> HandVal {
        use crate::tables::rank_lookup::{
            NOFLUSH_LOOKUP, PERF_HASH_ROW_OFFSETS, PERF_HASH_ROW_SHIFT, SUIT_HASH,
        };
        let key = SUIT_HASH[ss as usize]
            .wrapping_add(SUIT_HASH[sc as usize])
            .wrapping_add(SUIT_HASH[sd as usize])
            .wrapping_add(SUIT_HASH[sh as usize]);
        let index = key.wrapping_add(PERF_HASH_ROW_OFFSETS[(key >> PERF_HASH_ROW_SHIFT) as usize]);
        HandVal {
            value: NOFLUSH_LOOKUP[index as usize],
        }
    }

    /// O(1) flush hand evaluation.
    /// Uses the 13-bit suit mask directly into FLUSH_LOOKUP.
    #[inline]
    pub fn get_flush_val(suit_mask: u16) -> HandVal {
        use crate::tables::rank_lookup::FLUSH_LOOKUP;
        HandVal {
            value: FLUSH_LOOKUP[suit_mask as usize],
        }
    }

    #[inline]
    pub fn eval_n(cards: &StdDeckCardMask, _n_cards: usize) -> HandVal {
        let ss = cards.spades();
        let sc = cards.clubs();
        let sd = cards.diamonds();
        let sh = cards.hearts();

        // Check for flush (5+ cards of same suit)
        for &suit in &[ss, sc, sd, sh] {
            if suit.count_ones() >= 5 {
                return Self::get_flush_val(suit);
            }
        }

        Self::get_non_flush_val(ss, sc, sd, sh)
    }

    /// Optimized 5-card evaluation that skips flush and straight flush checks.
    /// Useful for Omaha when the board triplet is already known to be non-monotone.
    #[inline]
    pub fn eval_5_non_flush(_ranks: u16, ss: u16, sc: u16, sd: u16, sh: u16) -> HandVal {
        Self::get_non_flush_val(ss, sc, sd, sh)
    }

    #[inline]
    pub fn find_flush_or_straight_flush(ss: u16, sc: u16, sd: u16, sh: u16) -> Option<HandVal> {
        for &suit in &[ss, sc, sd, sh] {
            if suit.count_ones() >= 5 {
                // Extract the five highest cards
                let (top, second, third, fourth, fifth) =
                    crate::rules::std::extract_top_five_cards(suit);

                // Check if the cards form a straight
                // STRAIGHT_TABLE maps a 13-bit rank mask to the highest rank of a straight if one exists,
                // or 0 otherwise. This allows O(1) straight detection.
                let st_rank = STRAIGHT_TABLE[suit as usize];
                if st_rank != 0 {
                    // It is a Straight Flush. Use the straight's top card, not the flush's top card.
                    return Some(HandVal::new(
                        HandType::StFlush as u8,
                        st_rank,
                        if st_rank > 0 { st_rank - 1 } else { 0 },
                        if st_rank > 1 { st_rank - 2 } else { 0 },
                        if st_rank > 2 { st_rank - 3 } else { 0 },
                        if st_rank > 3 { st_rank - 4 } else { 0 },
                    ));
                } else {
                    return Some(HandVal::new(
                        HandType::Flush as u8,
                        top,
                        second,
                        third,
                        fourth,
                        fifth,
                    ));
                }
            }
        }
        None
    }

    #[inline]
    pub fn find_straight(ranks: u16) -> Option<HandVal> {
        let st = STRAIGHT_TABLE[ranks as usize];
        if st != 0 {
            // Straight found
            let top_card = st;
            let second_card = if top_card > 0 { top_card - 1 } else { 0 };
            let third_card = if top_card > 1 { top_card - 2 } else { 0 };
            let fourth_card = if top_card > 2 { top_card - 3 } else { 0 };
            let fifth_card = if top_card > 3 { top_card - 4 } else { 0 };

            return Some(HandVal::new(
                HandType::Straight as u8,
                top_card,
                second_card,
                third_card,
                fourth_card,
                fifth_card,
            ));
        }
        None
    }

    #[inline]
    pub fn evaluate_duplicates(
        n_dups: usize,
        ranks: u16,
        sc: u16,
        sd: u16,
        sh: u16,
        ss: u16,
    ) -> HandVal {
        match n_dups {
            0 => {
                // No pair hand — use the fast lookup path
                Self::get_non_flush_val(ss, sc, sd, sh)
            }
            1 => {
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                if two_mask.count_ones() as usize != n_dups {
                    // Three of a kind
                    let three_card = TOP_CARD_TABLE[two_mask as usize];
                    let kickers_mask = ranks ^ two_mask;
                    let k1 = TOP_CARD_TABLE[kickers_mask as usize];
                    let k2 = TOP_CARD_TABLE[(kickers_mask ^ (1 << k1)) as usize];

                    HandVal::new(HandType::Trips as u8, three_card, k1, k2, 0, 0)
                } else {
                    // One pair
                    let pair_card = TOP_CARD_TABLE[two_mask as usize];
                    let kickers_mask = ranks ^ two_mask;
                    let k1 = TOP_CARD_TABLE[kickers_mask as usize];
                    let k2 = TOP_CARD_TABLE[(kickers_mask ^ (1 << k1)) as usize];
                    let k3 = TOP_CARD_TABLE[(kickers_mask ^ (1 << k1) ^ (1 << k2)) as usize];

                    HandVal::new(HandType::OnePair as u8, pair_card, k1, k2, k3, 0)
                }
            }

            2 => {
                // Either two pair or three of a kind
                //
                // Bitwise Logic for Pair Detection:
                // `ranks` matches any rank present (1+ cards).
                // `sc ^ sd ^ sh ^ ss` is the XOR sum of suit masks.
                // - 1 card (Single): XOR=1, ranks=1. XOR ^ ranks = 0.
                // - 2 cards (Pair): XOR=0, ranks=1. XOR ^ ranks = 1.
                // - 3 cards (Trips): XOR=1, ranks=1. XOR ^ ranks = 0.
                // - 4 cards (Quads): XOR=0, ranks=1. XOR ^ ranks = 1.
                // Thus, `two_mask` captures ranks with exactly 2 OR 4 cards.
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                if two_mask != 0 {
                    // Two pair
                    // Isolate the lowest set bit (lowest rank pair) using x & (-x)
                    // In two's complement, -x inverts bits and adds 1, so x & -x keeps only the lowest bit.
                    let pair1_mask = two_mask & (-(two_mask as i16)) as u16; // Mask for the first pair
                    let pair2_mask = two_mask & (!(pair1_mask) & two_mask); // Mask for the second pair
                    let kickers_mask = ranks ^ two_mask; // Mask for the kickers
                    let pair1_top_card = TOP_CARD_TABLE[pair1_mask as usize];
                    let pair2_top_card = TOP_CARD_TABLE[pair2_mask as usize];
                    let kicker = TOP_CARD_TABLE[kickers_mask as usize];

                    HandVal::new(
                        HandType::TwoPair as u8,
                        pair1_top_card.max(pair2_top_card), // Higher pair
                        pair1_top_card.min(pair2_top_card), // Lower pair
                        kicker,
                        0,
                        0,
                    )
                } else {
                    // Three of a kind
                    let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                    let trips_card = TOP_CARD_TABLE[three_mask as usize];
                    let kickers_mask = ranks ^ three_mask;
                    let k1 = TOP_CARD_TABLE[kickers_mask as usize];
                    let k2 = TOP_CARD_TABLE[(kickers_mask ^ (1 << k1)) as usize];

                    HandVal::new(HandType::Trips as u8, trips_card, k1, k2, 0, 0)
                }
            }
            _ => {
                // Four of a kind (Quads)
                let four_mask = sh & sd & sc & ss;
                if four_mask != 0 {
                    let tc = TOP_CARD_TABLE[four_mask as usize];
                    let hand_val = HandVal::new(
                        HandType::Quads as u8,
                        tc,
                        TOP_CARD_TABLE[(ranks ^ (1 << tc)) as usize],
                        0,
                        0,
                        0,
                    );
                    return hand_val;
                }

                // Full House
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                if two_mask.count_ones() as usize != n_dups {
                    let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                    let tc = TOP_CARD_TABLE[three_mask as usize];
                    let t = (two_mask | three_mask) ^ (1 << tc);
                    let hand_val = HandVal::new(
                        HandType::FullHouse as u8,
                        tc,
                        TOP_CARD_TABLE[t as usize],
                        0,
                        0,
                        0,
                    );
                    return hand_val;
                }

                // Two Pair (with excess cards? n_dups > 2)
                let top = TOP_CARD_TABLE[two_mask as usize];
                let second = TOP_CARD_TABLE[(two_mask ^ (1 << top)) as usize];

                HandVal::new(
                    HandType::TwoPair as u8,
                    top,
                    second,
                    TOP_CARD_TABLE[(ranks ^ (1 << top) ^ (1 << second)) as usize],
                    0,
                    0,
                )
            }
        }
    }

    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    /// Evaluates 8 hands in parallel using AVX2.
    ///
    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    /// Evaluates 8 hands in parallel using AVX2.
    ///
    /// # Safety
    /// This function requires AVX2 support.
    pub unsafe fn eval_8_hands(masks: &[StdDeckCardMask; 8]) -> [HandVal; 8] {
        use crate::tables::rank_lookup::{NOFLUSH_LOOKUP, SUIT_HASH};
        use std::arch::x86_64::*;

        let mut results = [HandVal { value: 0 }; 8];
        let mut keys = [0i32; 8];
        let mut flush_mask = 0u8;

        // 1. Unrolled Scalar Extraction and Key Summing
        // This is faster than AVX2 permutes for this specific bit layout as SUIT_HASH is always in L1.
        for i in 0..8 {
            let raw = masks[i].as_raw();
            let ss = (raw & 0x1FFF) as usize;
            let sc = ((raw >> 16) & 0x1FFF) as usize;
            let sd = ((raw >> 32) & 0x1FFF) as usize;
            let sh = ((raw >> 48) & 0x1FFF) as usize;

            if ss.count_ones() >= 5
                || sc.count_ones() >= 5
                || sd.count_ones() >= 5
                || sh.count_ones() >= 5
            {
                results[i] = Self::eval_n(&masks[i], 0);
                flush_mask |= 1 << i;
            } else {
                keys[i] = (SUIT_HASH[ss]
                    .wrapping_add(SUIT_HASH[sc])
                    .wrapping_add(SUIT_HASH[sd])
                    .wrapping_add(SUIT_HASH[sh])) as i32;
            }
        }

        // 2. Batch Gather for non-flush hands (Overlaps 8 memory requests for the 9.6MB lookup table)
        let keys_v = _mm256_loadu_si256(keys.as_ptr() as *const __m256i);
        let vals_v = _mm256_i32gather_epi32(NOFLUSH_LOOKUP.as_ptr() as *const i32, keys_v, 4);

        let mut gathered = [0u32; 8];
        _mm256_storeu_si256(gathered.as_mut_ptr() as *mut __m256i, vals_v);

        // 3. Merge results
        for i in 0..8 {
            if (flush_mask & (1 << i)) == 0 {
                results[i] = HandVal { value: gathered[i] };
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::StdDeck;
    use crate::rules::HandType;

    fn hand(s: &str) -> (StdDeckCardMask, usize) {
        StdDeck::string_to_mask(s).expect("Invalid hand string")
    }

    #[test]
    fn test_high_card() {
        let (cards, n) = hand("AsKdQcJs9h");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::NoPair as u8);
    }

    #[test]
    fn test_one_pair() {
        let (cards, n) = hand("AsAhKdQcJs");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::OnePair as u8);
    }

    #[test]
    fn test_two_pair() {
        let (cards, n) = hand("AsAhKdKcJs");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::TwoPair as u8);
    }

    #[test]
    fn test_trips() {
        let (cards, n) = hand("AsAhAdKcJs");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::Trips as u8);
    }

    #[test]
    fn test_straight() {
        let (cards, n) = hand("9s8h7d6c5s");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::Straight as u8);
    }

    #[test]
    fn test_broadway_straight() {
        let (cards, n) = hand("AsKhQdJcTs");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::Straight as u8);
    }

    #[test]
    fn test_wheel_straight() {
        let (cards, n) = hand("As2h3d4c5s");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::Straight as u8);
    }

    #[test]
    fn test_flush() {
        let (cards, n) = hand("AsKsQsJs9s");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::Flush as u8);
    }

    #[test]
    fn test_full_house() {
        let (cards, n) = hand("AsAhAdKcKs");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::FullHouse as u8);
    }

    #[test]
    fn test_quads() {
        let (cards, n) = hand("AsAhAdAcKs");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::Quads as u8);
    }

    #[test]
    fn test_straight_flush() {
        let (cards, n) = hand("9s8s7s6s5s");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::StFlush as u8);
    }

    #[test]
    fn test_royal_flush() {
        let (cards, n) = hand("AsKsQsJsTs");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::StFlush as u8);
    }

    #[test]
    fn test_seven_card_holdem_hand() {
        let (cards, n) = hand("AsKsQsJsTs2h3d");
        let result = Eval::eval_n(&cards, n);
        assert_eq!(result.hand_type(), HandType::StFlush as u8);
    }

    #[test]
    fn test_hand_comparison() {
        let (flush, n1) = hand("AsKsQsJs9s");
        let (straight, n2) = hand("AsKhQdJcTs");
        assert!(Eval::eval_n(&flush, n1) > Eval::eval_n(&straight, n2));
    }

    #[test]
    fn test_pair_vs_high_card() {
        let (pair, n1) = hand("2s2hKdQcJs");
        let (high_card, n2) = hand("AsKdQcJs9h");
        assert!(Eval::eval_n(&pair, n1) > Eval::eval_n(&high_card, n2));
    }

    #[test]
    fn test_hand_ordering() {
        let hands = [
            ("AsKdQcJs9h", HandType::NoPair),    // high card
            ("AsAhKdQcJs", HandType::OnePair),   // pair
            ("AsAhKdKcJs", HandType::TwoPair),   // two pair
            ("AsAhAdKcJs", HandType::Trips),     // trips
            ("9s8h7d6c5s", HandType::Straight),  // straight
            ("AsKsQsJs9s", HandType::Flush),     // flush
            ("AsAhAdKcKs", HandType::FullHouse), // full house
            ("AsAhAdAcKs", HandType::Quads),     // quads
            ("9s8s7s6s5s", HandType::StFlush),   // straight flush
        ];

        let vals: Vec<_> = hands
            .iter()
            .map(|(s, _)| {
                let (cards, n) = hand(s);
                Eval::eval_n(&cards, n)
            })
            .collect();

        for i in 1..vals.len() {
            assert!(
                vals[i] > vals[i - 1],
                "Hand type {:?} should beat {:?}, got {} <= {}",
                hands[i].1,
                hands[i - 1].1,
                vals[i].value,
                vals[i - 1].value
            );
        }
    }

    #[test]
    fn test_within_category_ordering() {
        let (aces, n1) = hand("AsAhKdQcJs");
        let (kings, n2) = hand("KsKhAdQcJs");
        assert!(Eval::eval_n(&aces, n1) > Eval::eval_n(&kings, n2));

        let (ace_flush, n3) = hand("AsKsQsJs9s");
        let (king_flush, n4) = hand("KsQsJs9s8s");
        assert!(Eval::eval_n(&ace_flush, n3) > Eval::eval_n(&king_flush, n4));

        let (broadway, n5) = hand("AsKhQdJcTs");
        let (wheel, n6) = hand("As2h3d4c5s");
        assert!(Eval::eval_n(&broadway, n5) > Eval::eval_n(&wheel, n6));
    }

    #[test]
    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    fn test_eval_8_hands_simd() {
        let (h1, _) = hand("AsKsQsJsTs"); // Royal Flush
        let (h2, _) = hand("AsAhAdAcKs"); // Quads
        let (h3, _) = hand("AsAhAdKsKc"); // Full House
        let (h4, _) = hand("2s3s4s5s9s"); // Flush
        let (h5, _) = hand("9s8h7d6c5s"); // Straight
        let (h6, _) = hand("AsAhAdKcJs"); // Trips
        let (h7, _) = hand("AsAhKdKcJs"); // Two Pair
        let (h8, _) = hand("AsKdQcJs9h"); // High Card

        let masks = [h1, h2, h3, h4, h5, h6, h7, h8];
        let results = unsafe { Eval::eval_8_hands(&masks) };

        assert_eq!(results[0].hand_type(), HandType::StFlush as u8);
        assert_eq!(results[1].hand_type(), HandType::Quads as u8);
        assert_eq!(results[2].hand_type(), HandType::FullHouse as u8);
        assert_eq!(results[3].hand_type(), HandType::Flush as u8);
        assert_eq!(results[4].hand_type(), HandType::Straight as u8);
        assert_eq!(results[5].hand_type(), HandType::Trips as u8);
        assert_eq!(results[6].hand_type(), HandType::TwoPair as u8);
        assert_eq!(results[7].hand_type(), HandType::NoPair as u8);

        // Compare with scalar eval_n
        for i in 0..8 {
            assert_eq!(results[i].value, Eval::eval_n(&masks[i], 0).value);
        }
    }
}
