use poker_eval_rs::deck::StdDeckCardMask;
use poker_eval_rs::enumdefs::{EnumResult, Game};
use poker_eval_rs::enumerate::{enum_exhaustive, enum_sample};

#[test]
fn test_short_deck_equity_pair_vs_overcards() {
    let pocket1 = StdDeckCardMask::from_card_index(34) | StdDeckCardMask::from_card_index(35); // Th Ts
    let pocket2 = StdDeckCardMask::from_card_index(51) | StdDeckCardMask::from_card_index(47); // As Ks
    let pockets = vec![pocket1, pocket2];
    let board = StdDeckCardMask::new();
    let dead = StdDeckCardMask::new();

    let mut result = EnumResult::default();
    // enum_sample(game, pockets, board, dead, npockets, nboard, niter, orderflag, result)
    enum_sample(
        Game::ShortDeck,
        &pockets,
        board,
        dead,
        pockets.len(),
        0,
        100_000,
        false,
        &mut result,
    )
    .expect("Enumeration failed");

    println!("Short Deck ThTs vs AsKs (100k samples):");
    println!("P1 (ThTs) Win%: {:.2}", result.ev[0] * 100.0);
    println!("P2 (AsKs) Win%: {:.2}", result.ev[1] * 100.0);

    assert!(result.ev[0] > 0.40);
    assert!(result.ev[1] > 0.40);
}

#[test]
fn test_short_deck_straight_flush_dominance() {
    let pocket1 = StdDeckCardMask::from_card_index(30) | StdDeckCardMask::from_card_index(34); // 9h Th
    let pocket2 = StdDeckCardMask::from_card_index(50) | StdDeckCardMask::from_card_index(51); // Ah As

    let board = StdDeckCardMask::from_card_index(18) // 6h
              | StdDeckCardMask::from_card_index(22) // 7h
              | StdDeckCardMask::from_card_index(26); // 8h

    let pockets = vec![pocket1, pocket2];
    let dead = StdDeckCardMask::new();

    let mut result = EnumResult::default();
    // enum_exhaustive(game, pockets, board, dead, npockets, nboard, orderflag, result)
    enum_exhaustive(
        Game::ShortDeck,
        &pockets,
        board,
        dead,
        pockets.len(),
        3,
        false,
        &mut result,
    )
    .expect("Enumeration failed");

    println!("P1 (SF draw) vs P2 (AA) on 6h7h8h:");
    println!("P1 Win%: {:.2}", result.ev[0] * 100.0);
    println!("P2 Win%: {:.2}", result.ev[1] * 100.0);

    assert!(result.ev[0] > 0.999);
}
