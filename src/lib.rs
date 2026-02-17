// #![warn(missing_docs)] // Disabled to allow CI to pass without documenting everything immediately
//! # poker-eval-rs
//!
//! A high-performance poker hand evaluation library for Rust.
//!
//! This crate provides tools for:
//! - Evaluating hand values for various poker variants (Hold'em, Omaha, Lowball).
//! - Exhaustive and Monte Carlo equity calculations.
//! - Parallelized evaluation using [Rayon](https://crates.io/crates/rayon).
//! - Serialization support via [Serde](https://crates.io/crates/serde).
//!
//! ## Quick Start
//!
//! ```rust
//! use poker_eval_rs::deck::StdDeck;
//! use poker_eval_rs::evaluators::{HoldemEvaluator, OmahaHiEvaluator, HandEvaluator};
//!
//! // Texas Hold'em Evaluation
//! let (hole, _) = StdDeck::string_to_mask("AsKs").unwrap();
//! let (board, _) = StdDeck::string_to_mask("QsJsTs5h2d").unwrap();
//! let hand_val = HoldemEvaluator::evaluate_hand(&hole, &board).unwrap();
//! println!("Hold'em Hand: {}", hand_val.std_rules_hand_val_to_string());
//!
//! // Omaha High Evaluation
//! let (omaha_hole, _) = StdDeck::string_to_mask("AsKs2h2d").unwrap();
//! let (omaha_board, _) = StdDeck::string_to_mask("QsJsTs").unwrap();
//! let omaha_val = OmahaHiEvaluator::evaluate_hand(&omaha_hole, &omaha_board).unwrap().unwrap();
//! println!("Omaha Hand: {}", omaha_val.std_rules_hand_val_to_string());
//! ```
//!
//! ## Python Bindings
//!
//! This crate includes optional Python bindings. Enable the `python` feature to build the extension module.
//!
//! ```bash
//! maturin develop --features python
//! ```

pub mod board;
pub mod combinations;
pub mod deck;
pub mod enumdefs;
pub mod enumerate;
pub(crate) mod enumord;
pub mod errors;
pub mod evaluators;
#[cfg(feature = "gpu")]
pub mod gpu;
pub mod handval;
pub mod handval_low;
pub mod range;
pub mod rules;
pub mod solvers;
pub(crate) mod tables;
pub mod tournament;

// ===== Python Bindings (optional, enabled with "python" feature) =====
#[cfg(feature = "python")]
mod python_bindings;

#[cfg(feature = "python")]
pub use python_bindings::poker_eval_rs;

// ===== WASM Bindings (optional, enabled with "wasm" feature) =====
#[cfg(feature = "wasm")]
pub mod wasm_bindings;
