use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::enumdefs::{EnumResult, Game};
use poker_eval_rs::enumerate::{enum_exhaustive, enum_qmc, enum_sample};
use poker_eval_rs::errors::PokerError;

#[derive(Clone)]
struct SampleCase {
    game: Game,
    pockets: Vec<poker_eval_rs::deck::StdDeckCardMask>,
    board: poker_eval_rs::deck::StdDeckCardMask,
    nboard: usize,
}

fn mask(cards: &str) -> poker_eval_rs::deck::StdDeckCardMask {
    StdDeck::string_to_mask(cards).unwrap().0
}

#[test]
fn enum_sample_supported_games_smoke() {
    let cases = vec![
        SampleCase {
            game: Game::Holdem,
            pockets: vec![mask("As Ks"), mask("Qh Qd")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Holdem8,
            pockets: vec![mask("As Ks"), mask("Qh Qd")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Omaha,
            pockets: vec![mask("As Ks Qh Jh"), mask("Ad Kd Qc Jc")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Omaha5,
            pockets: vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Omaha6,
            pockets: vec![mask("As Ks Qh Jh 9c 8c"), mask("Ad Kd Qc Jc 9d 8d")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Omaha8,
            pockets: vec![mask("As Ks Qh Jh"), mask("Ad Kd Qc Jc")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Omaha85,
            pockets: vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::ShortDeck,
            pockets: vec![mask("As Ks"), mask("Qh Qd")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Stud7,
            pockets: vec![mask("As Ks Qh"), mask("Ad Kd Qc")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Stud78,
            pockets: vec![mask("As Ks Qh"), mask("Ad Kd Qc")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Stud7nsq,
            pockets: vec![mask("As Ks Qh"), mask("Ad Kd Qc")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Razz,
            pockets: vec![mask("As 2s 3h"), mask("Ad 2d 3c")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Draw5,
            pockets: vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Draw58,
            pockets: vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Draw5nsq,
            pockets: vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Lowball,
            pockets: vec![mask("As 2s 3h 4h 5c"), mask("Ad 2d 3c 4c 6d")],
            board: mask(""),
            nboard: 0,
        },
        SampleCase {
            game: Game::Lowball27,
            pockets: vec![mask("As 2s 3h 4h 5c"), mask("Ad 2d 3c 4c 6d")],
            board: mask(""),
            nboard: 0,
        },
    ];

    for case in cases {
        let mut result = EnumResult::new(case.game);
        enum_sample(
            case.game,
            &case.pockets,
            case.board,
            mask(""),
            case.pockets.len(),
            case.nboard,
            24,
            true,
            &mut result,
        )
        .unwrap_or_else(|e| panic!("enum_sample failed for {:?}: {}", case.game, e));

        assert_eq!(result.nsamples, 24, "nsamples mismatch for {:?}", case.game);
        assert!(
            result.ordering.is_some(),
            "ordering missing for {:?}",
            case.game
        );
    }
}

#[test]
fn enum_exhaustive_supported_games_smoke() {
    let board = mask("2c 7d 9h Jc Qd");
    let dead = mask("");

    let cases = vec![
        (Game::Holdem, vec![mask("As Ks"), mask("Qh Qd")]),
        (Game::Holdem8, vec![mask("As Ks"), mask("Qh Qd")]),
        (Game::Omaha, vec![mask("As Ks Qh Jh"), mask("Ad Kd Qc Jc")]),
        (
            Game::Omaha5,
            vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
        ),
        (
            Game::Omaha6,
            vec![mask("As Ks Qh Jh 9c 8c"), mask("Ad Kd Qc Jc 9d 8d")],
        ),
        (Game::Omaha8, vec![mask("As Ks Qh Jh"), mask("Ad Kd Qc Jc")]),
        (
            Game::Omaha85,
            vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
        ),
        (Game::ShortDeck, vec![mask("As Ks"), mask("Qh Qd")]),
    ];

    for (game, pockets) in cases {
        let mut result = EnumResult::new(game);
        enum_exhaustive(game, &pockets, board, dead, 2, 5, true, &mut result)
            .unwrap_or_else(|e| panic!("enum_exhaustive failed for {:?}: {}", game, e));
        assert_eq!(result.nsamples, 1, "nsamples mismatch for {:?}", game);
        assert!(result.ordering.is_some(), "ordering missing for {:?}", game);
    }
}

#[test]
fn enum_exhaustive_stud_razz_lowball27_smoke() {
    let cases = vec![
        (
            Game::Stud7,
            vec![mask("As Ks Qh Jh 9c 8c 7c"), mask("Ad Kd Qc Jc 9d 8d 7d")],
        ),
        (
            Game::Stud78,
            vec![mask("As Ks Qh Jh 9c 8c 7c"), mask("Ad Kd Qc Jc 9d 8d 7d")],
        ),
        (
            Game::Stud7nsq,
            vec![mask("As Ks Qh Jh 9c 8c 7c"), mask("Ad Kd Qc Jc 9d 8d 7d")],
        ),
        (
            Game::Razz,
            vec![mask("As 2s 3h 4h 5c 6c 7c"), mask("Ad 2d 3c 4c 5d 6d 7d")],
        ),
        (
            Game::Lowball27,
            vec![mask("As 2s 3h 4h 5c"), mask("Ad 2d 3c 4c 6d")],
        ),
    ];

    for (game, pockets) in cases {
        let mut result = EnumResult::new(game);
        enum_exhaustive(game, &pockets, mask(""), mask(""), 2, 0, true, &mut result)
            .unwrap_or_else(|e| panic!("enum_exhaustive failed for {:?}: {}", game, e));
        assert_eq!(result.nsamples, 1, "nsamples mismatch for {:?}", game);
        assert!(result.ordering.is_some(), "ordering missing for {:?}", game);
    }
}

#[test]
fn enum_exhaustive_draw_and_lowball_smoke() {
    let cases = vec![
        (
            Game::Draw5,
            vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
        ),
        (
            Game::Draw58,
            vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
        ),
        (
            Game::Draw5nsq,
            vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
        ),
        (
            Game::Lowball,
            vec![mask("As 2s 3h 4h 5c"), mask("Ad 2d 3c 4c 6d")],
        ),
    ];

    for (game, pockets) in cases {
        let mut result = EnumResult::new(game);
        enum_exhaustive(game, &pockets, mask(""), mask(""), 2, 0, true, &mut result)
            .unwrap_or_else(|e| panic!("enum_exhaustive failed for {:?}: {}", game, e));
        assert_eq!(result.nsamples, 1, "nsamples mismatch for {:?}", game);
        assert!(result.ordering.is_some(), "ordering missing for {:?}", game);
    }
}

#[test]
fn enum_qmc_numgames_is_unsupported() {
    let pockets = vec![mask("As Ks"), mask("Qh Qd")];
    let mut result = EnumResult::new(Game::Holdem);

    let err = enum_qmc(
        Game::NumGames,
        &pockets,
        mask(""),
        mask(""),
        2,
        0,
        24,
        false,
        &mut result,
    )
    .unwrap_err();

    assert_eq!(err, PokerError::UnsupportedGameType);
}

#[test]
fn enum_qmc_omaha_variants_smoke() {
    let cases = vec![
        (Game::Omaha, vec![mask("As Ks Qh Jh"), mask("Ad Kd Qc Jc")]),
        (Game::Omaha8, vec![mask("As Ks Qh Jh"), mask("Ad Kd Qc Jc")]),
        (
            Game::Omaha85,
            vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
        ),
    ];

    for (game, pockets) in cases {
        let mut result = EnumResult::new(game);
        enum_qmc(
            game,
            &pockets,
            mask(""),
            mask(""),
            2,
            0,
            24,
            true,
            &mut result,
        )
        .unwrap_or_else(|e| panic!("enum_qmc failed for {:?}: {}", game, e));
        assert_eq!(result.nsamples, 24, "nsamples mismatch for {:?}", game);
        assert!(result.ordering.is_some(), "ordering missing for {:?}", game);
    }
}

#[test]
fn enum_qmc_stud_razz_lowball27_smoke() {
    let cases = vec![
        (Game::Stud7, vec![mask("As Ks Qh"), mask("Ad Kd Qc")]),
        (Game::Stud78, vec![mask("As Ks Qh"), mask("Ad Kd Qc")]),
        (Game::Stud7nsq, vec![mask("As Ks Qh"), mask("Ad Kd Qc")]),
        (Game::Razz, vec![mask("As 2s 3h"), mask("Ad 2d 3c")]),
        (Game::Lowball27, vec![mask("As 2s 3h"), mask("Ad 2d 3c")]),
    ];

    for (game, pockets) in cases {
        let mut result = EnumResult::new(game);
        enum_qmc(
            game,
            &pockets,
            mask(""),
            mask(""),
            2,
            0,
            24,
            true,
            &mut result,
        )
        .unwrap_or_else(|e| panic!("enum_qmc failed for {:?}: {}", game, e));
        assert_eq!(result.nsamples, 24, "nsamples mismatch for {:?}", game);
        assert!(result.ordering.is_some(), "ordering missing for {:?}", game);
    }
}

#[test]
fn enum_qmc_draw_and_lowball_smoke() {
    let cases = vec![
        (
            Game::Draw5,
            vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
        ),
        (
            Game::Draw58,
            vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
        ),
        (
            Game::Draw5nsq,
            vec![mask("As Ks Qh Jh 9c"), mask("Ad Kd Qc Jc 9d")],
        ),
        (
            Game::Lowball,
            vec![mask("As 2s 3h 4h 5c"), mask("Ad 2d 3c 4c 6d")],
        ),
    ];

    for (game, pockets) in cases {
        let mut result = EnumResult::new(game);
        enum_qmc(
            game,
            &pockets,
            mask(""),
            mask(""),
            2,
            0,
            24,
            true,
            &mut result,
        )
        .unwrap_or_else(|e| panic!("enum_qmc failed for {:?}: {}", game, e));
        assert_eq!(result.nsamples, 24, "nsamples mismatch for {:?}", game);
        assert!(result.ordering.is_some(), "ordering missing for {:?}", game);
    }
}

#[test]
fn enum_qmc_holdem8_smoke() {
    let pockets = vec![mask("As Ks"), mask("Qh Qd")];
    let mut result = EnumResult::new(Game::Holdem8);
    enum_qmc(
        Game::Holdem8,
        &pockets,
        mask(""),
        mask(""),
        2,
        0,
        24,
        true,
        &mut result,
    )
    .unwrap();

    assert_eq!(result.nsamples, 24);
    assert!(result.ordering.is_some());
}

#[test]
fn enum_qmc_shortdeck_smoke() {
    let pockets = vec![mask("As Ks"), mask("Qh Qd")];
    let mut result = EnumResult::new(Game::ShortDeck);
    enum_qmc(
        Game::ShortDeck,
        &pockets,
        mask(""),
        mask(""),
        2,
        0,
        24,
        true,
        &mut result,
    )
    .unwrap();

    assert_eq!(result.nsamples, 24);
    assert!(result.ordering.is_some());
}
