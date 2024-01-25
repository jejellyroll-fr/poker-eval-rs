#![allow(dead_code)]
// Importez les modules nécessaires
pub mod deck;
pub mod deck_std;
pub mod enumord;
pub mod eval;
pub mod eval_low;
pub mod eval_low27;
pub mod eval_low8;
pub mod handval;
pub mod handval_low;
pub mod lowball;
pub mod rules_std;
pub mod t_botcard;
pub mod t_botfivecards;
pub mod t_cardmasks;
pub mod t_nbits;
pub mod t_straight;
pub mod t_topcard;
pub mod t_topfivecards;

use crate::eval::Eval;
use crate::eval_low::std_deck_lowball_eval;
use deck_std::*;

use pyo3::prelude::*;

#[pyfunction]
fn string_to_mask(input: &str) -> PyResult<String> {
    let result = StdDeck::string_to_mask(input);
    let (mask, _num_cards) = match result {
        Ok((mask, num_cards)) => (mask, num_cards),
        Err(e) => {
            eprintln!(
                "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
                e
            );
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
                e
            )));
        }
    };
    Ok(format!("{:b}", mask.mask)) // Convertir en binaire et retourner
}

#[pyfunction]
fn eval_n(input: &str) -> PyResult<String> {
    let result = StdDeck::string_to_mask(input);
    let (mask, num_cards) = match result {
        Ok((mask, num_cards)) => (mask, num_cards),
        Err(e) => {
            eprintln!(
                "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
                e
            );
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
                e
            )));
        }
    };
    let hand_val = Eval::eval_n(&mask, num_cards);
    Ok(hand_val.std_rules_hand_val_to_string())
}

#[pyfunction]
fn eval_low_func(input: &str) -> PyResult<String> {
    let result = StdDeck::string_to_mask(input);
    let (mask, num_cards) = match result {
        Ok((mask, num_cards)) => (mask, num_cards),
        Err(e) => {
            eprintln!(
                "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
                e
            );
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
                e
            )));
        }
    };
    let low_hand_val = std_deck_lowball_eval(&mask, num_cards);
    Ok(low_hand_val.to_string())
}

#[pymodule]
fn poker_eval_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(string_to_mask, m)?)?;
    m.add_function(wrap_pyfunction!(eval_n, m)?)?;
    m.add_function(wrap_pyfunction!(eval_low_func, m)?)?;
    Ok(())
}
