#[cfg(feature = "parallel")]
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Chance,
    Decision { player: usize, infoset: String },
    Terminal,
}

/// Minimal game-tree API for solver-core.
pub trait GameTree: Clone + Send + Sync {
    fn num_players(&self) -> usize;
    fn node_kind(&self) -> NodeKind;
    fn legal_actions(&self) -> Vec<usize>;
    fn apply_action(&self, action: usize) -> Self;
    fn chance_outcomes(&self) -> Vec<(f64, Self)>;
    fn terminal_utility(&self) -> Vec<f64>;
    fn cache_key(&self) -> Option<String> {
        None
    }
    fn subtree_action_cache_key(&self) -> Option<String> {
        self.cache_key()
    }
    fn subtree_value_cache_key(&self, _update_player: usize) -> Option<String> {
        None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InfosetValues {
    pub regrets: Vec<f64>,
    pub strategy_sum: Vec<f64>,
}

impl InfosetValues {
    pub fn new(num_actions: usize) -> Self {
        Self {
            regrets: vec![0.0; num_actions],
            strategy_sum: vec![0.0; num_actions],
        }
    }

    pub fn current_strategy(&self) -> Vec<f64> {
        let mut strategy = vec![0.0; self.regrets.len()];
        let mut sum = 0.0;
        for (i, s) in strategy.iter_mut().enumerate() {
            *s = self.regrets[i].max(0.0);
            sum += *s;
        }
        if sum > 0.0 {
            for s in &mut strategy {
                *s /= sum;
            }
        } else if !strategy.is_empty() {
            let u = 1.0 / strategy.len() as f64;
            strategy.fill(u);
        }
        strategy
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExploitabilityPoint {
    pub iteration: usize,
    pub exploitability: f64,
}

#[derive(Debug, Clone)]
pub struct CfrPlusConfig {
    pub linear_averaging: bool,
    pub cache_opponent_strategies: bool,
    pub cache_subtree_actions: bool,
    pub cache_subtree_values: bool,
}

impl Default for CfrPlusConfig {
    fn default() -> Self {
        Self {
            linear_averaging: true,
            cache_opponent_strategies: true,
            cache_subtree_actions: true,
            cache_subtree_values: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CfrPlusSolver<G: GameTree> {
    pub root: G,
    pub iteration: usize,
    pub table: HashMap<String, InfosetValues>,
    pub config: CfrPlusConfig,
}

pub(crate) struct TraversalCaches<G: GameTree> {
    pub strategy_cache: HashMap<String, Vec<f64>>,
    pub action_cache: HashMap<(String, usize), G>,
    pub chance_cache: HashMap<String, Vec<(f64, G)>>,
    pub value_cache: HashMap<(usize, String), f64>,
}

impl<G: GameTree> Default for TraversalCaches<G> {
    fn default() -> Self {
        Self {
            strategy_cache: HashMap::new(),
            action_cache: HashMap::new(),
            chance_cache: HashMap::new(),
            value_cache: HashMap::new(),
        }
    }
}

impl<G: GameTree> CfrPlusSolver<G> {
    pub fn new(root: G) -> Self {
        Self {
            root,
            iteration: 0,
            table: HashMap::new(),
            config: CfrPlusConfig::default(),
        }
    }

    fn average_profile_cache(&self) -> HashMap<String, Vec<f64>> {
        let mut out = HashMap::with_capacity(self.table.len());
        for (infoset, node) in &self.table {
            let n = node.regrets.len().max(node.strategy_sum.len());
            if n == 0 {
                out.insert(infoset.clone(), Vec::new());
                continue;
            }
            let mut s = if !node.strategy_sum.is_empty() {
                node.strategy_sum.clone()
            } else {
                node.current_strategy()
            };
            if s.len() != n {
                s.resize(n, 0.0);
            }
            let sum: f64 = s.iter().sum();
            if sum > 0.0 {
                for v in &mut s {
                    *v /= sum;
                }
            } else {
                s.fill(1.0 / n as f64);
            }
            out.insert(infoset.clone(), s);
        }
        out
    }

    pub fn train(&mut self, iterations: usize) {
        if iterations == 0 {
            return;
        }
        for _ in 0..iterations {
            self.iteration += 1;
            run_cfr_plus_iteration(
                &self.root,
                &mut self.table,
                self.iteration,
                self.config.linear_averaging,
                self.config.cache_opponent_strategies,
                self.config.cache_subtree_actions,
                self.config.cache_subtree_values,
            );
        }
    }

    pub fn average_strategy_for_infoset(&self, infoset: &str, n_actions: usize) -> Vec<f64> {
        if let Some(node) = self.table.get(infoset) {
            let total: f64 = node.strategy_sum.iter().sum();
            if total > 0.0 {
                return node.strategy_sum.iter().map(|v| v / total).collect();
            }
            return node.current_strategy();
        }
        if n_actions == 0 {
            return Vec::new();
        }
        vec![1.0 / n_actions as f64; n_actions]
    }

    /// Train and capture exploitability checkpoints for 2-player games.
    /// Returns an empty vector for games with player count different from 2.
    pub fn train_with_exploitability(
        &mut self,
        iterations: usize,
        checkpoint_every: usize,
    ) -> Vec<ExploitabilityPoint> {
        if iterations == 0 || checkpoint_every == 0 || self.root.num_players() != 2 {
            return Vec::new();
        }

        let mut out = Vec::new();
        for step in 0..iterations {
            self.train(1);
            let iter = step + 1;
            if iter % checkpoint_every == 0 || iter == iterations {
                let profile = self.average_profile_cache();
                let exp = exploitability_two_player(&self.root, &|infoset, n| {
                    profile
                        .get(infoset)
                        .cloned()
                        .unwrap_or_else(|| vec![1.0 / n.max(1) as f64; n.max(1)])
                });
                out.push(ExploitabilityPoint {
                    iteration: self.iteration,
                    exploitability: exp,
                });
            }
        }
        out
    }

    /// Train and capture exploitability checkpoints for n-player games.
    /// Metric is `sum_i(BR_i - U_i)` against the current average profile.
    pub fn train_with_n_player_exploitability(
        &mut self,
        iterations: usize,
        checkpoint_every: usize,
    ) -> Vec<ExploitabilityPoint> {
        if iterations == 0 || checkpoint_every == 0 || self.root.num_players() == 0 {
            return Vec::new();
        }
        let mut out = Vec::new();
        for step in 0..iterations {
            self.train(1);
            let iter = step + 1;
            if iter % checkpoint_every == 0 || iter == iterations {
                let profile = self.average_profile_cache();
                let exp = exploitability_n_player(&self.root, &|infoset, n| {
                    profile
                        .get(infoset)
                        .cloned()
                        .unwrap_or_else(|| vec![1.0 / n.max(1) as f64; n.max(1)])
                });
                out.push(ExploitabilityPoint {
                    iteration: self.iteration,
                    exploitability: exp,
                });
            }
        }
        out
    }
}

pub(crate) fn run_cfr_plus_iteration<G: GameTree>(
    root: &G,
    table: &mut HashMap<String, InfosetValues>,
    iteration: usize,
    linear_averaging: bool,
    cache_opponent_strategies: bool,
    cache_subtree_actions: bool,
    cache_subtree_values: bool,
) {
    let nplayers = root.num_players();
    for player in 0..nplayers {
        let mut reach = vec![1.0; nplayers];
        let mut caches = TraversalCaches::<G>::default();
        let _ = cfr_plus_traverse(
            root,
            player,
            &mut reach,
            table,
            iteration,
            linear_averaging,
            cache_opponent_strategies,
            cache_subtree_actions,
            cache_subtree_values,
            &mut caches,
        );
    }
}

pub(crate) fn cfr_plus_traverse<G: GameTree>(
    state: &G,
    update_player: usize,
    reach: &mut [f64],
    table: &mut HashMap<String, InfosetValues>,
    iteration: usize,
    linear_averaging: bool,
    cache_opponent_strategies: bool,
    cache_subtree_actions: bool,
    cache_subtree_values: bool,
    caches: &mut TraversalCaches<G>,
) -> f64 {
    if cache_subtree_values {
        if let Some(k) = state.subtree_value_cache_key(update_player) {
            if let Some(v) = caches.value_cache.get(&(update_player, k.clone())) {
                return *v;
            }
        }
    }

    match state.node_kind() {
        NodeKind::Terminal => state.terminal_utility()[update_player],
        NodeKind::Chance => {
            let mut ev = 0.0;
            let outcomes = if cache_subtree_actions {
                if let Some(k) = state.subtree_action_cache_key() {
                    if let Some(v) = caches.chance_cache.get(&k) {
                        v.clone()
                    } else {
                        let v = state.chance_outcomes();
                        caches.chance_cache.insert(k, v.clone());
                        v
                    }
                } else {
                    state.chance_outcomes()
                }
            } else {
                state.chance_outcomes()
            };
            for (p, child) in outcomes {
                ev += p * cfr_plus_traverse(
                    &child,
                    update_player,
                    reach,
                    table,
                    iteration,
                    linear_averaging,
                    cache_opponent_strategies,
                    cache_subtree_actions,
                    cache_subtree_values,
                    caches,
                );
            }
            if cache_subtree_values {
                if let Some(k) = state.subtree_value_cache_key(update_player) {
                    caches.value_cache.insert((update_player, k), ev);
                }
            }
            ev
        }
        NodeKind::Decision { player, infoset } => {
            let actions = state.legal_actions();
            let node = table
                .entry(infoset.clone())
                .or_insert_with(|| InfosetValues::new(actions.len()));
            if node.regrets.len() != actions.len() {
                node.regrets.resize(actions.len(), 0.0);
                node.strategy_sum.resize(actions.len(), 0.0);
            }
            let strategy = if cache_opponent_strategies && player != update_player {
                if let Some(cached) = caches.strategy_cache.get(&infoset) {
                    if cached.len() == actions.len() {
                        cached.clone()
                    } else {
                        let s = node.current_strategy();
                        caches.strategy_cache.insert(infoset.clone(), s.clone());
                        s
                    }
                } else {
                    let s = node.current_strategy();
                    caches.strategy_cache.insert(infoset.clone(), s.clone());
                    s
                }
            } else {
                node.current_strategy()
            };

            let mut action_utils = vec![0.0; actions.len()];
            let mut node_util = 0.0;

            for (i, action) in actions.iter().enumerate() {
                let prev = reach[player];
                reach[player] *= strategy[i];
                let child = if cache_subtree_actions {
                    if let Some(base) = state.subtree_action_cache_key() {
                        let key = (base, *action);
                        if let Some(c) = caches.action_cache.get(&key) {
                            c.clone()
                        } else {
                            let c = state.apply_action(*action);
                            caches.action_cache.insert(key, c.clone());
                            c
                        }
                    } else {
                        state.apply_action(*action)
                    }
                } else {
                    state.apply_action(*action)
                };
                action_utils[i] = cfr_plus_traverse(
                    &child,
                    update_player,
                    reach,
                    table,
                    iteration,
                    linear_averaging,
                    cache_opponent_strategies,
                    cache_subtree_actions,
                    cache_subtree_values,
                    caches,
                );
                reach[player] = prev;
                node_util += strategy[i] * action_utils[i];
            }

            let avg_weight = if linear_averaging {
                iteration as f64
            } else {
                1.0
            };
            let node = table.get_mut(&infoset).expect("infoset exists");
            for (i, s) in strategy.iter().enumerate() {
                node.strategy_sum[i] += avg_weight * reach[player] * s;
            }

            if player == update_player {
                let mut cf_reach = 1.0;
                for (p, r) in reach.iter().enumerate() {
                    if p != player {
                        cf_reach *= *r;
                    }
                }
                for (i, action_util) in action_utils.iter().enumerate().take(actions.len()) {
                    let regret = cf_reach * (*action_util - node_util);
                    node.regrets[i] = (node.regrets[i] + regret).max(0.0);
                }
            }

            if cache_subtree_values {
                if let Some(k) = state.subtree_value_cache_key(update_player) {
                    caches.value_cache.insert((update_player, k), node_util);
                }
            }
            node_util
        }
    }
}

fn normalize_strategy(mut s: Vec<f64>, n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    if s.len() != n {
        return vec![1.0 / n as f64; n];
    }
    for v in &mut s {
        *v = v.max(0.0);
    }
    let sum: f64 = s.iter().sum();
    if sum <= 0.0 {
        vec![1.0 / n as f64; n]
    } else {
        s.into_iter().map(|v| v / sum).collect()
    }
}

pub fn expected_utility<G, F>(state: &G, policy: &F) -> Vec<f64>
where
    G: GameTree,
    F: Fn(&str, usize) -> Vec<f64>,
{
    let mut memo = HashMap::<String, Vec<f64>>::new();
    expected_utility_memo(state, policy, &mut memo)
}

fn expected_utility_memo<G, F>(
    state: &G,
    policy: &F,
    memo: &mut HashMap<String, Vec<f64>>,
) -> Vec<f64>
where
    G: GameTree,
    F: Fn(&str, usize) -> Vec<f64>,
{
    if let Some(k) = state.cache_key() {
        if let Some(v) = memo.get(&k) {
            return v.clone();
        }
    }
    let out = match state.node_kind() {
        NodeKind::Terminal => state.terminal_utility(),
        NodeKind::Chance => {
            let n = state.num_players();
            let mut total = vec![0.0; n];
            for (p, child) in state.chance_outcomes() {
                let u = expected_utility_memo(&child, policy, memo);
                for i in 0..n {
                    total[i] += p * u[i];
                }
            }
            total
        }
        NodeKind::Decision { infoset, .. } => {
            let actions = state.legal_actions();
            let strategy = normalize_strategy(policy(&infoset, actions.len()), actions.len());
            let n = state.num_players();
            let mut total = vec![0.0; n];
            for (i, action) in actions.iter().enumerate() {
                let child = state.apply_action(*action);
                let u = expected_utility_memo(&child, policy, memo);
                for p in 0..n {
                    total[p] += strategy[i] * u[p];
                }
            }
            total
        }
    };
    if let Some(k) = state.cache_key() {
        memo.insert(k, out.clone());
    }
    out
}

pub fn best_response_utility<G, F>(state: &G, br_player: usize, policy: &F) -> f64
where
    G: GameTree,
    F: Fn(&str, usize) -> Vec<f64>,
{
    let mut memo = HashMap::<(usize, String), f64>::new();
    best_response_utility_memo(state, br_player, policy, &mut memo)
}

fn best_response_utility_memo<G, F>(
    state: &G,
    br_player: usize,
    policy: &F,
    memo: &mut HashMap<(usize, String), f64>,
) -> f64
where
    G: GameTree,
    F: Fn(&str, usize) -> Vec<f64>,
{
    if let Some(k) = state.cache_key() {
        if let Some(v) = memo.get(&(br_player, k.clone())) {
            return *v;
        }
    }
    let out = match state.node_kind() {
        NodeKind::Terminal => state.terminal_utility()[br_player],
        NodeKind::Chance => state
            .chance_outcomes()
            .into_iter()
            .map(|(p, child)| p * best_response_utility_memo(&child, br_player, policy, memo))
            .sum(),
        NodeKind::Decision { player, infoset } => {
            let actions = state.legal_actions();
            if player == br_player {
                actions
                    .into_iter()
                    .map(|a| {
                        best_response_utility_memo(&state.apply_action(a), br_player, policy, memo)
                    })
                    .fold(f64::NEG_INFINITY, f64::max)
            } else {
                let strategy = normalize_strategy(policy(&infoset, actions.len()), actions.len());
                actions
                    .into_iter()
                    .enumerate()
                    .map(|(i, a)| {
                        strategy[i]
                            * best_response_utility_memo(
                                &state.apply_action(a),
                                br_player,
                                policy,
                                memo,
                            )
                    })
                    .sum()
            }
        }
    };
    if let Some(k) = state.cache_key() {
        memo.insert((br_player, k), out);
    }
    out
}

pub fn exploitability_two_player<G, F>(state: &G, policy: &F) -> f64
where
    G: GameTree,
    F: Fn(&str, usize) -> Vec<f64>,
{
    let u = expected_utility(state, policy);
    let br0 = best_response_utility(state, 0, policy);
    let br1 = best_response_utility(state, 1, policy);
    (br0 - u[0]) + (br1 - u[1])
}

pub fn exploitability_n_player<G, F>(state: &G, policy: &F) -> f64
where
    G: GameTree,
    F: Fn(&str, usize) -> Vec<f64> + Sync,
{
    let u = expected_utility(state, policy);
    #[cfg(feature = "parallel")]
    {
        (0..state.num_players())
            .into_par_iter()
            .map(|i| best_response_utility(state, i, policy) - u[i])
            .sum()
    }
    #[cfg(not(feature = "parallel"))]
    {
        let mut sum = 0.0;
        for (i, ui) in u.iter().enumerate().take(state.num_players()) {
            let br = best_response_utility(state, i, policy);
            sum += br - *ui;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct ToyTerminal;
    impl GameTree for ToyTerminal {
        fn num_players(&self) -> usize {
            2
        }
        fn node_kind(&self) -> NodeKind {
            NodeKind::Terminal
        }
        fn legal_actions(&self) -> Vec<usize> {
            vec![]
        }
        fn apply_action(&self, _action: usize) -> Self {
            self.clone()
        }
        fn chance_outcomes(&self) -> Vec<(f64, Self)> {
            vec![]
        }
        fn terminal_utility(&self) -> Vec<f64> {
            vec![1.0, -1.0]
        }
    }

    #[test]
    fn strategy_fallback_uniform() {
        let n = InfosetValues::new(2);
        let s = n.current_strategy();
        assert_eq!(s.len(), 2);
        assert!((s[0] - 0.5).abs() < 1e-12);
    }

    #[test]
    fn utility_terminal_works() {
        let s = ToyTerminal;
        let u = expected_utility(&s, &|_, _| vec![]);
        assert_eq!(u, vec![1.0, -1.0]);
    }

    #[test]
    fn exploitability_checkpoints_terminal_game() {
        let mut solver = CfrPlusSolver::new(ToyTerminal);
        let points = solver.train_with_exploitability(5, 2);
        assert_eq!(points.len(), 3);
        assert_eq!(points[0].iteration, 2);
        assert_eq!(points[1].iteration, 4);
        assert_eq!(points[2].iteration, 5);
        for p in points {
            assert!(p.exploitability.is_finite());
            assert!(p.exploitability.abs() < 1e-12);
        }
    }

    #[test]
    fn n_player_exploitability_matches_two_player_on_two_player_game() {
        let s = ToyTerminal;
        let p = |_infoset: &str, _n: usize| Vec::<f64>::new();
        let e2 = exploitability_two_player(&s, &p);
        let en = exploitability_n_player(&s, &p);
        assert!((e2 - en).abs() < 1e-12);
    }
}
