use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::evaluators::{
    Eval, HandEvaluator, HoldemEvaluator, LowballEvaluator, OmahaHiEvaluator, OmahaHiLoEvaluator,
    ShortDeckEvaluator,
};
use poker_eval_rs::handval_low::LOW_HAND_VAL_NOTHING;
use poker_eval_rs::rules::HandType;

#[test]
fn test_variant_holdem() {
    // Royal Flush
    let (hole, _) = StdDeck::string_to_mask("AsKs").unwrap();
    let (board, _) = StdDeck::string_to_mask("QsJsTs").unwrap();
    let val = HoldemEvaluator::evaluate_hand(&hole, &board).unwrap();
    assert_eq!(val.hand_type(), HandType::StFlush as u8);
}

#[test]
fn test_variant_omaha() {
    // Quad Aces using 2 hole cards and 3 board cards
    // Hole: As Ac Ks Kc
    // Board: Ah Ad 2s 3s 4s
    // Best hand: As Ac Ah Ad K (Quads) - Wait, Omaha requires EXACTLY 2 from hole and 3 from board.
    // Hole: As Ac Ks Kc
    // Board: Ah Ad 2s 3s 4s
    // Hand: As Ac (hole) + Ah Ad 2s (board)? No, must use 3 board.
    // As Ac (hole) + Ah Ad 2s (board).
    // Wait, 3 board cards... Ah Ad 2s.
    // So As Ac Ah Ad 2s is valid. Quad Aces.

    let (hole, _) = StdDeck::string_to_mask("AsAcKsKc").unwrap();
    let (board, _) = StdDeck::string_to_mask("AhAd2s3s4s").unwrap();
    let val = OmahaHiEvaluator::evaluate_hand(&hole, &board)
        .unwrap()
        .unwrap();
    assert_eq!(val.hand_type(), HandType::Quads as u8);
}

#[test]
fn test_variant_omaha8_hilo() {
    // A-2-3-4-5 Low evaluation
    // Hole: As 2s 3d 4d
    // Board: 5s 6s 7s 8s 9s
    // Low: 2 cards from hole (As 2s) + 3 from board (5s 6s 7s)?
    // No, A-2-5-6-7 is a low.

    let (hole, _) = StdDeck::string_to_mask("As2sKDQD").unwrap();
    let (board, _) = StdDeck::string_to_mask("3s4s5s8h9h").unwrap();
    // Low: A-2 (hole) + 3-4-5 (board) = A-2-3-4-5 (Wheel)

    let (hi_opt, lo_opt) = OmahaHiLoEvaluator::evaluate_hand(&hole, &board).unwrap();

    assert!(hi_opt.is_some());
    assert!(lo_opt.is_some());

    // Check Straight Flush for High: As 2s 3s 4s 5s
    let hi = hi_opt.unwrap();
    assert_eq!(hi.hand_type(), HandType::StFlush as u8);

    // Check Low
    let lo = lo_opt.unwrap();
    assert_ne!(lo.value, LOW_HAND_VAL_NOTHING);
}

#[test]
fn test_variant_lowball_a5() {
    // A-2-3-4-5 is the best hand (Wheel), straights/flushes don't count against you in A-5
    let (hole, _) = StdDeck::string_to_mask("As2s").unwrap();
    let (board, _) = StdDeck::string_to_mask("3s4s5s").unwrap();

    let val = LowballEvaluator::evaluate_hand(&hole, &board).unwrap();
    assert_ne!(val.value, LOW_HAND_VAL_NOTHING);
    // In A-5, the value increases as the hand gets worse (or better, depending on encoding).
    // Usually we compare.

    let (hole2, _) = StdDeck::string_to_mask("KsQs").unwrap(); // K-Q-3-4-5 (Worse)
    let val2 = LowballEvaluator::evaluate_hand(&hole2, &board).unwrap();

    // Check which one is better. lowball8/lowball evaluators typically return values where
    // comparators work (smaller is better? or inverted logic?).
    // Usually HandVal/LowHandVal implement Ord correctly for the game.
    // For LowHandVal, typically lower raw value IS better low? Or Higher?
    // Let's assume Ord is implemented such that `better_hand > worse_hand`.

    assert!(
        val < val2,
        "Wheel should be better (lower numeric value) than King-high low"
    );
}

#[test]
fn test_variant_short_deck() {
    // Flush beats Full House
    let (hole_flush, _) = StdDeck::string_to_mask("AhKh").unwrap();
    let (board_flush, _) = StdDeck::string_to_mask("QhJh9h").unwrap(); // A-K-Q-J-9 Flush
    let val_flush = ShortDeckEvaluator::evaluate_hand(&hole_flush, &board_flush).unwrap();

    let (hole_fh, _) = StdDeck::string_to_mask("AsAc").unwrap();
    let (board_fh, _) = StdDeck::string_to_mask("AhKdKs").unwrap(); // A-A-A-K-K FH
    let val_fh = ShortDeckEvaluator::evaluate_hand(&hole_fh, &board_fh).unwrap();

    assert!(val_flush > val_fh);
    assert_eq!(val_flush.hand_type(), HandType::FullHouse as u8); // Swapped enum val
    assert_eq!(val_fh.hand_type(), HandType::Flush as u8); // Swapped enum val
}

#[test]
fn test_variant_stud7() {
    // Stud7 is essentially "best 5 of 7 cards".
    // We can simulate this using Eval::eval_n with 7 cards.
    // 7 cards: As Ks Qs Js Ts 2h 3h
    // Best: Royal Flush

    let (cards, _) = StdDeck::string_to_mask("AsKsQsJsTs2h3h").unwrap();
    let val = Eval::eval_n(&cards, 7);

    assert_eq!(val.hand_type(), HandType::StFlush as u8);
}

#[test]
fn test_variant_stud8() {
    // Stud8 is Hi/Lo 7 cards.
    // We don't have a direct "Stud8Evaluator" exposed in `evaluators::mod.rs`
    // efficiently, but we can verify if `std_deck_lowball8_eval` works for 7 cards
    // or if we need to call separate hi and lo evals.

    // We can combine Eval::eval_n (Hi) and std_deck_lowball8_eval (Lo).
    use poker_eval_rs::evaluators::std_deck_lowball8_eval;

    let (cards, count) = StdDeck::string_to_mask("As2s3s4s5s8h9h").unwrap(); // A-2-3-4-5 StFlush + Low

    let hi_val = Eval::eval_n(&cards, count);
    assert_eq!(hi_val.hand_type(), HandType::StFlush as u8);

    let lo_val = std_deck_lowball8_eval(&cards, count);
    assert_ne!(lo_val.unwrap().value, LOW_HAND_VAL_NOTHING);
}

#[test]
fn test_variant_razz() {
    // Razz is 7-card Stud Low (A-5).
    // Best hand: A-2-3-4-5.
    use poker_eval_rs::evaluators::std_deck_lowball_eval;

    let (cards, count) = StdDeck::string_to_mask("As2h3d4c5sKsQs").unwrap();
    let val = std_deck_lowball_eval(&cards, count);

    assert_ne!(val.value, LOW_HAND_VAL_NOTHING);

    // Compare against a worse hand
    let (cards2, count2) = StdDeck::string_to_mask("KsQsJsTs9s2d3c").unwrap(); // K-high
    let val2 = std_deck_lowball_eval(&cards2, count2);

    assert!(
        val < val2,
        "Wheel should beat K-high in Razz (lower is better)"
    );
}

#[test]
fn test_variant_lowball27() {
    // 2-7 Lowball (Single Draw or Triple Draw).
    // Best hand: 2-3-4-5-7 (no straight, no flush).
    // Straights and Flushes count AGAINST you.
    use poker_eval_rs::evaluators::std_deck_lowball27_eval;

    // 2-3-4-5-7 (Rainbow)
    let (wheel, count) = StdDeck::string_to_mask("2s3h4d5c7s").unwrap();
    let val_wheel = std_deck_lowball27_eval(&wheel, count);

    // 2-3-4-5-6 (Straight - Bad in 2-7)
    let (straight, count2) = StdDeck::string_to_mask("2s3h4d5c6s").unwrap();
    let val_straight = std_deck_lowball27_eval(&straight, count2);

    assert!(
        val_wheel < val_straight,
        "2-3-4-5-7 should beat 2-3-4-5-6 Straight (lower is better)"
    );
}
