use poker_eval_rs::deck::{JokerDeck, StdDeck};
use poker_eval_rs::evaluators::{
    joker_lowball8_eval, joker_lowball_eval, std_deck_lowball8_eval, std_deck_lowball_eval,
    std_deck_omaha_hi_eval, std_deck_omaha_hi_low8_eval, Eval, EvalJoker,
};
use poker_eval_rs::handval_low::LOW_HAND_VAL_NOTHING;
use poker_eval_rs::rules::HandType;

// Tests for missing variants from verification report

#[test]
fn test_variant_holdem8() {
    // Holdem Hi/Lo 8-or-better
    // Hole: As 2s. Board: 3s 4s 5s Qs Ks
    // Hi: StFlush (A-2-3-4-5)
    // Lo: A-2-3-4-5 (Wheel)

    let (hole, _) = StdDeck::string_to_mask("As2s").unwrap();
    let (board, _) = StdDeck::string_to_mask("3s4s5sQsKs").unwrap();

    let mut hand = hole;
    hand.or(&board);
    let count = hand.num_cards(); // 7

    // Hi
    let hival = Eval::eval_n(&hand, count);
    assert_eq!(hival.hand_type(), HandType::StFlush as u8);

    // Lo
    let loval = std_deck_lowball8_eval(&hand, count).unwrap();
    assert_ne!(loval.value, LOW_HAND_VAL_NOTHING);

    // Non-qualifying Lo
    // Hole: As Ks. Board: Qs Js Ts 9s 8s.
    // Low cards: 8. Not enough for 8-low.
    let (hole2, _) = StdDeck::string_to_mask("AsKs").unwrap();
    let (board2, _) = StdDeck::string_to_mask("QsJsTs9s8s").unwrap();
    let mut hand2 = hole2;
    hand2.or(&board2);
    let count2 = hand2.num_cards();

    let loval2 = std_deck_lowball8_eval(&hand2, count2).unwrap();
    assert_eq!(loval2.value, LOW_HAND_VAL_NOTHING);
}

#[test]
fn test_variant_omaha5() {
    // 5-Card Omaha Hi
    // Hole: As Ac Ks Kc 2s (5 cards). Board: Ah Ad 3s 4s 5s.
    // Must use exactly 2 hole, 3 board.
    // Best combo: hole As+2s + board 3s+4s+5s = A-2-3-4-5 Straight Flush (spades).

    let (hole, _) = StdDeck::string_to_mask("AsAcKsKc2s").unwrap();
    let (board, _) = StdDeck::string_to_mask("AhAd3s4s5s").unwrap();

    let mut hival = None;
    std_deck_omaha_hi_eval(hole, board, &mut hival).unwrap();

    assert!(hival.is_some());
    assert_eq!(hival.unwrap().hand_type(), HandType::StFlush as u8);
}

#[test]
fn test_variant_omaha6() {
    // 6-Card Omaha Hi
    // Hole: As Ac Ks Kc 2s 2d (6 cards).
    // Board: Ah Ad 3s 4s 5s.
    // Best combo: hole As+2s + board 3s+4s+5s = A-2-3-4-5 Straight Flush (spades).

    let (hole, _) = StdDeck::string_to_mask("AsAcKsKc2s2d").unwrap();
    let (board, _) = StdDeck::string_to_mask("AhAd3s4s5s").unwrap();

    let mut hival = None;
    std_deck_omaha_hi_eval(hole, board, &mut hival).unwrap();

    assert!(hival.is_some());
    assert_eq!(hival.unwrap().hand_type(), HandType::StFlush as u8);
}

#[test]
fn test_variant_omaha85() {
    // 5-Card Omaha Hi/Lo
    // Hole: As 2s 3d 4d Ks. (5 cards)
    // Board: 5s 6s 7s 8s 9s.
    // Lo: A-2 (hole) + 3-4-5 (board) -> NO, must use board 3 cards.
    // Board low cards: 5, 6, 7, 8.
    // Hole low cards: A, 2, 3, 4.
    // Best Lo: A-2 (hole) + 5-6-7 (board) -> 7-6-5-2-A (7-low).

    let (hole, _) = StdDeck::string_to_mask("As2s3d4dKs").unwrap();
    let (board, _) = StdDeck::string_to_mask("5s6s7s8s9s").unwrap();

    let mut hival = None;
    let mut loval = None;
    std_deck_omaha_hi_low8_eval(hole, board, &mut hival, &mut loval).unwrap();

    assert!(hival.is_some());
    assert!(loval.is_some());
    assert_ne!(loval.unwrap().value, LOW_HAND_VAL_NOTHING);
}

#[test]
fn test_variant_stud78() {
    // Stud Hi/Lo 8-or-better
    // 7 cards total, mixed suits to avoid flush.
    // Cards: As 2h 3d 4c 5s Kh Qd
    // Hi: A-2-3-4-5 Straight (wheel).
    // Lo: A-2-3-4-5 (Wheel, qualifies for 8-or-better).

    let (cards, _) = StdDeck::string_to_mask("As2h3d4c5sKhQd").unwrap();
    let count = cards.num_cards();

    // Hi
    let hival = Eval::eval_n(&cards, count);
    assert_eq!(hival.hand_type(), HandType::Straight as u8);

    // Lo
    let loval = std_deck_lowball8_eval(&cards, count).unwrap();
    assert_ne!(loval.value, LOW_HAND_VAL_NOTHING);
}

#[test]
fn test_variant_stud7nsq() {
    // Stud Hi/Lo No Qualifier
    // 7 cards mixed suits to avoid flush.
    // Hi: Straight (9-T-J-Q-K)
    // Lo: 7-8-9-T-J (Jack low). (Best low5 from 7).
    // In NSQ, even high lows qualify.

    let (cards, _) = StdDeck::string_to_mask("ThJdQsKc9h8d7s").unwrap();
    let count = cards.num_cards();

    // Hi
    let hival = Eval::eval_n(&cards, count);
    assert_eq!(hival.hand_type(), HandType::Straight as u8);

    // Lo
    let loval = std_deck_lowball_eval(&cards, count);
    assert_ne!(loval.value, LOW_HAND_VAL_NOTHING);
    // Usually std_deck_lowball_eval returns a value for any hand >= 5 cards
}

#[test]
fn test_variant_draw5() {
    // 5-Card Draw Hi with Joker
    // Joker should improve hand.
    // A-A-2-3-Joker -> A-A-A-2-3? No, pair -> trips.
    // Hand: As Ah 2d 3c Xx.
    // Should be Trips.

    let (cards, _) = JokerDeck::string_to_mask("AsAh2d3cXx").unwrap();
    let val = EvalJoker::eval_n(cards, 5);

    assert_eq!(val.hand_type(), HandType::Trips as u8);
}

#[test]
fn test_variant_draw58() {
    // 5-Card Draw Hi/Lo 8-or-better with Joker
    // Hand: A-2-3-4-Joker (mixed suits to avoid flush).
    // Hi: A-2-3-4-5 Straight (Joker acts as 5).
    // Lo: A-2-3-4-5 Wheel.

    let (cards, _) = JokerDeck::string_to_mask("As2h3d4cXx").unwrap();

    // Hi
    let hival = EvalJoker::eval_n(cards, 5);
    assert_eq!(hival.hand_type(), HandType::Straight as u8);

    // Lo
    let loval = joker_lowball8_eval(&cards, 5);
    assert_ne!(loval.value, LOW_HAND_VAL_NOTHING);
}

#[test]
fn test_variant_draw5nsq() {
    // 5-Card Draw Hi/Lo No Qualifier with Joker
    // Hand: T-J-Q-K-Joker (mixed suits to avoid flush).
    // Hi: Straight (Joker acts as 9 or A).
    // Lo: T-J-Q-K-A? (Joker=A).

    let (cards, _) = JokerDeck::string_to_mask("ThJdQsKcXx").unwrap();

    // Hi
    let hival = EvalJoker::eval_n(cards, 5);
    assert_eq!(hival.hand_type(), HandType::Straight as u8);

    // Lo
    let loval = joker_lowball_eval(&cards, 5);
    assert_ne!(loval.value, LOW_HAND_VAL_NOTHING);
}
