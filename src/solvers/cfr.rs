//! Counterfactual Regret Minimization (CFR) framework.
//!
//! This module provides the infrastructure for building and solving poker
//! games using CFR algorithms, specifically MCCFR (Monte Carlo CFR).

use crate::deck::StdDeckCardMask;
use crate::gpu::GPUEvaluator;
use crate::solvers::core::{
    cfr_plus_traverse, run_cfr_plus_iteration, GameTree, InfosetValues, NodeKind, TraversalCaches,
};
use crate::solvers::games::HoldemPushFoldState;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait representing a game state in a poker game tree.
pub trait GameState: Clone + Send + Sync {
    /// Returns which player is to act at this state.
    fn current_player(&self) -> usize;

    /// Returns a unique string or key representing the information set
    /// for the current player at this state.
    fn information_set_key(&self) -> String;

    /// Returns the legal actions at this state.
    fn legal_actions(&self) -> Vec<usize>;

    /// Returns true if the state is terminal.
    fn is_terminal(&self) -> bool;

    /// Returns the utility for each player if the state is terminal.
    fn terminal_utility(&self) -> Vec<f64>;

    /// Returns a new state after taking an action.
    fn act(&self, action: usize) -> Self;

    /// Returns a chance node outcome (e.g., card dealing).
    fn sample_chance(&self) -> Self;
}

/// Trait for infoset abstraction (bucketing).
pub trait Abstraction: Send + Sync {
    fn abstract_key(&self, key: &str) -> String;
}

/// No-op abstraction.
pub struct IdentityAbstraction;
impl Abstraction for IdentityAbstraction {
    fn abstract_key(&self, key: &str) -> String {
        key.to_string()
    }
}

/// A bucket-based abstraction that maps keys using a provided mapping or logic.
pub struct BucketAbstraction {
    pub mapper: Box<dyn Fn(&str) -> String + Send + Sync>,
}

impl BucketAbstraction {
    pub fn new<F>(mapper: F) -> Self
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        Self {
            mapper: Box::new(mapper),
        }
    }
}

impl Abstraction for BucketAbstraction {
    fn abstract_key(&self, key: &str) -> String {
        (self.mapper)(key)
    }
}

/// A node in the strategy tree storing regrets and cumulative strategy.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CFRNode {
    pub regrets: Vec<f64>,
    pub strategy_sum: Vec<f64>,
    pub num_actions: usize,
}

impl CFRNode {
    pub fn new(num_actions: usize) -> Self {
        Self {
            regrets: vec![0.0; num_actions],
            strategy_sum: vec![0.0; num_actions],
            num_actions,
        }
    }

    /// Applies DCFR discounting to regrets and strategy sum.
    pub fn apply_discount(&mut self, iteration: usize, alpha: f64, beta: f64, gamma: f64) {
        let t = iteration as f64;
        let pos_weight = (t.powf(alpha)) / (t.powf(alpha) + 1.0);
        let neg_weight = (t.powf(beta)) / (t.powf(beta) + 1.0);
        let strat_weight = (t / (t + 1.0)).powf(gamma);

        for i in 0..self.num_actions {
            if self.regrets[i] > 0.0 {
                self.regrets[i] *= pos_weight;
            } else {
                self.regrets[i] *= neg_weight;
            }
            self.strategy_sum[i] *= strat_weight;
        }
    }

    /// Returns the current strategy using regret matching or ECFR.
    pub fn get_strategy(&self, use_ecfr: bool) -> Vec<f64> {
        let mut strategy = vec![0.0; self.num_actions];
        let mut sum = 0.0;

        if use_ecfr {
            // Exponential CFR: x_i = exp(eta * R_i) / sum(exp(eta * R_j))
            // Using eta = 1.0 for now as a default
            let eta = 1.0;
            let max_regret = self
                .regrets
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max);
            for (i, strat_item) in strategy.iter_mut().enumerate().take(self.num_actions) {
                *strat_item = (eta * (self.regrets[i] - max_regret)).exp();
                sum += *strat_item;
            }
        } else {
            // Standard Regret Matching
            for (i, strat_item) in strategy.iter_mut().enumerate().take(self.num_actions) {
                *strat_item = self.regrets[i].max(0.0);
                sum += *strat_item;
            }
        }

        for strat_item in strategy.iter_mut().take(self.num_actions) {
            if sum > 0.0 {
                *strat_item /= sum;
            } else {
                *strat_item = 1.0 / (self.num_actions as f64);
            }
        }

        strategy
    }
}

/// The CFR Solver state.
#[derive(Serialize, Deserialize)]
pub struct CFRSolver {
    pub nodes: HashMap<String, CFRNode>,
    pub iteration: usize,
    pub num_players: usize,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub use_ecfr: bool,
    pub use_mccfvfp: bool,
    #[serde(default = "default_linear_avg_power")]
    pub linear_avg_power: f64,
    #[serde(skip)]
    #[serde(default = "default_abstraction")]
    pub abstraction: Box<dyn Abstraction>,
}

fn default_abstraction() -> Box<dyn Abstraction> {
    Box::new(IdentityAbstraction)
}

fn default_linear_avg_power() -> f64 {
    1.0
}

#[derive(Clone)]
struct LegacyDecisionState<'a, G: GameState> {
    inner: G,
    abstraction: &'a dyn Abstraction,
    num_players: usize,
}

impl<'a, G: GameState> LegacyDecisionState<'a, G> {
    fn new(inner: G, num_players: usize, abstraction: &'a dyn Abstraction) -> Self {
        Self {
            inner,
            abstraction,
            num_players,
        }
    }
}

impl<G: GameState> GameTree for LegacyDecisionState<'_, G> {
    fn num_players(&self) -> usize {
        self.num_players
    }

    fn node_kind(&self) -> NodeKind {
        if self.inner.is_terminal() {
            NodeKind::Terminal
        } else {
            NodeKind::Decision {
                player: self.inner.current_player(),
                infoset: self
                    .abstraction
                    .abstract_key(&self.inner.information_set_key()),
            }
        }
    }

    fn legal_actions(&self) -> Vec<usize> {
        self.inner.legal_actions()
    }

    fn apply_action(&self, action: usize) -> Self {
        Self::new(self.inner.act(action), self.num_players, self.abstraction)
    }

    fn chance_outcomes(&self) -> Vec<(f64, Self)> {
        Vec::new()
    }

    fn terminal_utility(&self) -> Vec<f64> {
        self.inner.terminal_utility()
    }
}

impl CFRSolver {
    pub fn new(num_players: usize) -> Self {
        Self {
            nodes: HashMap::new(),
            iteration: 0,
            num_players,
            alpha: 1.5,
            beta: 0.0,
            gamma: 2.0,
            use_ecfr: false,
            use_mccfvfp: true, // Default to true as it was emphasized
            linear_avg_power: default_linear_avg_power(),
            abstraction: Box::new(IdentityAbstraction),
        }
    }

    fn apply_discounts(&mut self) {
        let alpha = self.alpha;
        let beta = self.beta;
        let gamma = self.gamma;
        let iter = self.iteration;
        for node in self.nodes.values_mut() {
            node.apply_discount(iter, alpha, beta, gamma);
        }
    }

    /// Trains the solver for a number of iterations.
    pub fn train<G: GameState>(&mut self, initial_state: &G, iterations: usize) {
        let mut table = self.to_core_table();
        for _ in 0..iterations {
            self.iteration += 1;
            // Apply discounting to all nodes at the start of each iteration for DCFR
            self.apply_discounts();

            // Sample a new chance state for this iteration (e.g., deal new cards)
            let sampled_state = initial_state.sample_chance();
            let root = LegacyDecisionState::new(
                sampled_state,
                self.num_players,
                self.abstraction.as_ref(),
            );
            run_cfr_plus_iteration(
                &root,
                &mut table,
                self.iteration,
                self.linear_avg_power > 0.0,
                true,
                true,
                false,
            );
        }
        self.replace_from_core_table(table);
    }

    /// Trains CFR on a toy Hold'em push/fold game using GPU-resident terminal strength batches.
    ///
    /// This method is a first integration point between solver loops and GPU evaluation:
    /// it precomputes showdown strengths in GPU batches (resident dispatch + delayed readback),
    /// then runs CFR updates on compact game trees.
    pub fn train_push_fold_with_gpu(
        &mut self,
        iterations: usize,
        batch_size: usize,
        gpu: &mut GPUEvaluator,
    ) {
        if self.num_players != 2 || iterations == 0 {
            return;
        }

        let mut table = self.to_core_table();
        let batch_size = batch_size.max(1);
        let mut remaining = iterations;
        while remaining > 0 {
            let batch = remaining.min(batch_size);
            let (p0_masks, p1_masks) = generate_random_showdown_masks(batch);

            gpu.dispatch_batch_resident(&p0_masks);
            let p0_ranks = gpu.collect_resident_results().unwrap_or_default();
            gpu.dispatch_batch_resident(&p1_masks);
            let p1_ranks = gpu.collect_resident_results().unwrap_or_default();

            let count = p0_ranks.len().min(p1_ranks.len()).min(batch);
            for i in 0..count {
                self.iteration += 1;
                self.apply_discounts();
                let state = HoldemPushFoldState::new(p0_ranks[i], p1_ranks[i]);
                let root =
                    LegacyDecisionState::new(state, self.num_players, self.abstraction.as_ref());
                run_cfr_plus_iteration(
                    &root,
                    &mut table,
                    self.iteration,
                    self.linear_avg_power > 0.0,
                    true,
                    true,
                    false,
                );
            }

            if count == 0 {
                break;
            }
            remaining -= count;
        }
        self.replace_from_core_table(table);
    }

    /// Recursive CFR traversal.
    ///
    /// # Arguments
    /// * `state` - Current game state.
    /// * `player_idx` - The player we are currently updating regrets for.
    /// * `p_i` - Reach probability of the current player.
    /// * `p_o` - Reach probability of all other players.
    pub fn cfr<G: GameState>(&mut self, state: &G, player_idx: usize, p_i: f64, p_o: f64) -> f64 {
        let mut table = self.to_core_table();
        let root =
            LegacyDecisionState::new(state.clone(), self.num_players, self.abstraction.as_ref());
        let mut reach = vec![1.0; self.num_players];
        if player_idx < self.num_players {
            reach[player_idx] = p_i;
            for (idx, r) in reach.iter_mut().enumerate() {
                if idx != player_idx {
                    *r = p_o;
                }
            }
        }
        let util = cfr_plus_traverse(
            &root,
            player_idx,
            &mut reach,
            &mut table,
            self.iteration.max(1),
            self.linear_avg_power > 0.0,
            true,
            true,
            false,
            &mut TraversalCaches::default(),
        );
        self.replace_from_core_table(table);
        util
    }

    /// Returns the average strategy for an information set if it exists.
    pub fn average_strategy(&self, info_set_key: &str) -> Option<Vec<f64>> {
        let node = self.nodes.get(info_set_key)?;
        let mut avg = node.strategy_sum.clone();
        let total: f64 = avg.iter().sum();
        if total > 0.0 {
            for v in &mut avg {
                *v /= total;
            }
        } else if !avg.is_empty() {
            let uniform = 1.0 / (avg.len() as f64);
            avg.fill(uniform);
        }
        Some(avg)
    }

    /// Returns a strategy for an information set.
    ///
    /// Preference order:
    /// 1. Average strategy (if accumulated)
    /// 2. Current regret-matched strategy (if node exists)
    /// 3. Uniform fallback
    pub fn strategy_for_infoset(&self, info_set_key: &str) -> Vec<f64> {
        if let Some(avg) = self.average_strategy(info_set_key) {
            return avg;
        }
        if let Some(node) = self.nodes.get(info_set_key) {
            return node.get_strategy(self.use_ecfr);
        }
        // Fallback for unknown infosets: uniform over 2 actions (common in small games)
        // or we could potentially try to deduce N actions from context if we had it.
        vec![0.5, 0.5]
    }

    /// Saves the solver state to a file using Bincode.
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        bincode::serialize_into(writer, self)?;
        Ok(())
    }

    /// Loads the solver state from a file using Bincode.
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let mut solver: Self = bincode::deserialize_from(reader)?;
        // Abstraction is not serialized, so we reset it to IdentityAbstraction
        solver.abstraction = Box::new(IdentityAbstraction);
        Ok(solver)
    }

    fn to_core_table(&self) -> HashMap<String, InfosetValues> {
        self.nodes
            .iter()
            .map(|(k, n)| {
                (
                    k.clone(),
                    InfosetValues {
                        regrets: n.regrets.clone(),
                        strategy_sum: n.strategy_sum.clone(),
                    },
                )
            })
            .collect()
    }

    fn replace_from_core_table(&mut self, table: HashMap<String, InfosetValues>) {
        self.nodes = table
            .into_iter()
            .map(|(k, n)| {
                let num_actions = n.regrets.len().max(n.strategy_sum.len());
                (
                    k,
                    CFRNode {
                        regrets: n.regrets,
                        strategy_sum: n.strategy_sum,
                        num_actions,
                    },
                )
            })
            .collect();
    }
}

fn generate_random_showdown_masks(n: usize) -> (Vec<u64>, Vec<u64>) {
    let mut rng = thread_rng();
    let mut p0_masks = Vec::with_capacity(n);
    let mut p1_masks = Vec::with_capacity(n);
    let base_cards: Vec<usize> = (0..52).collect();

    for _ in 0..n {
        let mut indices = base_cards.clone();
        indices.shuffle(&mut rng);
        let mut p0 = StdDeckCardMask::new();
        let mut p1 = StdDeckCardMask::new();
        let mut board = StdDeckCardMask::new();

        p0.set(indices[0]);
        p0.set(indices[1]);
        p1.set(indices[2]);
        p1.set(indices[3]);
        for idx in &indices[4..9] {
            board.set(*idx);
        }

        p0_masks.push((p0 | board).as_raw());
        p1_masks.push((p1 | board).as_raw());
    }

    (p0_masks, p1_masks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::games::KuhnGameState;
    // ...

    #[test]
    fn test_cfr_kuhn_poker() {
        let mut solver = CFRSolver::new(2);
        let initial_cards = vec![0, 1, 2]; // J, Q, K

        let state = KuhnGameState::new(initial_cards);
        // DCFR training
        solver.train(&state, 1000);

        assert!(solver.nodes.len() >= 12);
        println!("Nodes trained (No-Bucket): {}", solver.nodes.len());

        if let Some(node) = solver.nodes.get("2:B") {
            let strategy = node.get_strategy(false);
            assert!(strategy[1] > 0.9);
        }
    }

    struct KuhnBucketAbstraction;
    impl Abstraction for KuhnBucketAbstraction {
        fn abstract_key(&self, key: &str) -> String {
            // Bucket J(0) and Q(1) together for the first part of the key
            if key.starts_with('0') || key.starts_with('1') {
                format!("JQ{}", &key[1..])
            } else {
                key.to_string()
            }
        }
    }

    #[test]
    fn test_cfr_kuhn_bucketing() {
        let mut solver = CFRSolver::new(2);
        solver.abstraction = Box::new(KuhnBucketAbstraction);
        let initial_cards = vec![0, 1, 2];

        let state = KuhnGameState::new(initial_cards);
        solver.train(&state, 1000);

        println!("Nodes trained (Bucketed): {}", solver.nodes.len());
        // Should have fewer nodes than 30 (which was the no-bucket count)
        assert!(solver.nodes.len() < 30);
    }

    #[test]
    fn test_cfr_checkpoint() {
        let mut solver = CFRSolver::new(2);
        let initial_cards = vec![0, 1, 2];
        let state = KuhnGameState::new(initial_cards);

        solver.train(&state, 100);
        let nodes_before = solver.nodes.len();
        let iter_before = solver.iteration;

        // Serialize
        let json = serde_json::to_string(&solver).unwrap();

        // Deserialize
        let mut solver_loaded: CFRSolver = serde_json::from_str(&json).unwrap();

        assert_eq!(solver_loaded.nodes.len(), nodes_before);
        assert_eq!(solver_loaded.iteration, iter_before);

        // Continue training
        solver_loaded.train(&state, 100);
        assert_eq!(solver_loaded.iteration, iter_before + 100);
    }

    #[test]
    fn test_cfr_ecfr() {
        let mut solver = CFRSolver::new(2);
        solver.use_ecfr = true;
        let initial_cards = vec![0, 1, 2];
        let state = KuhnGameState::new(initial_cards);

        solver.train(&state, 1000);
        assert!(solver.nodes.len() >= 12);

        if let Some(node) = solver.nodes.get("2:B") {
            let strategy = node.get_strategy(true);
            println!("ECFR Strategy for King after Bet: {:?}", strategy);
            assert!(strategy[1] > 0.8);
        }
    }

    #[test]
    fn test_average_strategy_normalized() {
        let mut solver = CFRSolver::new(2);
        let state = KuhnGameState::new(vec![0, 1, 2]);
        solver.train(&state, 500);

        let avg = solver
            .average_strategy("1:")
            .expect("average strategy for infoset 1: should exist");
        let sum: f64 = avg.iter().sum();
        assert!((sum - 1.0).abs() < 1e-9);
        assert!(avg.iter().all(|p| *p >= 0.0));
    }

    #[test]
    fn test_checkpoint_keeps_linear_avg_power() {
        let mut solver = CFRSolver::new(2);
        solver.linear_avg_power = 2.0;
        let json = serde_json::to_string(&solver).unwrap();
        let loaded: CFRSolver = serde_json::from_str(&json).unwrap();
        assert!((loaded.linear_avg_power - 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_kuhn_nash_conv_improves_with_training() {
        use crate::solvers::games::kuhn_nash_conv;

        let initial = CFRSolver::new(2);
        let initial_nc = kuhn_nash_conv(&|key| {
            let s = initial.strategy_for_infoset(key);
            [s[0], s[1]]
        });

        let mut trained = CFRSolver::new(2);
        let state = KuhnGameState::new(vec![0, 1, 2]);
        trained.train(&state, 5_000);
        let trained_nc = kuhn_nash_conv(&|key| {
            let s = trained.strategy_for_infoset(key);
            [s[0], s[1]]
        });

        assert!(
            trained_nc < initial_nc,
            "NashConv should improve with training: initial={initial_nc:.4}, trained={trained_nc:.4}"
        );
    }

    #[test]
    fn test_generate_random_showdown_masks_count() {
        let n = 16;
        let (a, b) = generate_random_showdown_masks(n);
        assert_eq!(a.len(), n);
        assert_eq!(b.len(), n);
    }

    #[test]
    fn test_cfr_bincode_persistence() {
        let mut solver = CFRSolver::new(2);
        let initial_cards = vec![0, 1, 2];
        let state = KuhnGameState::new(initial_cards);

        solver.train(&state, 100);
        let nodes_before = solver.nodes.len();

        let path = "test_solver.bin";
        solver.save(path).unwrap();

        let solver_loaded = CFRSolver::load(path).unwrap();
        assert_eq!(solver_loaded.nodes.len(), nodes_before);
        assert_eq!(solver_loaded.iteration, 100);

        std::fs::remove_file(path).unwrap();
    }
}
