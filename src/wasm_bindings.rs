#![cfg(target_arch = "wasm32")]

//! WebAssembly bindings for poker-eval-rs.
//!
//! This module exposes key functionality to WASM environments via `wasm-bindgen`.

use crate::deck::{JokerDeck, StdDeck};
use crate::deck::{JokerDeckCardMask, StdDeckCardMask};
use crate::enumdefs::{EnumResult, Game, SampleType};
use crate::enumerate::evaluation::{enum_exhaustive, enum_sample};
use crate::evaluators::{
    joker_lowball_eval, EvalJoker, HandEvaluator, LowballEvaluator, OmahaHiEvaluator,
    ShortDeckEvaluator,
};
use crate::handval::HandVal;
use crate::handval_low::LowHandVal;
use serde::Serialize;
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct WasmPlayerStat {
    pub hand: String,
    pub win_pct: f64,
    pub tie_pct: f64,
    pub lose_pct: f64,
    pub ev: f64,
}

#[derive(Serialize)]
pub struct WasmEquityResult {
    pub game: String,
    pub samples: u64,
    pub players: Vec<WasmPlayerStat>,
}

/// Evaluates a single Omaha Hi hand.
#[wasm_bindgen]
pub fn eval_omaha_hi(hand: &str, board: &str) -> Result<String, JsValue> {
    let (h_mask, n_h) = StdDeck::string_to_mask(hand).map_err(|e| JsValue::from_str(&e))?;
    let (b_mask, n_b) = StdDeck::string_to_mask(board).map_err(|e| JsValue::from_str(&e))?;

    if n_h != 4 {
        return Err(JsValue::from_str("Omaha Hi requires 4 hole cards"));
    }
    if n_b < 3 {
        return Err(JsValue::from_str(
            "Omaha Hi requires at least 3 board cards",
        ));
    }

    if let Some(val) = OmahaHiEvaluator::evaluate_hand(&h_mask, &b_mask) {
        Ok(val.std_rules_hand_val_to_string())
    } else {
        Ok("No Hand".to_string())
    }
}

/// Evaluates a single Lowball (A-5) hand.
#[wasm_bindgen]
pub fn eval_lowball(hand: &str) -> Result<String, JsValue> {
    let (mask, num) = StdDeck::string_to_mask(hand).map_err(|e| JsValue::from_str(&e))?;
    // A-5 usually doesn't need board, but depends on variant.
    // Assuming 5-card Lowball.
    if num != 5 {
        return Err(JsValue::from_str("Lowball requires 5 cards"));
    }
    let board = StdDeckCardMask::new();

    if let Ok(val) = LowballEvaluator::evaluate_hand(&mask, &board) {
        Ok(val.to_string())
    } else {
        Err(JsValue::from_str("Evaluation failed"))
    }
}

/// Evaluates a single Short Deck hand.
#[wasm_bindgen]
pub fn eval_short_deck(hand: &str, board: &str) -> Result<String, JsValue> {
    let (h_mask, n_h) = StdDeck::string_to_mask(hand).map_err(|e| JsValue::from_str(&e))?;
    let (b_mask, n_b) = StdDeck::string_to_mask(board).map_err(|e| JsValue::from_str(&e))?;

    if n_h != 2 {
        return Err(JsValue::from_str("Short Deck requires 2 hole cards"));
    }
    if n_b < 3 || n_b > 5 {
        return Err(JsValue::from_str("Short Deck requires 3-5 board cards"));
    }

    match ShortDeckEvaluator::evaluate_hand(&h_mask, &b_mask) {
        Ok(val) => Ok(val.std_rules_hand_val_to_string()),
        Err(e) => Err(JsValue::from_str(&format!("Evaluation failed: {:?}", e))),
    }
}

/// Evaluates a single Joker (High) hand.
#[wasm_bindgen]
pub fn eval_joker(hand: &str) -> Result<String, JsValue> {
    // Parse using JokerDeck which handles "Xx"
    let (mask, num) = JokerDeck::string_to_mask(hand).map_err(|e| JsValue::from_str(&e))?;

    let val = EvalJoker::eval_n(mask, num);
    Ok(val.std_rules_hand_val_to_string())
}

/// Calculates equity for a set of hands.
///
/// # Arguments
///
/// * `hands_str` - Space-separated list of hands (e.g., "AsKs QdJd 22").
/// * `board_str` - Board cards (e.g., "Qs Js Ts"). Can be empty.
/// * `dead_str` - Dead cards (e.g., "2s 3s"). Can be empty.
/// * `game` - Game variant ("holdem", "omaha", "omaha8", "stud7", "razz", "lowball", etc.).
/// * `monte_carlo` - If true, uses Monte Carlo simulation. Otherwise exhaustive.
/// * `iterations` - Number of iterations for Monte Carlo simulation.
#[wasm_bindgen]
pub fn calculate_equity(
    hands_str: &str,
    board_str: &str,
    dead_str: &str,
    game: &str,
    monte_carlo: bool,
    iterations: usize,
) -> Result<JsValue, JsValue> {
    let hands: Vec<String> = hands_str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    if hands.len() < 2 {
        return Err(JsValue::from_str("Need at least 2 hands"));
    }

    let game_variant = match game.to_lowercase().as_str() {
        "holdem" => Game::Holdem,
        "holdem8" => Game::Holdem8,
        "omaha" => Game::Omaha,
        "omaha5" => Game::Omaha5,
        "omaha6" => Game::Omaha6,
        "omaha8" => Game::Omaha8,
        "omaha85" => Game::Omaha85,
        "stud7" => Game::Stud7,
        "stud78" => Game::Stud78,
        "stud7nsq" => Game::Stud7nsq,
        "razz" => Game::Razz,
        "draw5" => Game::Draw5,
        "draw58" => Game::Draw58,
        "draw5nsq" => Game::Draw5nsq,
        "lowball" => Game::Lowball,
        "lowball27" => Game::Lowball27,
        "shortdeck" => Game::ShortDeck,
        _ => return Err(JsValue::from_str("Unsupported game")),
    };

    // Parse hands
    let mut pockets: Vec<StdDeckCardMask> = Vec::new();
    for hand in &hands {
        let (mask, _) = StdDeck::string_to_mask(hand)
            .map_err(|e| JsValue::from_str(&format!("Error parsing hand {}: {}", hand, e)))?;
        pockets.push(mask);
    }

    let board_mask = if board_str.is_empty() {
        StdDeckCardMask::new()
    } else {
        StdDeck::string_to_mask(board_str)
            .map_err(|e| JsValue::from_str(&format!("Error parsing board: {}", e)))?
            .0
    };

    let dead_mask = if dead_str.is_empty() {
        StdDeckCardMask::new()
    } else {
        StdDeck::string_to_mask(dead_str)
            .map_err(|e| JsValue::from_str(&format!("Error parsing dead cards: {}", e)))?
            .0
    };

    let npockets = pockets.len();
    let nboard = board_mask.num_cards();

    let mut result = EnumResult::new(game_variant);
    result.sample_type = if monte_carlo {
        SampleType::Sample
    } else {
        SampleType::Exhaustive
    };
    result.nplayers = npockets as u32;

    let calc_result = if monte_carlo {
        enum_sample(
            game_variant,
            &pockets,
            board_mask,
            dead_mask,
            npockets,
            nboard,
            iterations,
            false,
            &mut result,
        )
    } else {
        enum_exhaustive(
            game_variant,
            &pockets,
            board_mask,
            dead_mask,
            npockets,
            nboard,
            false,
            &mut result,
        )
    };

    match calc_result {
        Ok(_) => {
            let mut player_stats = Vec::new();
            for (i, hand) in hands.iter().enumerate() {
                let total = result.nwinhi[i] + result.ntiehi[i] + result.nlosehi[i];
                if total > 0 {
                    player_stats.push(WasmPlayerStat {
                        hand: hand.clone(),
                        win_pct: (result.nwinhi[i] as f64 / total as f64) * 100.0,
                        tie_pct: (result.ntiehi[i] as f64 / total as f64) * 100.0,
                        lose_pct: (result.nlosehi[i] as f64 / total as f64) * 100.0,
                        ev: result.ev[i],
                    });
                }
            }

            let output = WasmEquityResult {
                game: game.to_string(),
                samples: result.nsamples,
                players: player_stats,
            };

            Ok(serde_wasm_bindgen::to_value(&output)?)
        }
        Err(e) => Err(JsValue::from_str(&format!("Calculation error: {:?}", e))),
    }
}
