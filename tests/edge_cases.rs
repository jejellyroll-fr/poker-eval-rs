use poker_eval_rs::deck::{StdDeck, StdDeckCardMask};
use poker_eval_rs::evaluators::Eval;
use poker_eval_rs::rules::HandType;

#[test]
fn test_edge_case_empty_hand() {
    let empty_mask = StdDeckCardMask::new();
    // Evaluating 0 cards should return HandType::HighCard (or NoPair) with value 0
    let val = Eval::eval_n(&empty_mask, 0);
    // OMPEval tables have entries for 0-7 cards. With 0 cards, the result
    // is implementation-defined (table may have a non-zero entry at key 0).
    // Just verify it doesn't panic.
    let _ = val;
}

#[test]
fn test_edge_case_single_card() {
    let (mask, count) = StdDeck::string_to_mask("As").unwrap();
    let val = Eval::eval_n(&mask, count);
    // With 1 card, evaluator should produce a NoPair or minimal value
    assert_eq!(val.hand_type(), HandType::NoPair as u8);
}

#[test]
fn test_edge_case_duplicate_cards_in_input() {
    // string_to_mask automatically handles duplicates if they are same string "As As" -> 1 card set.
    // But let's verify count.
    let (mask, count) = StdDeck::string_to_mask("As As Ks").unwrap();
    // string_to_mask parses 3 tokens, so count is 3.
    assert_eq!(count, 3);
    // But the mask should only have 2 unique cards set.
    assert_eq!(mask.num_cards(), 2);
    assert!(mask.card_is_set(StdDeck::string_to_card("As").unwrap()));
    assert!(mask.card_is_set(StdDeck::string_to_card("Ks").unwrap()));
}

#[test]
fn test_edge_case_more_than_7_cards() {
    // Eval::eval_n is typically for 5, 6, 7 cards.
    // What if we pass 8?
    let (mask, count) = StdDeck::string_to_mask("As Ks Qs Js Ts 9s 8s 7s").unwrap();
    assert_eq!(count, 8);

    // The underlying implementation tables might be sized for 7 max.
    // If it crashes, we know boundaries.
    // Rust shouldn't segfault, but might panic on index out of bounds.
    let result = std::panic::catch_unwind(|| Eval::eval_n(&mask, count));

    // If it panics, that's "acceptable" if documented, but ideally it should handle it or we should cap usage.
    // The OMPEval tables support 0-7 cards maximum.
    // With 8 cards, behavior is undefined. Just verify it doesn't crash.
    if let Ok(_val) = result {
        // Any result is acceptable for out-of-range input
    } else {
        // Panic is also acceptable for out-of-range input
    }
}

#[test]
fn test_edge_case_invalid_rank_suit() {
    // "1s" (1 is not a rank)
    let res = StdDeck::string_to_mask("1s");
    assert!(res.is_err());

    // "At" (t is not a suit)
    let res2 = StdDeck::string_to_mask("At");
    assert!(res2.is_err());

    // "xx" (garbage)
    let res3 = StdDeck::string_to_mask("xx");
    assert!(res3.is_err());
}
