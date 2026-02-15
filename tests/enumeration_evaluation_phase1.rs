use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::enumdefs::{EnumResult, Game, SampleType, ENUM_MAXPLAYERS};
use poker_eval_rs::enumerate::{enum_exhaustive, enum_qmc, enum_sample};
use poker_eval_rs::errors::PokerError;

fn holdem_heads_up_pockets() -> Vec<poker_eval_rs::deck::StdDeckCardMask> {
    let (p1, _) = StdDeck::string_to_mask("As Ks").unwrap();
    let (p2, _) = StdDeck::string_to_mask("Qh Qd").unwrap();
    vec![p1, p2]
}

#[test]
fn enum_sample_rejects_too_many_players() {
    let pockets = vec![StdDeck::string_to_mask("As Ks").unwrap().0; ENUM_MAXPLAYERS + 1];
    let mut result = EnumResult::new(Game::Holdem);

    let err = enum_sample(
        Game::Holdem,
        &pockets,
        StdDeck::string_to_mask("").unwrap().0,
        StdDeck::string_to_mask("").unwrap().0,
        pockets.len(),
        0,
        10,
        false,
        &mut result,
    )
    .unwrap_err();

    assert_eq!(err, PokerError::TooManyPlayers);
}

#[test]
fn enum_sample_orderflag_controls_ordering_allocation() {
    let pockets = holdem_heads_up_pockets();
    let board = StdDeck::string_to_mask("").unwrap().0;
    let dead = StdDeck::string_to_mask("").unwrap().0;

    let mut with_ordering = EnumResult::new(Game::Holdem);
    enum_sample(
        Game::Holdem,
        &pockets,
        board,
        dead,
        2,
        0,
        64,
        true,
        &mut with_ordering,
    )
    .unwrap();
    assert_eq!(with_ordering.nsamples, 64);
    assert!(with_ordering.ordering.is_some());

    let mut without_ordering = EnumResult::new(Game::Holdem);
    enum_sample(
        Game::Holdem,
        &pockets,
        board,
        dead,
        2,
        0,
        64,
        false,
        &mut without_ordering,
    )
    .unwrap();
    assert_eq!(without_ordering.nsamples, 64);
    assert!(without_ordering.ordering.is_none());
}

#[test]
fn enum_qmc_sets_sample_type_and_samples() {
    let pockets = holdem_heads_up_pockets();
    let board = StdDeck::string_to_mask("").unwrap().0;
    let dead = StdDeck::string_to_mask("").unwrap().0;
    let mut result = EnumResult::new(Game::Holdem);

    enum_qmc(
        Game::Holdem,
        &pockets,
        board,
        dead,
        2,
        0,
        128,
        true,
        &mut result,
    )
    .unwrap();

    assert_eq!(result.sample_type, SampleType::QuasiMonteCarlo);
    assert_eq!(result.nsamples, 128);
    assert!(result.ordering.is_some());
}

#[test]
fn enum_qmc_rejects_unsupported_game() {
    let pockets = holdem_heads_up_pockets();
    let board = StdDeck::string_to_mask("").unwrap().0;
    let dead = StdDeck::string_to_mask("").unwrap().0;
    let mut result = EnumResult::new(Game::Holdem);

    let err = enum_qmc(
        Game::Omaha,
        &pockets,
        board,
        dead,
        2,
        0,
        32,
        false,
        &mut result,
    )
    .unwrap_err();

    assert_eq!(err, PokerError::UnsupportedGameType);
    assert_eq!(result.sample_type, SampleType::QuasiMonteCarlo);
    assert_eq!(result.nsamples, 0);
}

#[test]
fn enum_exhaustive_orderflag_allocates_ordering() {
    let pockets = holdem_heads_up_pockets();
    let (board, nboard) = StdDeck::string_to_mask("2c 7d 9h").unwrap();
    let dead = StdDeck::string_to_mask("").unwrap().0;
    let mut result = EnumResult::new(Game::Holdem);

    enum_exhaustive(
        Game::Holdem,
        &pockets,
        board,
        dead,
        2,
        nboard,
        true,
        &mut result,
    )
    .unwrap();

    assert_eq!(result.nsamples, 990);
    assert!(result.ordering.is_some());
}

#[test]
fn enum_exhaustive_rejects_unsupported_game_type() {
    let pockets = holdem_heads_up_pockets();
    let board = StdDeck::string_to_mask("").unwrap().0;
    let dead = StdDeck::string_to_mask("").unwrap().0;
    let mut result = EnumResult::new(Game::Holdem);

    let err = enum_exhaustive(
        Game::Lowball,
        &pockets,
        board,
        dead,
        2,
        0,
        true,
        &mut result,
    )
    .unwrap_err();

    assert_eq!(err, PokerError::UnsupportedGameType);
    assert!(result.ordering.is_some());
}

#[test]
fn exhaustive_sample_and_qmc_match_on_complete_board() {
    let (p1, _) = StdDeck::string_to_mask("As Ah").unwrap();
    let (p2, _) = StdDeck::string_to_mask("Kd Kh").unwrap();
    let pockets = vec![p1, p2];
    let (board, _) = StdDeck::string_to_mask("Ac Ad 2c 3d 4h").unwrap();
    let dead = StdDeck::string_to_mask("").unwrap().0;

    let mut ex = EnumResult::new(Game::Holdem);
    enum_exhaustive(Game::Holdem, &pockets, board, dead, 2, 5, false, &mut ex).unwrap();
    assert_eq!(ex.nsamples, 1);
    let ex_ev0 = ex.ev[0] / ex.nsamples as f64;
    let ex_ev1 = ex.ev[1] / ex.nsamples as f64;
    assert!(ex_ev0 > ex_ev1);

    let mut mc = EnumResult::new(Game::Holdem);
    enum_sample(
        Game::Holdem,
        &pockets,
        board,
        dead,
        2,
        5,
        256,
        false,
        &mut mc,
    )
    .unwrap();
    assert_eq!(mc.nsamples, 256);
    let mc_ev0 = mc.ev[0] / mc.nsamples as f64;
    let mc_ev1 = mc.ev[1] / mc.nsamples as f64;
    assert!((mc_ev0 - ex_ev0).abs() < 1e-12);
    assert!((mc_ev1 - ex_ev1).abs() < 1e-12);

    let mut qmc = EnumResult::new(Game::Holdem);
    enum_qmc(
        Game::Holdem,
        &pockets,
        board,
        dead,
        2,
        5,
        256,
        false,
        &mut qmc,
    )
    .unwrap();
    assert_eq!(qmc.nsamples, 256);
    let qmc_ev0 = qmc.ev[0] / qmc.nsamples as f64;
    let qmc_ev1 = qmc.ev[1] / qmc.nsamples as f64;
    assert!((qmc_ev0 - ex_ev0).abs() < 1e-12);
    assert!((qmc_ev1 - ex_ev1).abs() < 1e-12);
}

#[test]
fn enum_sample_holdem8_allocates_hilo_ordering() {
    let pockets = holdem_heads_up_pockets();
    let board = StdDeck::string_to_mask("").unwrap().0;
    let dead = StdDeck::string_to_mask("").unwrap().0;
    let mut result = EnumResult::new(Game::Holdem8);

    enum_sample(
        Game::Holdem8,
        &pockets,
        board,
        dead,
        2,
        0,
        64,
        true,
        &mut result,
    )
    .unwrap();

    assert_eq!(result.nsamples, 64);
    let ordering = result
        .ordering
        .as_ref()
        .expect("ordering should be allocated");
    assert_eq!(ordering.nentries, 256);
}

#[test]
fn enum_metadata_is_preserved_after_execution() {
    let pockets = holdem_heads_up_pockets();
    let empty = StdDeck::string_to_mask("").unwrap().0;
    let board5 = StdDeck::string_to_mask("2c 7d 9h Jc Qd").unwrap().0;

    let mut sample_res = EnumResult::new(Game::Holdem);
    let (omaha_p1, _) = StdDeck::string_to_mask("As Ks Qh Jh").unwrap();
    let (omaha_p2, _) = StdDeck::string_to_mask("Ad Kd Qc Jc").unwrap();
    enum_sample(
        Game::Omaha,
        &[omaha_p1, omaha_p2],
        empty,
        empty,
        2,
        0,
        16,
        false,
        &mut sample_res,
    )
    .unwrap();
    assert_eq!(sample_res.game, Game::Omaha);
    assert_eq!(sample_res.sample_type, SampleType::Sample);
    assert_eq!(sample_res.nplayers, 2);

    let mut qmc_res = EnumResult::new(Game::Holdem);
    enum_qmc(
        Game::Holdem,
        &pockets,
        empty,
        empty,
        2,
        0,
        16,
        false,
        &mut qmc_res,
    )
    .unwrap();
    assert_eq!(qmc_res.game, Game::Holdem);
    assert_eq!(qmc_res.sample_type, SampleType::QuasiMonteCarlo);
    assert_eq!(qmc_res.nplayers, 2);

    let mut ex_res = EnumResult::new(Game::Holdem);
    enum_exhaustive(
        Game::ShortDeck,
        &pockets,
        board5,
        empty,
        2,
        5,
        false,
        &mut ex_res,
    )
    .unwrap();
    assert_eq!(ex_res.game, Game::ShortDeck);
    assert_eq!(ex_res.sample_type, SampleType::Exhaustive);
    assert_eq!(ex_res.nplayers, 2);
}
