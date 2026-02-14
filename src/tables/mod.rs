//! Precomputed lookup tables for fast hand evaluation.
//!
//! This module includes tables for card masks, top card values, and other
//! optimizations used by the evaluators.

pub(crate) mod rank_lookup;
pub mod t_botcard;
pub mod t_botfivecards;
pub mod t_cardmasks;
pub mod t_jokercardmasks;
pub mod t_jokerstraight;
pub mod t_straight;
pub mod t_topcard;
pub mod t_topfivecards;
