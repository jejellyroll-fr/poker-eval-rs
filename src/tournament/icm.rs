//! Independent Chip Model (ICM) implementation.
//!
//! ICM is used to estimate the equity of players in a tournament based on their
//! chip stacks and the prize pool distribution.

use crate::errors::PokerError;
use std::collections::HashMap;

/// Calculator for Independent Chip Model (ICM).
pub struct ICMCalculator;

/// Computes ICM equities using Malmuth-Harville recursion with memoization.
pub fn calculate_icm(stacks: &[f64], prizes: &[f64]) -> Result<Vec<f64>, PokerError> {
    validate_inputs(stacks, prizes)?;

    let n = stacks.len();
    let m = prizes.len().min(n);
    let mut normalized_prizes = vec![0.0; m];
    normalized_prizes.copy_from_slice(&prizes[..m]);

    let full_mask = if n == 64 { u64::MAX } else { (1u64 << n) - 1 };
    let mut results = vec![0.0; n];

    for (player, result_item) in results.iter_mut().enumerate().take(n) {
        let mut memo: HashMap<(u64, usize), f64> = HashMap::new();
        *result_item =
            player_equity_memo(player, full_mask, 0, stacks, &normalized_prizes, &mut memo);
    }

    Ok(results)
}

impl ICMCalculator {
    /// Calculates the equity of each player using the Malmuth-Harville algorithm.
    ///
    /// # Arguments
    ///
    /// * `stacks` - A slice of chip stacks for each player.
    /// * `prizes` - A slice of prize values (ordered from 1st place to Nth place).
    ///
    /// # Returns
    ///
    /// A vector of equity values (monetary value) for each player.
    pub fn calculate(stacks: &[f64], prizes: &[f64]) -> Vec<f64> {
        calculate_icm(stacks, prizes).unwrap_or_else(|_| vec![0.0; stacks.len()])
    }
}

fn validate_inputs(stacks: &[f64], prizes: &[f64]) -> Result<(), PokerError> {
    if stacks.is_empty() {
        return Err(PokerError::InvalidInput(
            "stacks must not be empty".to_string(),
        ));
    }
    if prizes.is_empty() {
        return Err(PokerError::InvalidInput(
            "prizes must not be empty".to_string(),
        ));
    }
    if stacks.len() > 64 {
        return Err(PokerError::InvalidInput(
            "ICM currently supports at most 64 players".to_string(),
        ));
    }

    let mut total_chips = 0.0;
    for &s in stacks {
        if !s.is_finite() || s < 0.0 {
            return Err(PokerError::InvalidInput(
                "stacks must be finite and >= 0".to_string(),
            ));
        }
        total_chips += s;
    }
    if total_chips <= 0.0 {
        return Err(PokerError::InvalidInput(
            "sum of stacks must be > 0".to_string(),
        ));
    }

    for &p in prizes {
        if !p.is_finite() || p < 0.0 {
            return Err(PokerError::InvalidInput(
                "prizes must be finite and >= 0".to_string(),
            ));
        }
    }
    Ok(())
}

fn player_equity_memo(
    target_player: usize,
    active_mask: u64,
    prize_idx: usize,
    stacks: &[f64],
    prizes: &[f64],
    memo: &mut HashMap<(u64, usize), f64>,
) -> f64 {
    if prize_idx >= prizes.len() {
        return 0.0;
    }
    if (active_mask & (1u64 << target_player)) == 0 {
        return 0.0;
    }
    if let Some(v) = memo.get(&(active_mask, prize_idx)) {
        return *v;
    }

    let mut total = 0.0;
    let mut i = active_mask;
    while i != 0 {
        let p = i.trailing_zeros() as usize;
        total += stacks[p];
        i &= i - 1;
    }
    if total <= 0.0 {
        memo.insert((active_mask, prize_idx), 0.0);
        return 0.0;
    }

    let mut equity = (stacks[target_player] / total) * prizes[prize_idx];
    if prize_idx + 1 < prizes.len() {
        let mut bits = active_mask;
        while bits != 0 {
            let winner = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            if winner == target_player {
                continue;
            }
            let p_winner = stacks[winner] / total;
            let next_mask = active_mask & !(1u64 << winner);
            equity += p_winner
                * player_equity_memo(
                    target_player,
                    next_mask,
                    prize_idx + 1,
                    stacks,
                    prizes,
                    memo,
                );
        }
    }

    memo.insert((active_mask, prize_idx), equity);
    equity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icm_basic() {
        let stacks = vec![5000.0, 5000.0];
        let prizes = vec![100.0, 0.0];
        let result = ICMCalculator::calculate(&stacks, &prizes);

        assert_eq!(result[0], 50.0);
        assert_eq!(result[1], 50.0);
    }

    #[test]
    fn test_icm_three_players() {
        let stacks = vec![5000.0, 3000.0, 2000.0];
        let prizes = vec![50.0, 30.0, 20.0];
        let result = ICMCalculator::calculate(&stacks, &prizes);

        let total_equity: f64 = result.iter().sum();
        let total_prizes: f64 = prizes.iter().sum();

        assert!((total_equity - total_prizes).abs() < 1e-9);
        assert!(result[0] > result[1]);
        assert!(result[1] > result[2]);
    }

    #[test]
    fn test_calculate_icm_invalid_inputs() {
        let err = calculate_icm(&[], &[100.0]).unwrap_err();
        assert!(matches!(err, PokerError::InvalidInput(_)));

        let err = calculate_icm(&[0.0, 0.0], &[100.0]).unwrap_err();
        assert!(matches!(err, PokerError::InvalidInput(_)));

        let err = calculate_icm(&[100.0, -1.0], &[100.0]).unwrap_err();
        assert!(matches!(err, PokerError::InvalidInput(_)));
    }

    #[test]
    fn test_calculate_icm_sum_of_equities_matches_prize_sum() {
        let stacks = vec![5000.0, 3000.0, 2000.0, 1000.0];
        let prizes = vec![60.0, 25.0, 15.0];
        let result = calculate_icm(&stacks, &prizes).unwrap();

        let total_equity: f64 = result.iter().sum();
        let total_prizes: f64 = prizes.iter().sum();
        assert!((total_equity - total_prizes).abs() < 1e-9);
    }
}
