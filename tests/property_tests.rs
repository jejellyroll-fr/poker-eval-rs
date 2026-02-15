use poker_eval_rs::board::BoardTexture;
use poker_eval_rs::deck::{StdDeck, StdDeckCardMask};
use poker_eval_rs::evaluators::Eval;
use proptest::prelude::*;

// Helper to create a mask from a list of card indices (0-51)
fn mask_from_indices(indices: &[u8]) -> StdDeckCardMask {
    let mut mask = StdDeckCardMask::new();
    for &idx in indices {
        mask.set(StdDeck::make_card(
            poker_eval_rs::deck::Rank::new(idx % 13),
            poker_eval_rs::deck::Suit::new(idx / 13),
        ));
    }
    mask
}

proptest! {
    // Generate random 7-card hands and verify transitivity of comparison
    #[test]
    fn test_holdem_comparison_transitivity(
        cards1 in proptest::collection::vec(0u8..52, 7),
        cards2 in proptest::collection::vec(0u8..52, 7),
        cards3 in proptest::collection::vec(0u8..52, 7)
    ) {
        // Ensure unique cards within each hand?
        // Real deal deals without replacement from single deck.
        // Proptest might generate duplicates.
        // We'll just construct masks. Duplicates in input `vec` act as single set bit.
        // Note: comparing Hand values.

        let m1 = mask_from_indices(&cards1);
        let m2 = mask_from_indices(&cards2);
        let m3 = mask_from_indices(&cards3);

        // Only evaluate if we have at least 5 cards
        if m1.num_cards() >= 5 && m2.num_cards() >= 5 && m3.num_cards() >= 5 {
            let v1 = Eval::eval_n(&m1, m1.num_cards());
            let v2 = Eval::eval_n(&m2, m2.num_cards());
            let v3 = Eval::eval_n(&m3, m3.num_cards());

            // Transitivity: if v1 > v2 and v2 > v3, then v1 > v3
            if v1 > v2 && v2 > v3 {
                assert!(v1 > v3);
            }
            if v1 == v2 && v2 == v3 {
                assert_eq!(v1, v3);
            }
        }
    }

    #[test]
    fn test_board_texture_properties(
        cards in proptest::collection::vec(0u8..52, 3..=5)
    ) {
        let board = mask_from_indices(&cards);
        if board.num_cards() >= 3 {
            let texture = BoardTexture::analyze(&board);

            // Invariants
            // Can't be both rainbow and monotone (unless 3 cards?)
            // Rainbow: 3 distinct suits (flop), 4 distinct (turn).
            // Monotone: 1 suit.
            if texture.is_monotone {
                assert!(!texture.is_rainbow);
                assert!(!texture.is_two_tone);
                assert!(texture.has_flush_draw); // Monotone >= 3 cards same suit -> Flush draw ormade.
            }

            if texture.is_rainbow {
                assert!(!texture.is_monotone);
                assert!(!texture.is_two_tone);
                assert!(!texture.has_flush_draw); // Rainbow means max suit count = 1.
            }

            if texture.is_full_house {
                assert!(texture.is_paired);
                assert!(texture.is_trips);
            }

            if texture.is_quads {
                // Quads implies Pair. Does it imply Trips?
                // logic: count==4 -> four_count++.
                // texture analysis checks if three_count > 0.
                // If I have 4 aces, I have 0 cards with count==3. So is_trips=false.
                assert!(!texture.is_trips); // Based on current logic which bins counts exactly.
            }
        }
    }

    #[test]
    fn test_enum_result_merge_properties(
        ns1 in 0u32..1000,
        ns2 in 0u32..1000,
        win1 in 0u32..1000,
        win2 in 0u32..1000,
    ) {
        use poker_eval_rs::enumdefs::{EnumResult, Game};
        let mut res1 = EnumResult::new(Game::Holdem);
        res1.nsamples = ns1;
        res1.nwinhi[0] = win1;

        let mut res2 = EnumResult::new(Game::Holdem);
        res2.nsamples = ns2;
        res2.nwinhi[0] = win2;

        res1.merge(&res2);
        assert_eq!(res1.nsamples, ns1 + ns2);
        assert_eq!(res1.nwinhi[0], win1 + win2);
    }

    #[test]
    fn test_equity_sum_is_one(
        cards1 in proptest::collection::vec(0u8..52, 2),
        cards2 in proptest::collection::vec(0u8..52, 2),
    ) {
        use poker_eval_rs::enumerate::enum_sample;
        use poker_eval_rs::enumdefs::{EnumResult, Game};

        let m1 = mask_from_indices(&cards1);
        let m2 = mask_from_indices(&cards2);

        // Ensure distinct pockets
        let mut intersection = m1;
        intersection.and(&m2);
        if intersection.is_empty() && m1.num_cards() == 2 && m2.num_cards() == 2 {
            let pockets = vec![m1, m2];
            let board = StdDeckCardMask::new();
            let dead = StdDeckCardMask::new();
            let mut result = EnumResult::new(Game::Holdem);

            if enum_sample(Game::Holdem, &pockets, board, dead, 2, 0, 100, false, &mut result).is_ok()
                && result.nsamples > 0
            {
                let total_ev: f64 = result.ev.iter().sum::<f64>() / result.nsamples as f64;
                assert!((total_ev - 1.0).abs() < 1e-6, "Total EV should be 1.0, got {}", total_ev);
            }
        }
    }
}
