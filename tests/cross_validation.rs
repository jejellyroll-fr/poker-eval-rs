use poker_eval_rs::deck::{StdDeck, StdDeckCardMask};
use poker_eval_rs::enumdefs::{EnumResult, Game};

#[test]
fn test_cross_validate_exhaustive_vs_monte_carlo_holdem() {
    // Scenario: Head-up Hold'em
    // Player 1: As Ks (Big Slick)
    // Player 2: 2h 2d (Pocket Deuces)
    // Board: 3s 4s 5s (Flop) - Strong potential for both (Straight draw for P2, Flush draw for P1, etc.)

    let (h1, _) = StdDeck::string_to_mask("AsKs").unwrap();
    let (h2, _) = StdDeck::string_to_mask("2h2d").unwrap();
    let (board, _) = StdDeck::string_to_mask("3s4s5s").unwrap();

    let pockets = vec![h1, h2];
    let dead = StdDeckCardMask::new();

    // 1. Run Exhaustive Evaluation (The Truth)
    let mut res_ex = EnumResult::new(Game::Holdem);
    let start_ex = std::time::Instant::now();
    poker_eval_rs::enumerate::evaluation::enum_exhaustive(
        Game::Holdem,
        &pockets,
        board,
        dead,
        2,
        3, // 3 board cards known, 2 to come
        false,
        &mut res_ex,
    )
    .expect("Exhaustive evaluation failed");
    println!("Exhaustive time: {:?}", start_ex.elapsed());

    let ev_ex_p1 = res_ex.ev[0] / res_ex.nsamples as f64;
    let ev_ex_p2 = res_ex.ev[1] / res_ex.nsamples as f64;

    println!(
        "Exhaustive EV: P1={:.6}, P2={:.6} (Samples: {})",
        ev_ex_p1, ev_ex_p2, res_ex.nsamples
    );

    // 2. Run Monte Carlo Simulation (The Estimate)
    let mut res_mc = EnumResult::new(Game::Holdem);
    let n_iter = 200_000; // Enough samples to converge within ~0.5%
    let start_mc = std::time::Instant::now();
    poker_eval_rs::enumerate::evaluation::enum_sample(
        Game::Holdem,
        &pockets,
        board,
        dead,
        2,
        3,
        n_iter,
        false,
        &mut res_mc,
    )
    .expect("Monte Carlo evaluation failed");
    println!("Monte Carlo time: {:?}", start_mc.elapsed());

    let ev_mc_p1 = res_mc.ev[0] / res_mc.nsamples as f64;
    let ev_mc_p2 = res_mc.ev[1] / res_mc.nsamples as f64;

    println!(
        "Monte Carlo EV: P1={:.6}, P2={:.6} (Samples: {})",
        ev_mc_p1, ev_mc_p2, res_mc.nsamples
    );

    // 3. Compare Results
    // We expect MC to be within a small diff of Exhaustive.
    // Standard Error ~= 1 / sqrt(N). For 200k, SE is small.
    // Let's accept 0.5% absolute difference.

    let diff_p1 = (ev_ex_p1 - ev_mc_p1).abs();
    let diff_p2 = (ev_ex_p2 - ev_mc_p2).abs();

    println!("Diff P1: {:.6}, Diff P2: {:.6}", diff_p1, diff_p2);

    assert!(diff_p1 < 0.005, "MC P1 EV should be close to Exhaustive EV");
    assert!(diff_p2 < 0.005, "MC P2 EV should be close to Exhaustive EV");
}

#[test]
fn test_cross_validate_exhaustive_vs_monte_carlo_omaha() {
    // Scenario: Head-up Omaha
    // P1: As Ks Ah Kh (Double Suited Aces)
    // P2: Js Jc Td 9d (Rundown, double suited)
    // Board: Qs 8s 2d (Flop)

    let (h1, _) = StdDeck::string_to_mask("AsKsAhKh").unwrap();
    let (h2, _) = StdDeck::string_to_mask("JsJcTd9d").unwrap();
    let (board, _) = StdDeck::string_to_mask("Qs8s2d").unwrap();

    let pockets = vec![h1, h2];
    let dead = StdDeckCardMask::new();

    // 1. Run Exhaustive (Can be slow for Omaha 2-cards-to-come? 45*44/2 = 990 boards. Fast.)
    let mut res_ex = EnumResult::new(Game::Omaha);
    poker_eval_rs::enumerate::evaluation::enum_exhaustive(
        Game::Omaha,
        &pockets,
        board,
        dead,
        2,
        3,
        false,
        &mut res_ex,
    )
    .expect("Exhaustive Omaha failed");

    let ev_ex_p1 = res_ex.ev[0] / res_ex.nsamples as f64;

    // 2. Run Monte Carlo
    let mut res_mc = EnumResult::new(Game::Omaha);
    let n_iter = 200_000;
    poker_eval_rs::enumerate::evaluation::enum_sample(
        Game::Omaha,
        &pockets,
        board,
        dead,
        2,
        3,
        n_iter,
        false,
        &mut res_mc,
    )
    .expect("MC Omaha failed");

    let ev_mc_p1 = res_mc.ev[0] / res_mc.nsamples as f64;

    println!("Omaha Exhaustive: {:.5}, MC: {:.5}", ev_ex_p1, ev_mc_p1);

    // Omaha variance is higher, so we allow slightly more deviation or need more samples
    assert!((ev_ex_p1 - ev_mc_p1).abs() < 0.02);
}
