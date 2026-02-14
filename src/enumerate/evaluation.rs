//! Top-level enumeration functions: exhaustive and sample-based evaluation.

use crate::enumdefs::{EnumResult, Game, ENUM_MAXPLAYERS};
use crate::enumord::EnumOrderingMode;
use crate::errors::PokerError;
use crate::tables::t_cardmasks::StdDeckCardMask;

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
        Game::Omaha => {
            result.exhaustive_omaha_evaluation(pockets, board, dead, npockets, nboard)?;
        }
        Game::ShortDeck => {
            result.exhaustive_short_deck_evaluation(pockets, board, dead, npockets, nboard)?;
        }
        _ => return Err(PokerError::UnsupportedGameType),
    }

    Ok(())
}
