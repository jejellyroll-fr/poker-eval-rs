use poker_eval_rs::evaluators::omaha::std_deck_omaha_hi_low8_eval;
use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::handval_low::LOW_HAND_VAL_NOTHING;

#[test]
fn test_omaha_hi_low_bug_reproduction() {
    // Hole: A, 2, 3, K (3 low cards)
    // Board: 4, 5, T, J, Q (2 low cards: 4, 5)
    //
    // Total low cards available = {A, 2, 3, 4, 5} -> A valid A-5 low IF we could use 3 hole cards.
    // However, Omaha requires EXACTLY 2 hole cards and 3 board cards.
    //
    // Case 1: Use {A, 2} from hole -> Need 3 low cards from board. Board only has {4, 5}. Fails.
    // Case 2: Use {A, 3} from hole -> Need 3 low cards from board. Board only has {4, 5}. Fails.
    // Case 3: Use {2, 3} from hole -> Need 3 low cards from board. Board only has {4, 5}. Fails.
    //
    // Therefore, NO qualifying low hand is possible.
    // The buggy implementation likely returns a valid low because it sees A,2,3,4,5 in the union set.

    let hole_str = "As2s3sKs";
    let board_str = "4d5dTdJdQd";

    let (hole, _) = StdDeck::string_to_mask(hole_str).expect("parse hole");
    let (board, _) = StdDeck::string_to_mask(board_str).expect("parse board");

    let mut hival = None;
    let mut loval = None;

    std_deck_omaha_hi_low8_eval(hole, board, &mut hival, &mut loval).expect("eval");

    // We expect loval to be None or LOW_HAND_VAL_NOTHING
    let low_is_valid = if let Some(val) = loval {
        val.value != LOW_HAND_VAL_NOTHING
    } else {
        false
    };

    assert!(!low_is_valid, "Bug reproduced: Found a valid low hand where none should exist (likely used >2 hole cards)!");
}
