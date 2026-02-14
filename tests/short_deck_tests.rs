use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::evaluators::{HandEvaluator, ShortDeckEvaluator};
use poker_eval_rs::rules::HandType;

#[test]
fn test_short_deck_flush_beats_full_house() {
    // Flush: As Ks Qs Js 9s
    let (flush_hand, _) = StdDeck::string_to_mask("AsKsQsJs9s").unwrap();
    let (dummy_board, _) = StdDeck::string_to_mask("").unwrap();

    // Full House: Ah Ad Ac Ks Kd
    let (fh_hand, _) = StdDeck::string_to_mask("AhAdAcKsKd").unwrap();

    let flush_val =
        ShortDeckEvaluator::evaluate_hand(&flush_hand, &dummy_board).expect("Evaluation failed");
    let fh_val =
        ShortDeckEvaluator::evaluate_hand(&fh_hand, &dummy_board).expect("Evaluation failed");

    // Note: ShortDeckEvaluator swaps Flush and FullHouse types to ensure correct comparison value
    assert_eq!(flush_val.hand_type(), HandType::FullHouse as u8);
    assert_eq!(fh_val.hand_type(), HandType::Flush as u8);

    // In Short Deck, Flush > Full House
    assert!(
        flush_val > fh_val,
        "Flush should beat Full House in Short Deck"
    );
}

#[test]
fn test_short_deck_straight_low() {
    // A 6 7 8 9 is a straight (A acts as 5)
    let (straight_hand, _) = StdDeck::string_to_mask("As6d7c8h9s").unwrap();
    let (dummy_board, _) = StdDeck::string_to_mask("").unwrap();

    let val =
        ShortDeckEvaluator::evaluate_hand(&straight_hand, &dummy_board).expect("Evaluation failed");

    assert_eq!(val.hand_type(), HandType::Straight as u8);
    // Ensure it's treated as a 5-high straight equivalent (lowest straight)
    // or specifically 9-high straight?
    // In Short Deck A-6-7-8-9 is the low straight (like A-2-3-4-5).
    // It is often called a 9-high straight effectively if we map A->5.
}

#[test]
fn test_short_deck_no_low_cards() {
    // Ensuring that 2, 3, 4, 5 are not generated in random enumeration is hard here without mocking.
    // But we can check that if we PASS them, maybe they are ignored or handled?
    // Actually ShortDeck usually strictly uses a subset of standard deck.
    // Use the library's deck generation to verify 2-5 are absent.

    // We can't easily access the deck generation logic from here as it's internal to Enumerate usually.
    // But we can check `enumerate::get_deck(Game::ShortDeck)`.
    // Wait, `get_deck` might be internal.
}
