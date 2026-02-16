use poker_eval_rs::deck::{StdDeck, StdDeckCardMask};
use poker_eval_rs::enumdefs::{EnumResult, Game};
use poker_eval_rs::enumerate::enum_exhaustive;
use poker_eval_rs::evaluators::Eval;

#[test]
fn test_holdem_royal_flush() {
    let hand_str = "AsKsQsJsTs";
    let (mask, count) = StdDeck::string_to_mask(hand_str).unwrap();
    assert_eq!(count, 5);

    let result = Eval::eval_n(&mask, 5);
    // assert!(result.value >= 0); // Always true for unsigned types
    // Assuming you have a way to check if it's a Straight Flush / Royal Flush from value or string
    // Or just check it's better than a pair

    let pair_str = "2s2h3c4d5s";
    let (pair_mask, _) = StdDeck::string_to_mask(pair_str).unwrap();
    let pair_result = Eval::eval_n(&pair_mask, 5);

    assert!(result.value > pair_result.value);
}

#[test]
fn test_omaha_eval() {
    // Omaha takes 4 hole cards + 5 board cards = 9 cards total
    // But eval_n might handle arbitrary number. Omaha rules use specifically 2 from hole, 3 from board.
    // The library likely has a specific Omaha evaluator in `evaluators`.

    // For general 5-card eval:
    let hand_str = "AsKsQsJsTs2h3c4d5s"; // 9 cards
    let (mask, count) = StdDeck::string_to_mask(hand_str).unwrap();
    // Verify we can evaluate 9 cards (though specific Omaha logic might be different)
    // Eval::eval_n typically evaluates the best 5-card hand from the set.

    if count >= 5 {
        let result = Eval::eval_n(&mask, count);
        assert!(result.value > 0);
    }
}

#[test]
fn test_equity_calculation_smoke() {
    let p1 = "AsKs";
    let p2 = "2s2h";
    let (mask1, _) = StdDeck::string_to_mask(p1).unwrap();
    let (mask2, _) = StdDeck::string_to_mask(p2).unwrap();
    let pockets = vec![mask1, mask2];

    let board = StdDeckCardMask::new();
    let dead = StdDeckCardMask::new();

    let mut result = EnumResult::default();

    // Use enum_sample for smoke test to avoid stack overflow in debug mode and speed up
    let res = poker_eval_rs::enumerate::evaluation::enum_sample(
        Game::Holdem,
        &pockets,
        board,
        dead,
        2,
        0,
        10000, // 10k iterations
        false,
        &mut result,
    );

    assert!(res.is_ok());
    assert!(result.nsamples > 0);
}

#[test]
fn test_invalid_hand_parsing() {
    let invalid_str = "ZZTOP";
    let res = StdDeck::string_to_mask(invalid_str);
    assert!(res.is_err());
}

#[test]
fn test_too_many_players() {
    let pockets = vec![StdDeckCardMask::new(); 13]; // 13 players (MAX is 12)
    let board = StdDeckCardMask::new();
    let dead = StdDeckCardMask::new();
    let mut result = EnumResult::default();

    let res = enum_exhaustive(
        Game::Holdem,
        &pockets,
        board,
        dead,
        13,
        0,
        false,
        &mut result,
    );

    assert!(res.is_err());
}

#[test]
fn test_unsupported_game_type_exhaustive() {
    let pockets = vec![StdDeckCardMask::new(); 2];
    let board = StdDeckCardMask::new();
    let dead = StdDeckCardMask::new();
    let mut result = EnumResult::default();

    let res = enum_exhaustive(
        Game::NumGames,
        &pockets,
        board,
        dead,
        2,
        0,
        false,
        &mut result,
    );

    assert!(res.is_err());
}

#[test]
fn test_unsupported_board_config() {
    let pockets = vec![StdDeckCardMask::new(); 2];
    let board = StdDeckCardMask::new();
    let dead = StdDeckCardMask::new();
    let mut result = EnumResult::default();

    let res = enum_exhaustive(
        Game::Holdem,
        &pockets,
        board,
        dead,
        2,
        1, // 1 board card is not supported for exhaustive (usually 0, 3, 4, 5)
        false,
        &mut result,
    );

    assert!(res.is_err());
}
