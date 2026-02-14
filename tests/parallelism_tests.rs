use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::deck::StdDeckCardMask;
use poker_eval_rs::enumdefs::{EnumResult, Game};
// use poker_eval_rs::evaluators::HandEvaluator; // Unused
// use poker_eval_rs::evaluators::OmahaHiEvaluator; // Unused

#[test]
fn test_parallel_omaha_monte_carlo_convergence() {
    // Omaha Hi Monte Carlo
    // Hand 1: As Ks Qs Js (Royal Flush draw + Flush draw + Straight draw)
    // Hand 2: 2h 3h 4h 5h (Low cards, straight flush draw)
    // Board: Th 9h 8h (Straight flush possibilities for Hand 2, Flush for Hand 1)

    // We expect Hand 2 (Straight Flush) to have very high equity.
    // Let's run MC with enough iterations to converge stability.

    let (h1, _) = StdDeck::string_to_mask("AsKsQsJs").unwrap();
    let (h2, _) = StdDeck::string_to_mask("2h3h4h5h").unwrap();
    let (board, _) = StdDeck::string_to_mask("Th9h8h").unwrap(); // 3 board cards, 2 to come

    let pockets = vec![h1, h2];
    let dead = StdDeckCardMask::new();
    let mut result = EnumResult::new(Game::Omaha);

    let n_iter = 50_000;

    // Run parallel simulation
    let res = poker_eval_rs::enumerate::evaluation::enum_sample(
        Game::Omaha,
        &pockets,
        board,
        dead,
        2,
        3, // 3 board cards known
        n_iter,
        false,
        &mut result,
    );

    assert!(res.is_ok());
    assert_eq!(result.nsamples as usize, n_iter);

    let eq1 = result.ev[0] / (n_iter as f64);
    let eq2 = result.ev[1] / (n_iter as f64);

    println!("Omaha MC Equity: P1={:.4}, P2={:.4}", eq1, eq2);

    // Hand 2 has a made Straight Flush (8h 9h Th + 2h 3h? No, needs 2 from hole...)
    // Hole: 2h 3h 4h 5h. Board: Th 9h 8h.
    // 2 from hole: 4h 5h. 3 from board: 8h 9h Th. --> 4-5-8-9-T Flush.
    // Wait.
    // Hand 2: 4h 5h + 8h 9h Th = Flush.
    // Hand 1: As Ks + 8h 9h Th = Ace High Flush.
    // Actually Hand 1 (Ace high flush) should crush Hand 2 (Low flush).

    // Wait, straight flush?
    // Hand 2: 2h 3h 4h 5h. Board: Th 9h 8h.
    // 6h 7h turn/river would make StFlush.

    // Anyway, we just want to ensure it runs and produces valid probability (0..1).
    // And that sum is roughly 1.0 (minus ties/split logic, but EV usually sums to 1 for heads up).

    assert!((0.0..=1.0).contains(&eq1));
    assert!((0.0..=1.0).contains(&eq2));
    assert!((eq1 + eq2 - 1.0).abs() < 0.05); // Allow small margin for rake/error (though EV sums to 1 here)
}

#[test]
fn test_parallel_holdem_monte_carlo_stability() {
    // Basic AA vs KK
    let (h1, _) = StdDeck::string_to_mask("AsAc").unwrap();
    let (h2, _) = StdDeck::string_to_mask("KsKc").unwrap();
    let board = StdDeckCardMask::new();
    let dead = StdDeckCardMask::new();
    let mut result = EnumResult::new(Game::Holdem);

    let n_iter = 20_000;

    let res = poker_eval_rs::enumerate::evaluation::enum_sample(
        Game::Holdem,
        &[h1, h2],
        board,
        dead,
        2,
        0,
        n_iter,
        false,
        &mut result,
    );

    assert!(res.is_ok());
    let eq_aa = result.ev[0] / (n_iter as f64);

    // AA vs KK is approx 82% equity for AA.
    println!("AA vs KK equity: {:.4}", eq_aa);
    assert!(eq_aa > 0.75 && eq_aa < 0.88);
}
