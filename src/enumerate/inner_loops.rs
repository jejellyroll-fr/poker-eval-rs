//! Inner loop functions for evaluating poker hands across different game variants.
//!
//! Each function evaluates all players' hands for a specific game type,
//! populating `hival` and `loval` arrays with the evaluation results.

use crate::errors::PokerError;
use crate::evaluators::{
    joker_lowball8_eval, joker_lowball_eval, std_deck_lowball27_eval, std_deck_lowball_eval,
    std_deck_omaha_hi_low8_eval, Eval, EvalJoker, ShortDeckEvaluator,
};
use crate::handval::HandVal;
use crate::handval_low::LowHandVal;
use crate::tables::t_cardmasks::StdDeckCardMask;
use crate::tables::t_jokercardmasks::JokerDeckCardMask;

/// Evaluates Texas Hold'em hands for all players.
#[inline]
pub fn inner_loop_holdem(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    let final_board = *board | *shared_cards;
    // Optimization: Assume 2 hole cards if confident, or pre-calc board_len.
    // Existing code assumed +2. usage of pocket.num_cards() + board_len is safer generic.
    // But holdem is usually 2.
    // Let's keep existing logic but inline.
    let n_cards = final_board.num_cards() + 2; // +2 for hole cards
    for (i, pocket) in pockets.iter().enumerate() {
        let hand = *pocket | final_board;

        hival[i] = Eval::eval_n(&hand, n_cards);

        loval[i] = LowHandVal { value: 0 };
    }
    Ok(())
}

/// Evaluates Short Deck Hold'em hands for all players.
#[inline]
pub fn inner_loop_short_deck(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    let final_board = *board | *shared_cards;
    let board_len = final_board.num_cards();
    for (i, pocket) in pockets.iter().enumerate() {
        let hand = *pocket | final_board;
        // Optimization: Pre-calc length (pocket + board) to avoid popcounting the whole hand.
        // Pockets are disjoint from board.
        // Optimization 2: Use evaluate_combined to skip re-or'ing inside evaluator (we already did it).
        let n_cards = pocket.num_cards() + board_len;
        hival[i] = ShortDeckEvaluator::evaluate_combined(&hand, n_cards)?;
        loval[i] = LowHandVal { value: 0 };
    }
    Ok(())
}

/// Generic Omaha inner loop shared by all Omaha variants.
///
/// When `use_low` is `true`, evaluates and stores the low hand value.
/// When `use_low` is `false`, sets the low hand value to zero.
#[inline]
fn inner_loop_omaha_generic(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
    use_low: bool,
) -> Result<(), PokerError> {
    let final_board = *board | *shared_cards;
    for (i, pocket) in pockets.iter().enumerate() {
        // Omaha always evaluates exactly 5 cards (2 hole + 3 board) regardless of total cards available?
        // Wait, Omaha *rules* require using exactly 2 hole and 3 board.
        // std_deck_omaha_hi_low8_eval handles this selection internally.
        // It selects the best 5 card hand.
        // So we don't need to pass a count to it.
        // But let's verify if std_deck_omaha_hi_low8_eval uses a card count internally?
        // It uses `allcards.num_cards()` for the lowball check part in the fix I saw earlier.
        // But for high hand, it iterates combinations.
        let mut high_option: Option<HandVal> = None;
        let mut low_option: Option<LowHandVal> = None;

        std_deck_omaha_hi_low8_eval(*pocket, final_board, &mut high_option, &mut low_option)?;

        if let Some(high_hand) = high_option {
            hival[i] = high_hand;
        }

        if use_low {
            if let Some(low_hand) = low_option {
                loval[i] = low_hand;
            }
        } else {
            loval[i] = LowHandVal { value: 0 };
        }
    }
    Ok(())
}

/// Evaluates Omaha (4-card) hi-only hands for all players.
#[inline]
pub fn inner_loop_omaha(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    inner_loop_omaha_generic(pockets, board, shared_cards, hival, loval, false)
}

/// Evaluates Omaha 5-card hi-only hands for all players.
#[inline]
pub fn inner_loop_omaha5(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    inner_loop_omaha_generic(pockets, board, shared_cards, hival, loval, false)
}

/// Evaluates Omaha 6-card hi-only hands for all players.
#[inline]
pub fn inner_loop_omaha6(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    inner_loop_omaha_generic(pockets, board, shared_cards, hival, loval, false)
}

/// Evaluates Omaha 4-card hi/lo hands for all players.
#[inline]
pub fn inner_loop_omaha8(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    inner_loop_omaha_generic(pockets, board, shared_cards, hival, loval, true)
}

/// Evaluates Omaha 5-card hi/lo hands for all players.
#[inline]
pub fn inner_loop_omaha85(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    inner_loop_omaha_generic(pockets, board, shared_cards, hival, loval, true)
}

/// Evaluates 7-Card Stud hi-only hands for all players.
#[inline]
pub fn inner_loop_7stud(
    pockets: &[StdDeckCardMask],
    unshared_cards: &[StdDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    for (i, pocket) in pockets.iter().enumerate() {
        if i >= unshared_cards.len() {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Insufficient unshared cards for index {}",
                i
            )));
        }

        let hand = *pocket | unshared_cards[i];
        hival[i] = Eval::eval_n(&hand, hand.num_cards());
        loval[i] = LowHandVal { value: 0 };
    }
    Ok(())
}

/// Evaluates 7-Card Stud hi/lo (no qualifier) hands for all players.
#[inline]
pub fn inner_loop_7studnsq(
    pockets: &[StdDeckCardMask],
    unshared_cards: &[StdDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    for (i, pocket) in pockets.iter().enumerate() {
        if i >= unshared_cards.len() {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Insufficient unshared cards for index {}",
                i
            )));
        }
        let hand = *pocket | unshared_cards[i];
        hival[i] = Eval::eval_n(&hand, hand.num_cards());
        loval[i] = std_deck_lowball_eval(&hand, hand.num_cards());
    }
    Ok(())
}

/// Evaluates Razz (A-5 lowball) hands for all players.
#[inline]
pub fn inner_loop_razz(
    pockets: &[StdDeckCardMask],
    unshared_cards: &[StdDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    for (i, pocket) in pockets.iter().enumerate() {
        if i >= unshared_cards.len() {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Insufficient unshared cards for index {}",
                i
            )));
        }
        let hand = *pocket | unshared_cards[i];
        hival[i] = HandVal { value: 0 };
        loval[i] = std_deck_lowball_eval(&hand, hand.num_cards());
    }
    Ok(())
}

/// Evaluates 5-Card Draw hi-only hands (with joker) for all players.
#[inline]
pub fn inner_loop_5draw(
    pockets: &[JokerDeckCardMask],
    unshared_cards: &[JokerDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    for (i, pocket) in pockets.iter().enumerate() {
        if i >= unshared_cards.len() {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Insufficient unshared cards for index {}",
                i
            )));
        }

        let hand = *pocket | unshared_cards[i];
        hival[i] = EvalJoker::eval_n(hand, hand.num_cards());
        loval[i] = LowHandVal { value: 0 };
    }
    Ok(())
}

/// Evaluates 5-Card Draw hi/lo 8-or-better hands (with joker) for all players.
#[inline]
pub fn inner_loop_5draw8(
    pockets: &[JokerDeckCardMask],
    unshared_cards: &[JokerDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    for (i, pocket) in pockets.iter().enumerate() {
        if i >= unshared_cards.len() {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Insufficient unshared cards for index {}",
                i
            )));
        }

        let hand = *pocket | unshared_cards[i];
        hival[i] = EvalJoker::eval_n(hand, hand.num_cards());
        loval[i] = joker_lowball8_eval(&hand, hand.num_cards());
    }
    Ok(())
}

/// Evaluates 5-Card Draw hi/lo (no qualifier, with joker) hands for all players.
#[inline]
pub fn inner_loop_5drawnsq(
    pockets: &[JokerDeckCardMask],
    unshared_cards: &[JokerDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    for (i, pocket) in pockets.iter().enumerate() {
        if i >= unshared_cards.len() {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Insufficient unshared cards for index {}",
                i
            )));
        }

        let hand = *pocket | unshared_cards[i];
        hival[i] = EvalJoker::eval_n(hand, hand.num_cards());
        loval[i] = joker_lowball_eval(&hand, hand.num_cards());
    }
    Ok(())
}

/// Evaluates A-5 Lowball hands (with joker) for all players.
#[inline]
pub fn inner_loop_lowball(
    pockets: &[JokerDeckCardMask],
    unshared_cards: &[JokerDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    for (i, pocket) in pockets.iter().enumerate() {
        if i >= unshared_cards.len() {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Insufficient unshared cards for index {}",
                i
            )));
        }

        let hand = *pocket | unshared_cards[i];
        hival[i] = HandVal { value: 0 };
        loval[i] = joker_lowball_eval(&hand, hand.num_cards());
    }
    Ok(())
}

/// Evaluates 2-7 Lowball hands for all players.
#[inline]
pub fn inner_loop_lowball27(
    pockets: &[StdDeckCardMask],
    unshared_cards: &[StdDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) -> Result<(), PokerError> {
    for (i, pocket) in pockets.iter().enumerate() {
        if i >= unshared_cards.len() {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Insufficient unshared cards for index {}",
                i
            )));
        }

        let hand = *pocket | unshared_cards[i];
        hival[i] = HandVal { value: 0 };
        let hand_val_result = std_deck_lowball27_eval(&hand, hand.num_cards());
        loval[i] = LowHandVal {
            value: hand_val_result.value,
        };
    }
    Ok(())
}

/// Generic inner loop that evaluates hands and updates enumeration result statistics.
///
/// `evalwrap` is called for each player to get hi/lo hand values.
/// `ordering_increment` and `ordering_increment_hilo` update hand ordering histograms.
#[allow(dead_code)]
pub(crate) fn inner_loop<F, G, H>(
    npockets: usize,
    mut evalwrap: F,
    mut ordering_increment: G,
    mut ordering_increment_hilo: H,
    result: &mut crate::enumdefs::EnumResult,
) where
    F: FnMut(usize) -> (Result<HandVal, i32>, Result<LowHandVal, i32>),
    G: FnMut(&mut crate::enumdefs::EnumResult, &[usize], &[usize]),
    H: FnMut(&mut crate::enumdefs::EnumResult, &[usize], &[usize]),
{
    use crate::enumdefs::ENUM_MAXPLAYERS;
    use crate::enumord::EnumOrderingMode;
    use crate::handval_low::LOW_HAND_VAL_NOTHING;

    let handval_nothing: u32 = HandVal::new(0, 0, 0, 0, 0, 0).value;

    let mut hival = [handval_nothing; ENUM_MAXPLAYERS];
    let mut loval = [LOW_HAND_VAL_NOTHING; ENUM_MAXPLAYERS];
    let mut besthi = handval_nothing;
    let mut bestlo = LOW_HAND_VAL_NOTHING;
    let mut hishare = 0;
    let mut loshare = 0;

    for i in 0..npockets {
        let (hi_res, lo_res) = evalwrap(i);

        let hi = hi_res.map(|h| h.value).unwrap_or(handval_nothing);
        let lo = lo_res.map(|l| l.value).unwrap_or(LOW_HAND_VAL_NOTHING);

        hival[i] = hi;
        loval[i] = lo;

        if hi != handval_nothing {
            if hi > besthi {
                besthi = hi;
                hishare = 1;
            } else if hi == besthi {
                hishare += 1;
            }
        }

        if lo != LOW_HAND_VAL_NOTHING {
            if lo < bestlo {
                bestlo = lo;
                loshare = 1;
            } else if lo == bestlo {
                loshare += 1;
            }
        }
    }

    let hipot = if besthi != handval_nothing {
        1.0 / hishare as f64
    } else {
        0.0
    };
    let lopot = if bestlo != LOW_HAND_VAL_NOTHING {
        1.0 / loshare as f64
    } else {
        0.0
    };

    for i in 0..npockets {
        let mut potfrac = 0.0;

        if hival[i] == besthi {
            potfrac += hipot;
            if hishare == 1 {
                result.nwinhi[i] += 1;
            } else {
                result.ntiehi[i] += 1;
            }
        } else if hival[i] != handval_nothing {
            result.nlosehi[i] += 1;
        }

        if loval[i] == bestlo {
            potfrac += lopot;
            if loshare == 1 {
                result.nwinlo[i] += 1;
            } else {
                result.ntielo[i] += 1;
            }
        } else if loval[i] != LOW_HAND_VAL_NOTHING {
            result.nloselo[i] += 1;
        }

        result.ev[i] += potfrac;
    }

    if let Some(ordering) = &result.ordering {
        let hiranks: Vec<_> = hival.iter().map(|&val| val as usize).collect();
        let loranks: Vec<_> = loval.iter().map(|&val| val as usize).collect();

        match ordering.mode {
            EnumOrderingMode::Hi => ordering_increment(result, &hiranks, &loranks),
            EnumOrderingMode::Lo => ordering_increment(result, &loranks, &hiranks),
            EnumOrderingMode::Hilo => ordering_increment_hilo(result, &hiranks, &loranks),
            _ => (),
        }
    }

    result.nsamples += 1;
}
