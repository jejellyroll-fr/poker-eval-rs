//! Poker game implementations for CFR solvers.

use crate::evaluators::HandEvaluator;
use crate::solvers::cfr::GameState;
use crate::{deck::StdDeckCardMask, handval::HandVal};
use rand::prelude::*;

#[derive(Clone)]
pub struct KuhnGameState {
    pub cards: Vec<usize>,
    pub history: Vec<usize>, // 0: Pass, 1: Bet
    pub current_player: usize,
}

impl KuhnGameState {
    pub fn new(cards: Vec<usize>) -> Self {
        Self {
            cards,
            history: Vec::new(),
            current_player: 0,
        }
    }
}

impl GameState for KuhnGameState {
    fn current_player(&self) -> usize {
        self.current_player
    }

    fn information_set_key(&self) -> String {
        let card = self.cards[self.current_player];
        let hist: String = self
            .history
            .iter()
            .map(|&a| if a == 0 { 'P' } else { 'B' })
            .collect();
        format!("{}:{}", card, hist)
    }

    fn legal_actions(&self) -> Vec<usize> {
        vec![0, 1] // 0: Pass, 1: Bet/Call
    }

    fn is_terminal(&self) -> bool {
        let h = &self.history;
        if h.len() < 2 {
            return false;
        }

        if h.len() == 2 && h[0] == 1 && h[1] == 0 {
            return true;
        } // Bet-Pass
        if h.len() == 2 && h[0] == 0 && h[1] == 0 {
            return true;
        } // Pass-Pass
        if h.len() == 3 && h[0] == 0 && h[1] == 1 && h[2] == 0 {
            return true;
        } // Pass-Bet-Pass
        if h.len() == 2 && h[0] == 1 && h[1] == 1 {
            return true;
        } // Bet-Bet
        if h.len() == 3 && h[0] == 0 && h[1] == 1 && h[2] == 1 {
            return true;
        } // Pass-Bet-Bet
        false
    }

    fn terminal_utility(&self) -> Vec<f64> {
        let h = &self.history;
        let card0 = self.cards[0];
        let card1 = self.cards[1];

        if h == &[0, 0] {
            return if card0 > card1 {
                vec![1.0, -1.0]
            } else {
                vec![-1.0, 1.0]
            };
        }
        if h == &[1, 0] {
            return vec![1.0, -1.0];
        }
        if h == &[1, 1] {
            return if card0 > card1 {
                vec![2.0, -2.0]
            } else {
                vec![-2.0, 2.0]
            };
        }
        if h == &[0, 1, 0] {
            return vec![-1.0, 1.0];
        }
        if h == &[0, 1, 1] {
            return if card0 > card1 {
                vec![2.0, -2.0]
            } else {
                vec![-2.0, 2.0]
            };
        }
        vec![0.0, 0.0]
    }

    fn act(&self, action: usize) -> Self {
        let mut next = self.clone();
        next.history.push(action);
        next.current_player = 1 - self.current_player;
        next
    }

    fn sample_chance(&self) -> Self {
        let mut next = self.clone();
        let mut rng = thread_rng();
        next.cards.shuffle(&mut rng);
        next
    }
}

/// Stub for a Hold'em GameState.
#[derive(Clone)]
pub struct HoldemGameState {
    // Pot, board cards, hole cards, bets, etc.
}

impl GameState for HoldemGameState {
    fn current_player(&self) -> usize {
        unimplemented!()
    }
    fn information_set_key(&self) -> String {
        unimplemented!()
    }
    fn legal_actions(&self) -> Vec<usize> {
        unimplemented!()
    }
    fn is_terminal(&self) -> bool {
        unimplemented!()
    }
    fn terminal_utility(&self) -> Vec<f64> {
        unimplemented!()
    }
    fn act(&self, _action: usize) -> Self {
        unimplemented!()
    }
    fn sample_chance(&self) -> Self {
        unimplemented!()
    }
}

/// Toy 2-player push/fold game where showdown strength is precomputed.
///
/// This state is intentionally compact for solver/GPU plumbing:
/// - Player 0 acts first: fold or shove.
/// - If shove, Player 1 folds or calls.
/// - On call, winner is determined by precomputed Hold'em ranks.
#[derive(Clone)]
pub struct HoldemPushFoldState {
    pub p0_rank: u32,
    pub p1_rank: u32,
    pub history: Vec<usize>, // 0: fold/check, 1: shove/call
    pub current_player: usize,
}

impl HoldemPushFoldState {
    pub fn new(p0_rank: u32, p1_rank: u32) -> Self {
        Self {
            p0_rank,
            p1_rank,
            history: Vec::new(),
            current_player: 0,
        }
    }
}

impl GameState for HoldemPushFoldState {
    fn current_player(&self) -> usize {
        self.current_player
    }

    fn information_set_key(&self) -> String {
        let hist: String = self
            .history
            .iter()
            .map(|&a| if a == 0 { 'F' } else { 'S' })
            .collect();
        format!("pf:{}", hist)
    }

    fn legal_actions(&self) -> Vec<usize> {
        vec![0, 1]
    }

    fn is_terminal(&self) -> bool {
        self.history == vec![0] || self.history == vec![1, 0] || self.history == vec![1, 1]
    }

    fn terminal_utility(&self) -> Vec<f64> {
        // [0]      : p0 folds immediately (small blind loss)
        // [1,0]    : p1 folds to shove
        // [1,1]    : all-in showdown
        if self.history == vec![0] {
            return vec![-0.5, 0.5];
        }
        if self.history == vec![1, 0] {
            return vec![1.0, -1.0];
        }
        if self.history == vec![1, 1] {
            let p0 = HandVal {
                value: self.p0_rank,
            };
            let p1 = HandVal {
                value: self.p1_rank,
            };
            if p0 > p1 {
                vec![2.0, -2.0]
            } else if p1 > p0 {
                vec![-2.0, 2.0]
            } else {
                vec![0.0, 0.0]
            }
        } else {
            vec![0.0, 0.0]
        }
    }

    fn act(&self, action: usize) -> Self {
        let mut next = self.clone();
        next.history.push(action);
        next.current_player = 1 - self.current_player;
        next
    }

    fn sample_chance(&self) -> Self {
        // CPU fallback for standalone usage of this toy game.
        let mut rng = thread_rng();
        let all_cards: Vec<usize> = (0..52).collect();
        let mut indices = all_cards;
        indices.shuffle(&mut rng);

        let mut p0 = StdDeckCardMask::new();
        p0.set(indices[0]);
        p0.set(indices[1]);
        let mut p1 = StdDeckCardMask::new();
        p1.set(indices[2]);
        p1.set(indices[3]);
        let mut board = StdDeckCardMask::new();
        for idx in &indices[4..9] {
            board.set(*idx);
        }

        let p0_rank = crate::evaluators::HoldemEvaluator::evaluate_hand(&p0, &board)
            .map(|v| v.value)
            .unwrap_or(0);
        let p1_rank = crate::evaluators::HoldemEvaluator::evaluate_hand(&p1, &board)
            .map(|v| v.value)
            .unwrap_or(0);

        HoldemPushFoldState::new(p0_rank, p1_rank)
    }
}

/// Stub for a Stud GameState.
#[derive(Clone)]
pub struct StudGameState {}

impl GameState for StudGameState {
    fn current_player(&self) -> usize {
        unimplemented!()
    }
    fn information_set_key(&self) -> String {
        unimplemented!()
    }
    fn legal_actions(&self) -> Vec<usize> {
        unimplemented!()
    }
    fn is_terminal(&self) -> bool {
        unimplemented!()
    }
    fn terminal_utility(&self) -> Vec<f64> {
        unimplemented!()
    }
    fn act(&self, _action: usize) -> Self {
        unimplemented!()
    }
    fn sample_chance(&self) -> Self {
        unimplemented!()
    }
}

/// Stub for a Draw GameState.
#[derive(Clone)]
pub struct DrawGameState {}

impl GameState for DrawGameState {
    fn current_player(&self) -> usize {
        unimplemented!()
    }
    fn information_set_key(&self) -> String {
        unimplemented!()
    }
    fn legal_actions(&self) -> Vec<usize> {
        unimplemented!()
    }
    fn is_terminal(&self) -> bool {
        unimplemented!()
    }
    fn terminal_utility(&self) -> Vec<f64> {
        unimplemented!()
    }
    fn act(&self, _action: usize) -> Self {
        unimplemented!()
    }
    fn sample_chance(&self) -> Self {
        unimplemented!()
    }
}

fn normalize_probs(mut probs: [f64; 2]) -> [f64; 2] {
    probs[0] = probs[0].max(0.0);
    probs[1] = probs[1].max(0.0);
    let sum = probs[0] + probs[1];
    if sum <= 0.0 {
        [0.5, 0.5]
    } else {
        [probs[0] / sum, probs[1] / sum]
    }
}

fn kuhn_profile_value_for_player<F>(state: &KuhnGameState, player: usize, strategy: &F) -> f64
where
    F: Fn(&str) -> [f64; 2],
{
    if state.is_terminal() {
        return state.terminal_utility()[player];
    }

    let key = state.information_set_key();
    let probs = normalize_probs(strategy(&key));
    let s0 = state.act(0);
    let s1 = state.act(1);
    probs[0] * kuhn_profile_value_for_player(&s0, player, strategy)
        + probs[1] * kuhn_profile_value_for_player(&s1, player, strategy)
}

fn kuhn_best_response_value_for_player<F>(state: &KuhnGameState, player: usize, strategy: &F) -> f64
where
    F: Fn(&str) -> [f64; 2],
{
    if state.is_terminal() {
        return state.terminal_utility()[player];
    }

    if state.current_player() == player {
        let a0 = kuhn_best_response_value_for_player(&state.act(0), player, strategy);
        let a1 = kuhn_best_response_value_for_player(&state.act(1), player, strategy);
        a0.max(a1)
    } else {
        let key = state.information_set_key();
        let probs = normalize_probs(strategy(&key));
        let s0 = state.act(0);
        let s1 = state.act(1);
        probs[0] * kuhn_best_response_value_for_player(&s0, player, strategy)
            + probs[1] * kuhn_best_response_value_for_player(&s1, player, strategy)
    }
}

/// Computes Kuhn Poker NashConv for a provided strategy profile.
///
/// The strategy closure receives infoset keys like `"0:"`, `"2:B"` and must
/// return action probabilities `[pass/check, bet/call]`.
pub fn kuhn_nash_conv<F>(strategy: &F) -> f64
where
    F: Fn(&str) -> [f64; 2],
{
    let mut u0 = 0.0;
    let mut u1 = 0.0;
    let mut br0 = 0.0;
    let mut br1 = 0.0;
    let mut deals = 0.0;

    // Chance over ordered two-card deals from deck {J,Q,K}: 6 equally likely deals.
    for c0 in 0..3 {
        for c1 in 0..3 {
            if c0 == c1 {
                continue;
            }
            let state = KuhnGameState {
                cards: vec![c0, c1],
                history: Vec::new(),
                current_player: 0,
            };
            u0 += kuhn_profile_value_for_player(&state, 0, strategy);
            u1 += kuhn_profile_value_for_player(&state, 1, strategy);
            br0 += kuhn_best_response_value_for_player(&state, 0, strategy);
            br1 += kuhn_best_response_value_for_player(&state, 1, strategy);
            deals += 1.0;
        }
    }

    u0 /= deals;
    u1 /= deals;
    br0 /= deals;
    br1 /= deals;
    (br0 - u0) + (br1 - u1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kuhn_nash_conv_uniform_is_positive() {
        let uniform = |_: &str| [0.5, 0.5];
        let nash_conv = kuhn_nash_conv(&uniform);
        assert!(nash_conv > 0.0);
    }

    #[test]
    fn test_push_fold_terminal_utilities() {
        let mut s = HoldemPushFoldState::new(10, 20);
        s.history = vec![0];
        assert_eq!(s.terminal_utility(), vec![-0.5, 0.5]);

        let mut s = HoldemPushFoldState::new(10, 20);
        s.history = vec![1, 0];
        assert_eq!(s.terminal_utility(), vec![1.0, -1.0]);

        let mut s = HoldemPushFoldState::new(30, 20);
        s.history = vec![1, 1];
        assert_eq!(s.terminal_utility(), vec![2.0, -2.0]);
    }
}
