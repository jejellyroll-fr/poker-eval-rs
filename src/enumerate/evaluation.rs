//! Top-level enumeration functions: exhaustive and sample-based evaluation.

use crate::enumdefs::{EnumResult, Game, ENUM_MAXPLAYERS};
use crate::enumord::EnumOrderingMode;
use crate::errors::PokerError;
use crate::tables::t_cardmasks::StdDeckCardMask;

/// Validates board configuration consistency for an enumeration request.
///
/// Checks:
/// - `nboard` matches `board.num_cards()`
/// - board size does not exceed game max board size
/// - board is empty for no-board games
/// - in exhaustive mode (board games), only supported streets are allowed
pub fn validate_enum_configuration(
    game: Game,
    board: StdDeckCardMask,
    nboard: usize,
    exhaustive: bool,
) -> Result<(), PokerError> {
    if nboard != board.num_cards() {
        return Err(PokerError::UnsupportedBoardConfiguration);
    }

    let params = game.game_params().ok_or(PokerError::UnsupportedGameType)?;
    let maxboard = params.maxboard as usize;

    if nboard > maxboard {
        return Err(PokerError::UnsupportedBoardConfiguration);
    }

    if maxboard == 0 && (!board.is_empty() || nboard != 0) {
        return Err(PokerError::UnsupportedBoardConfiguration);
    }

    if exhaustive && maxboard == 5 && !matches!(nboard, 0 | 3 | 4 | 5) {
        return Err(PokerError::UnsupportedBoardConfiguration);
    }

    Ok(())
}

/// Returns true if Monte Carlo sampling is supported for `game`.
pub fn supports_enum_sample(game: Game) -> bool {
    matches!(
        game,
        Game::Holdem
            | Game::Holdem8
            | Game::Omaha
            | Game::Omaha5
            | Game::Omaha6
            | Game::Omaha8
            | Game::Omaha85
            | Game::ShortDeck
            | Game::Stud7
            | Game::Stud78
            | Game::Stud7nsq
            | Game::Razz
            | Game::Draw5
            | Game::Draw58
            | Game::Draw5nsq
            | Game::Lowball
            | Game::Lowball27
    )
}

/// Returns true if Quasi-Monte Carlo sampling is supported for `game`.
pub fn supports_enum_qmc(game: Game) -> bool {
    matches!(
        game,
        Game::Holdem
            | Game::Holdem8
            | Game::Omaha
            | Game::Omaha5
            | Game::Omaha6
            | Game::Omaha8
            | Game::Omaha85
            | Game::ShortDeck
            | Game::Stud7
            | Game::Stud78
            | Game::Stud7nsq
            | Game::Razz
            | Game::Draw5
            | Game::Draw58
            | Game::Draw5nsq
            | Game::Lowball
            | Game::Lowball27
    )
}

/// Returns true if exhaustive enumeration is supported for `game`.
pub fn supports_enum_exhaustive(game: Game) -> bool {
    matches!(
        game,
        Game::Holdem
            | Game::Holdem8
            | Game::Omaha
            | Game::Omaha5
            | Game::Omaha6
            | Game::Omaha8
            | Game::Omaha85
            | Game::ShortDeck
            | Game::Stud7
            | Game::Stud78
            | Game::Stud7nsq
            | Game::Razz
            | Game::Draw5
            | Game::Draw58
            | Game::Draw5nsq
            | Game::Lowball
            | Game::Lowball27
    )
}

/// Runs a Monte Carlo sample evaluation for the given game and player hands.
///
/// Draws random boards `niter` times and aggregates win/tie/loss statistics.
///
/// # Examples
///
/// ```
/// use poker_eval_rs::enumerate::enum_sample;
/// use poker_eval_rs::enumdefs::{EnumResult, Game};
/// use poker_eval_rs::deck::StdDeck;
///
/// let (pocket1, _) = StdDeck::string_to_mask("As Ks").unwrap();
/// let (pocket2, _) = StdDeck::string_to_mask("2s 2d").unwrap();
/// let pockets = vec![pocket1, pocket2];
/// let board = StdDeck::string_to_mask("").unwrap().0;
/// let dead = StdDeck::string_to_mask("").unwrap().0;
/// let mut result = EnumResult::new(Game::Holdem);
///
/// enum_sample(Game::Holdem, &pockets, board, dead, 2, 0, 1000, true, &mut result).unwrap();
/// assert!(result.nsamples == 1000);
/// ```
#[allow(clippy::too_many_arguments)]
pub fn enum_sample(
    game: Game,
    pockets: &[StdDeckCardMask],
    board: StdDeckCardMask,
    dead: StdDeckCardMask,
    npockets: usize,
    nboard: usize,
    niter: usize,
    orderflag: bool,
    result: &mut EnumResult,
) -> Result<(), PokerError> {
    if npockets > ENUM_MAXPLAYERS {
        return Err(PokerError::TooManyPlayers);
    }
    result.clear();
    result.game = game;
    result.sample_type = crate::enumdefs::SampleType::Sample;
    result.nplayers = npockets as u32;
    validate_enum_configuration(game, board, nboard, false)?;

    let mode = match game {
        Game::Holdem
        | Game::Omaha
        | Game::Omaha5
        | Game::Omaha6
        | Game::Stud7
        | Game::Draw5
        | Game::ShortDeck => EnumOrderingMode::Hi,
        Game::Razz | Game::Lowball | Game::Lowball27 => EnumOrderingMode::Lo,
        Game::Holdem8
        | Game::Omaha8
        | Game::Omaha85
        | Game::Stud78
        | Game::Stud7nsq
        | Game::Draw58
        | Game::Draw5nsq => EnumOrderingMode::Hilo,
        _ => return Err(PokerError::UnsupportedGameType),
    };

    if orderflag {
        result.allocate_resources(npockets, mode)?;
    }

    match game {
        Game::Holdem => {
            result.simulate_holdem_game(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Holdem8 => {
            result.simulate_holdem8_game(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Omaha => {
            result.simulate_omaha_game(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Omaha5 => {
            result.simulate_omaha5_game(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Omaha6 => {
            result.simulate_omaha6_game(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Omaha8 => {
            result.simulate_omaha8_game(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Omaha85 => {
            // Omaha85 uses same logic as Omaha8 (5-card variant?)
            // Check result.rs simulate_omaha8_game comment: "Works for Omaha8 and Omaha85 variants."
            result.simulate_omaha8_game(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::ShortDeck => {
            result.simulate_short_deck_game(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Stud7 => {
            result.simulate_stud_game(pockets, dead, npockets, niter)?;
        }
        Game::Stud78 => {
            result.simulate_stud8_game(pockets, dead, npockets, niter)?;
        }
        Game::Stud7nsq => {
            result.simulate_studnsq_game(pockets, dead, npockets, niter)?;
        }
        Game::Razz => {
            result.simulate_razz_game(pockets, dead, npockets, niter)?;
        }
        Game::Draw5 => {
            result.simulate_draw_game(pockets, dead, npockets, niter)?;
        }
        Game::Draw58 => {
            result.simulate_draw8_game(pockets, dead, npockets, niter)?;
        }
        Game::Draw5nsq => {
            result.simulate_drawnsq_game(pockets, dead, npockets, niter)?;
        }
        Game::Lowball => {
            result.simulate_lowball_game(pockets, dead, npockets, niter)?;
        }
        Game::Lowball27 => {
            result.simulate_lowball27_game(pockets, dead, npockets, niter)?;
        }
        _ => return Err(PokerError::UnsupportedGameType),
    }
    Ok(())
}

/// Runs a Quasi-Monte Carlo (Sobol sequence) sample evaluation for the given game and player hands.
#[allow(clippy::too_many_arguments)]
pub fn enum_qmc(
    game: Game,
    pockets: &[StdDeckCardMask],
    board: StdDeckCardMask,
    dead: StdDeckCardMask,
    npockets: usize,
    nboard: usize,
    niter: usize,
    orderflag: bool,
    result: &mut EnumResult,
) -> Result<(), PokerError> {
    if npockets > ENUM_MAXPLAYERS {
        return Err(PokerError::TooManyPlayers);
    }
    result.clear();
    result.game = game;
    result.sample_type = crate::enumdefs::SampleType::QuasiMonteCarlo;
    result.nplayers = npockets as u32;
    validate_enum_configuration(game, board, nboard, false)?;

    let mode = match game {
        Game::Holdem
        | Game::Omaha
        | Game::Omaha5
        | Game::Omaha6
        | Game::Stud7
        | Game::Draw5
        | Game::ShortDeck => EnumOrderingMode::Hi,
        Game::Razz | Game::Lowball | Game::Lowball27 => EnumOrderingMode::Lo,
        Game::Holdem8
        | Game::Omaha8
        | Game::Omaha85
        | Game::Stud78
        | Game::Stud7nsq
        | Game::Draw58
        | Game::Draw5nsq => EnumOrderingMode::Hilo,
        _ => return Err(PokerError::UnsupportedGameType),
    };

    if orderflag {
        result.allocate_resources(npockets, mode)?;
    }

    match game {
        Game::Holdem => {
            result.simulate_holdem_game_qmc(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Holdem8 => {
            result.simulate_holdem8_game_qmc(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Omaha | Game::Omaha5 | Game::Omaha6 => {
            result.simulate_omaha_game_qmc(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Omaha8 | Game::Omaha85 => {
            result.simulate_omaha8_game_qmc(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::ShortDeck => {
            result.simulate_short_deck_game_qmc(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Stud7 => {
            result.simulate_stud_game_qmc(pockets, dead, npockets, niter)?;
        }
        Game::Stud78 => {
            result.simulate_stud8_game_qmc(pockets, dead, npockets, niter)?;
        }
        Game::Stud7nsq => {
            result.simulate_studnsq_game_qmc(pockets, dead, npockets, niter)?;
        }
        Game::Razz => {
            result.simulate_razz_game_qmc(pockets, dead, npockets, niter)?;
        }
        Game::Lowball27 => {
            result.simulate_lowball27_game_qmc(pockets, dead, npockets, niter)?;
        }
        Game::Draw5 => {
            result.simulate_draw_game_qmc(pockets, dead, npockets, niter)?;
        }
        Game::Draw58 => {
            result.simulate_draw8_game_qmc(pockets, dead, npockets, niter)?;
        }
        Game::Draw5nsq => {
            result.simulate_drawnsq_game_qmc(pockets, dead, npockets, niter)?;
        }
        Game::Lowball => {
            result.simulate_lowball_game_qmc(pockets, dead, npockets, niter)?;
        }
        _ => return Err(PokerError::UnsupportedGameType),
    }
    Ok(())
}

/// Runs an exhaustive (all possible boards) evaluation for the given game and player hands.
///
/// Enumerates every possible board runout and aggregates win/tie/loss/equity statistics.
///
/// # Examples
///
/// ```
/// use poker_eval_rs::enumerate::enum_exhaustive;
/// use poker_eval_rs::enumdefs::{EnumResult, Game};
/// use poker_eval_rs::deck::StdDeck;
///
/// let (pocket1, _) = StdDeck::string_to_mask("As Ks").unwrap();
/// let (pocket2, _) = StdDeck::string_to_mask("2s 2d").unwrap();
/// let pockets = vec![pocket1, pocket2];
/// let board = StdDeck::string_to_mask("7d 8c 9h").unwrap().0;
/// let dead = StdDeck::string_to_mask("").unwrap().0;
/// let mut result = EnumResult::new(Game::Holdem);
///
/// // Enumerates all combinations of remaining 2 cards (C(45, 2) = 990 iterations)
/// enum_exhaustive(Game::Holdem, &pockets, board, dead, 2, 3, true, &mut result).unwrap();
/// assert_eq!(result.nsamples, 990);
/// ```
#[allow(clippy::too_many_arguments)]
pub fn enum_exhaustive(
    game: Game,
    pockets: &[StdDeckCardMask],
    board: StdDeckCardMask,
    dead: StdDeckCardMask,
    npockets: usize,
    nboard: usize,
    orderflag: bool,
    result: &mut EnumResult,
) -> Result<(), PokerError> {
    result.clear();
    result.game = game;
    result.sample_type = crate::enumdefs::SampleType::Exhaustive;
    result.nplayers = npockets as u32;
    validate_enum_configuration(game, board, nboard, true)?;

    if npockets > ENUM_MAXPLAYERS {
        return Err(PokerError::TooManyPlayers);
    }

    if orderflag {
        let mode = match game {
            Game::Holdem
            | Game::Omaha
            | Game::Omaha5
            | Game::Omaha6
            | Game::Stud7
            | Game::Draw5
            | Game::ShortDeck => EnumOrderingMode::Hi,
            Game::Razz | Game::Lowball | Game::Lowball27 => EnumOrderingMode::Lo,
            Game::Holdem8
            | Game::Omaha8
            | Game::Omaha85
            | Game::Stud78
            | Game::Stud7nsq
            | Game::Draw58
            | Game::Draw5nsq => EnumOrderingMode::Hilo,
            _ => return Err(PokerError::UnsupportedGameType),
        };

        result.allocate_resources(npockets, mode)?;
    }

    match game {
        Game::Holdem => {
            result.exhaustive_holdem_evaluation(pockets, board, dead, npockets, nboard)?;
        }
        Game::Holdem8 => {
            result.exhaustive_holdem8_evaluation(pockets, board, dead, npockets, nboard)?;
        }
        Game::Omaha | Game::Omaha5 | Game::Omaha6 => {
            result.exhaustive_omaha_evaluation(pockets, board, dead, npockets, nboard)?;
        }
        Game::Omaha8 | Game::Omaha85 => {
            result.exhaustive_omaha8_evaluation(pockets, board, dead, npockets, nboard)?;
        }
        Game::ShortDeck => {
            result.exhaustive_short_deck_evaluation(pockets, board, dead, npockets, nboard)?;
        }
        Game::Stud7 => {
            result.exhaustive_stud_evaluation(pockets, dead, npockets)?;
        }
        Game::Stud78 => {
            result.exhaustive_stud8_evaluation(pockets, dead, npockets)?;
        }
        Game::Stud7nsq => {
            result.exhaustive_studnsq_evaluation(pockets, dead, npockets)?;
        }
        Game::Razz => {
            result.exhaustive_razz_evaluation(pockets, dead, npockets)?;
        }
        Game::Lowball27 => {
            result.exhaustive_lowball27_evaluation(pockets, dead, npockets)?;
        }
        Game::Draw5 => {
            result.exhaustive_draw_evaluation(pockets, dead, npockets)?;
        }
        Game::Draw58 => {
            result.exhaustive_draw8_evaluation(pockets, dead, npockets)?;
        }
        Game::Draw5nsq => {
            result.exhaustive_drawnsq_evaluation(pockets, dead, npockets)?;
        }
        Game::Lowball => {
            result.exhaustive_lowball_evaluation(pockets, dead, npockets)?;
        }
        _ => return Err(PokerError::UnsupportedGameType),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::StdDeck;
    use crate::enumdefs::{EnumResult, Game};

    #[test]
    fn test_mode_support_matrix() {
        assert!(supports_enum_sample(Game::Stud78));
        assert!(supports_enum_sample(Game::Draw5nsq));

        assert!(supports_enum_qmc(Game::Holdem));
        assert!(supports_enum_qmc(Game::Holdem8));
        assert!(supports_enum_qmc(Game::Omaha));
        assert!(supports_enum_qmc(Game::Omaha8));
        assert!(supports_enum_qmc(Game::ShortDeck));
        assert!(supports_enum_qmc(Game::Stud7));
        assert!(supports_enum_qmc(Game::Stud78));
        assert!(supports_enum_qmc(Game::Stud7nsq));
        assert!(supports_enum_qmc(Game::Razz));
        assert!(supports_enum_qmc(Game::Lowball27));
        assert!(supports_enum_qmc(Game::Draw5));
        assert!(supports_enum_qmc(Game::Draw58));
        assert!(supports_enum_qmc(Game::Draw5nsq));
        assert!(supports_enum_qmc(Game::Lowball));
        assert!(!supports_enum_qmc(Game::NumGames));

        assert!(supports_enum_exhaustive(Game::Holdem));
        assert!(supports_enum_exhaustive(Game::Omaha5));
        assert!(supports_enum_exhaustive(Game::Omaha6));
        assert!(supports_enum_exhaustive(Game::Omaha8));
        assert!(supports_enum_exhaustive(Game::Omaha85));
        assert!(supports_enum_exhaustive(Game::ShortDeck));
        assert!(supports_enum_exhaustive(Game::Stud7));
        assert!(supports_enum_exhaustive(Game::Stud78));
        assert!(supports_enum_exhaustive(Game::Stud7nsq));
        assert!(supports_enum_exhaustive(Game::Razz));
        assert!(supports_enum_exhaustive(Game::Draw5));
        assert!(supports_enum_exhaustive(Game::Draw58));
        assert!(supports_enum_exhaustive(Game::Draw5nsq));
        assert!(supports_enum_exhaustive(Game::Lowball));
        assert!(supports_enum_exhaustive(Game::Lowball27));
        assert!(!supports_enum_exhaustive(Game::NumGames));
    }

    #[test]
    fn test_enum_qmc() {
        let (pocket1, _) = StdDeck::string_to_mask("As Ks").unwrap();
        let (pocket2, _) = StdDeck::string_to_mask("2s 2d").unwrap();
        let pockets = vec![pocket1, pocket2];
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
            1000,
            true,
            &mut result,
        )
        .unwrap();
        assert_eq!(result.nsamples, 1000);
        assert!(result.ev[0] > 0.0);
        assert!(result.ev[1] > 0.0);
    }
}
