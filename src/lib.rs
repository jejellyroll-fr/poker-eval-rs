#![allow(dead_code)]
// Importez les modules nécessaires
mod deck;
mod deck_std;
mod eval;
mod eval_low;
mod handval;
mod handval_low;
mod lowball;
mod rules_std;
mod t_botcard;
mod t_cardmasks;
mod t_nbits;
mod t_straight;
mod t_topcard;
mod t_topfivecards;

use crate::eval::Eval;
use crate::eval_low::std_deck_lowball_eval;
use deck_std::*;

use pyo3::prelude::*;

#[pyfunction]
fn string_to_mask(input: &str) -> PyResult<String> {
    let (mask, _n) = StdDeck::string_to_mask(input); // Votre logique originale
    Ok(format!("{:b}", mask.mask)) // Convertir en binaire et retourner
}

#[pyfunction]
fn eval_n(input: &str) -> PyResult<String> {
    let (mask, num_cards) = StdDeck::string_to_mask(input);
    let hand_val = Eval::eval_n(&mask, num_cards);
    Ok(hand_val.std_rules_hand_val_to_string())
}

#[pyfunction]
fn eval_low(input: &str) -> PyResult<String> {
    let (mask, num_cards) = StdDeck::string_to_mask(input);
    let low_hand_val = std_deck_lowball_eval(&mask, num_cards);
    Ok(low_hand_val.to_string())
}

#[pymodule]
fn poker_eval_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(string_to_mask, m)?)?;
    m.add_function(wrap_pyfunction!(eval_n, m)?)?;
    m.add_function(wrap_pyfunction!(eval_low, m)?)?;
    Ok(())
}
