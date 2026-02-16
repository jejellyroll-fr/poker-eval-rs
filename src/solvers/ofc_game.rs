use crate::evaluators::ofc::OFCBoard;
use serde::{Deserialize, Serialize};

/// Represents the state of an OFC game for a single player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OFCGameState {
    pub board: OFCBoard,
    pub cards_placed: usize,
    pub deck: Vec<usize>,
    pub deck_ptr: usize,
    pub current_player: usize,
}

impl OFCGameState {
    pub fn new(deck: Vec<usize>) -> Self {
        Self {
            board: OFCBoard::default(),
            cards_placed: 0,
            deck,
            deck_ptr: 0,
            current_player: 0,
        }
    }

    /// Returns the number of cards already in each row.
    pub fn row_counts(&self) -> (usize, usize, usize) {
        (
            self.board.top.num_cards(),
            self.board.middle.num_cards(),
            self.board.bottom.num_cards(),
        )
    }

    /// Returns legal placements for a set of cards.
    /// Action is encoded as (row_top, row_mid, row_bot) bitsets or simpler.
    /// For a single card, it's just 0, 1, 2.
    pub fn legal_rows_for_card(&self) -> Vec<usize> {
        let (t, m, b) = self.row_counts();
        let mut rows = Vec::new();
        if t < 3 {
            rows.push(0);
        }
        if m < 5 {
            rows.push(1);
        }
        if b < 5 {
            rows.push(2);
        }
        rows
    }

    /// Places a card in a row.
    pub fn place_card(&mut self, card_index: usize, row: usize) {
        match row {
            0 => self.board.top.set(card_index),
            1 => self.board.middle.set(card_index),
            2 => self.board.bottom.set(card_index),
            _ => panic!("Invalid row"),
        }
        self.cards_placed += 1;
    }
}

/// Simple greedy solver for OFC card placement.
pub struct OFCPlacementSolver;

impl OFCPlacementSolver {
    /// Finds the best placement for the next cards given current board.
    /// For now, it just tries to maximize the immediate evaluation if complete,
    /// or uses heuristics if incomplete.
    pub fn find_best_placement(state: &OFCGameState, next_cards: &[usize]) -> Vec<usize> {
        // This is a complex optimization problem.
        // For 1 card, it's easy: try 3 rows.
        // For 5 cards, it's 3^5 combinations.

        let mut best_rows = Vec::new();
        let mut best_score = -1000.0;

        let legal_rows = state.legal_rows_for_card();

        if next_cards.len() == 1 {
            let card = next_cards[0];
            for &row in &legal_rows {
                let mut next_state = state.clone();
                next_state.place_card(card, row);
                let score = Self::evaluate_position_heuristic(&next_state);
                if score > best_score {
                    best_score = score;
                    best_rows = vec![row];
                }
            }
        } else {
            // For 5 cards (initial), we would need a more complex search.
            // Placeholder: just return a default valid 2-2-1 distribution.
            return vec![2, 2, 1, 1, 0]; // 2 bottom, 2 mid, 1 top
        }

        best_rows
    }

    fn evaluate_position_heuristic(state: &OFCGameState) -> f64 {
        if state.cards_placed == 13 {
            let res = state.board.evaluate();
            if res.is_foul {
                -100.0
            } else {
                res.royalties as f64
            }
        } else {
            // Heuristic for incomplete boards:
            // Prefer stronger bottom than middle, stronger middle than top.
            // Avoid cards that would likely cause a foul.
            let (_t, _m, _b) = state.row_counts();
            let mut score = 0.0;

            // Basic high card sum as proxy for strength
            score += state.board.bottom.num_cards() as f64 * 1.0;
            score += state.board.middle.num_cards() as f64 * 0.5;

            score
        }
    }
}
