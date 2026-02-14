use crate::deck::{StdDeckCardMask, STD_DECK_N_CARDS};
use crate::enumerate::CardMask;
use crate::evaluators::{HandEvaluator, HoldemEvaluator};
use crate::range::HandRange;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

/// Result of an equity calculation.
#[derive(Debug, Clone, Copy, Default)]
pub struct EquityResult {
    pub wins: usize,
    pub ties: usize,
    pub losses: usize,
    pub samples: usize,
    pub equity: f64,
}

/// Calculates equity between two ranges using Monte Carlo simulation.
///
/// # Arguments
/// * `range1` - The hand range for player 1.
/// * `range2` - The hand range for player 2.
/// * `board` - The current board (can be empty).
/// * `iterations` - Number of Monte Carlo samples to run.
///
/// # Returns
///
/// `EquityResult` containing win/tie/loss counts and equity percentage for player 1.
pub fn calculate_equity(
    range1: &HandRange,
    range2: &HandRange,
    board: &StdDeckCardMask,
    iterations: usize,
) -> Result<EquityResult, String> {
    if range1.is_empty() || range2.is_empty() {
        return Err("Ranges cannot be empty".to_string());
    }

    let mut rng = thread_rng();
    let mut wins = 0;
    let mut ties = 0;
    let mut losses = 0;
    let mut samples = 0;

    // Pre-filter ranges to remove hands that conflict with the board?
    // Doing it once here is efficient.
    // hands now contains (mask, weight)
    let hands1: Vec<_> = range1
        .hands()
        .iter()
        .filter(|(h, _)| !h.overlaps(board))
        .collect();
    let hands2: Vec<_> = range2
        .hands()
        .iter()
        .filter(|(h, _)| !h.overlaps(board))
        .collect();

    if hands1.is_empty() || hands2.is_empty() {
        return Err("All hands in range overlap with board".to_string());
    }

    // Create weighted distributions
    let dist1 = WeightedIndex::new(hands1.iter().map(|item| item.1))
        .map_err(|e| format!("Invalid weights for range 1: {}", e))?;
    let dist2 = WeightedIndex::new(hands2.iter().map(|item| item.1))
        .map_err(|e| format!("Invalid weights for range 2: {}", e))?;

    for _ in 0..iterations {
        // 1. Pick hand 1
        let (h1, _) = hands1[dist1.sample(&mut rng)];

        // 2. Pick hand 2 (must not overlap h1)
        // We try a few times to pick a non-overlapping hand.
        let mut h2_idx = dist2.sample(&mut rng);
        let (mut h2, _) = hands2[h2_idx];

        let mut retries = 0;
        while h1.overlaps(&h2) {
            h2_idx = dist2.sample(&mut rng);
            h2 = hands2[h2_idx].0;
            retries += 1;
            if retries > 10 {
                // If we can't find a valid matchup quickly, skip this sample
                break;
            }
        }
        if h1.overlaps(&h2) {
            continue;
        }

        // 3. Complete the board
        // Dead cards = h1 + h2 + board
        let mut dead = *board;
        dead.or(h1);
        dead.or(&h2);

        let current_board_count = board.num_cards();
        let cards_needed = 5 - current_board_count;

        let final_board = if cards_needed > 0 {
            // Sample `cards_needed` from remaining deck
            // We can use a simple sampling helper here.
            // Since we are inside a tight loop, we want something fast.
            // Let's implement a quick sampler or use the one from montecarlo if public.
            // For now, inline simple sampling.

            // Construct candidate deck
            // Iterating 52 bits to build deck every time is slow.
            // Better: pick random indices until valid.
            // Or: use the internal `enumerate` helpers if accessible.
            // Let's rely on `dead` mask collision check.

            let mut drawn_board = *board;
            let mut drawn_count = 0;
            while drawn_count < cards_needed {
                let c = rand::random::<usize>() % STD_DECK_N_CARDS;
                let mask = StdDeckCardMask::from_card_index(c); // Need this helper
                if !dead.overlaps(&mask) {
                    dead.or(&mask);
                    drawn_board.or(&mask);
                    drawn_count += 1;
                }
            }
            drawn_board
        } else {
            *board
        };

        // 4. Evaluate
        let val1 = HoldemEvaluator::evaluate_hand(h1, &final_board).unwrap(); // Should not fail
        let val2 = HoldemEvaluator::evaluate_hand(&h2, &final_board).unwrap();

        if val1 > val2 {
            wins += 1;
        } else if val1 < val2 {
            losses += 1;
        } else {
            ties += 1;
        }
        samples += 1;
    }

    if samples == 0 {
        return Err(
            "No valid samples generated (ranges might be disjoint or fully overlapping)"
                .to_string(),
        );
    }

    Ok(EquityResult {
        wins,
        ties,
        losses,
        samples,
        equity: (wins as f64 + (ties as f64 / 2.0)) / samples as f64,
    })
}

// Helpers needed in StdDeckCardMask:
// - all_cards() -> 0..52 set
// - from_card_index(usize) -> mask

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_equity_aa_vs_kk() {
        let range1 = HandRange::from_str("AA").unwrap();
        let range2 = HandRange::from_str("KK").unwrap();
        let board = StdDeckCardMask::default(); // Empty board

        // AA vs KK is approx 82% vs 18%
        let result = calculate_equity(&range1, &range2, &board, 1000).unwrap();

        println!(
            "AA vs KK equity: {:.2}% (samples: {})",
            result.equity * 100.0,
            result.samples
        );

        // Allow some variance, but AA should be clear favorite
        assert!(result.equity > 0.75);
        assert!(result.equity < 0.90);
    }

    #[test]
    fn test_equity_ak_vs_22() {
        let range1 = HandRange::from_str("AKs").unwrap();
        let range2 = HandRange::from_str("22").unwrap();
        let board = StdDeckCardMask::default();

        // Coin flip, slightly favoring 22 usually (approx 52 vs 48 if suited?) or close to 50/50
        // AKs vs 22 is ~48% vs 52%
        let result = calculate_equity(&range1, &range2, &board, 1000).unwrap();

        println!("AKs vs 22 equity: {:.2}%", result.equity * 100.0);
        assert!(result.equity > 0.40);
        assert!(result.equity < 0.60);
    }
}
