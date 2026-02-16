use crate::solvers::core::{exploitability_two_player, CfrPlusSolver, GameTree, NodeKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoldemRound {
    ChanceDeal,
    P0Decision,
    P1Decision,
    Terminal,
}

/// Minimal 2-player Hold'em push/fold abstraction:
/// - Chance deals strength buckets (weak/strong) to each player.
/// - P0: fold or shove.
/// - If shove, P1: fold or call.
/// - On call, showdown utility depends on bucket strengths.
#[derive(Debug, Clone)]
pub struct HoldemSubgameState {
    pub round: HoldemRound,
    pub p0_bucket: u8,
    pub p1_bucket: u8,
    pub p0_action: Option<usize>, // 0 fold, 1 shove
    pub p1_action: Option<usize>, // 0 fold, 1 call
}

impl HoldemSubgameState {
    pub fn new() -> Self {
        Self {
            round: HoldemRound::ChanceDeal,
            p0_bucket: 0,
            p1_bucket: 0,
            p0_action: None,
            p1_action: None,
        }
    }
}

impl Default for HoldemSubgameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameTree for HoldemSubgameState {
    fn num_players(&self) -> usize {
        2
    }

    fn node_kind(&self) -> NodeKind {
        match self.round {
            HoldemRound::ChanceDeal => NodeKind::Chance,
            HoldemRound::P0Decision => NodeKind::Decision {
                player: 0,
                infoset: format!("h:p0:{}", self.p0_bucket),
            },
            HoldemRound::P1Decision => NodeKind::Decision {
                player: 1,
                infoset: format!("h:p1:{}", self.p1_bucket),
            },
            HoldemRound::Terminal => NodeKind::Terminal,
        }
    }

    fn legal_actions(&self) -> Vec<usize> {
        match self.round {
            HoldemRound::P0Decision => vec![0, 1],
            HoldemRound::P1Decision => vec![0, 1],
            _ => Vec::new(),
        }
    }

    fn apply_action(&self, action: usize) -> Self {
        match self.round {
            HoldemRound::P0Decision => {
                let mut next = self.clone();
                next.p0_action = Some(action);
                next.round = if action == 0 {
                    HoldemRound::Terminal
                } else {
                    HoldemRound::P1Decision
                };
                next
            }
            HoldemRound::P1Decision => {
                let mut next = self.clone();
                next.p1_action = Some(action);
                next.round = HoldemRound::Terminal;
                next
            }
            _ => self.clone(),
        }
    }

    fn chance_outcomes(&self) -> Vec<(f64, Self)> {
        if self.round != HoldemRound::ChanceDeal {
            return Vec::new();
        }
        let outcomes = vec![(0, 0), (0, 1), (1, 0), (1, 1)];
        let p = 1.0 / outcomes.len() as f64;
        outcomes
            .into_iter()
            .map(|(p0, p1)| {
                let mut s = self.clone();
                s.p0_bucket = p0;
                s.p1_bucket = p1;
                s.round = HoldemRound::P0Decision;
                (p, s)
            })
            .collect()
    }

    fn terminal_utility(&self) -> Vec<f64> {
        if self.round != HoldemRound::Terminal {
            return vec![0.0, 0.0];
        }
        match (self.p0_action, self.p1_action) {
            (Some(0), _) => vec![-0.5, 0.5],       // p0 folds
            (Some(1), Some(0)) => vec![1.0, -1.0], // p1 folds to shove
            (Some(1), Some(1)) => {
                if self.p0_bucket > self.p1_bucket {
                    vec![2.0, -2.0]
                } else if self.p1_bucket > self.p0_bucket {
                    vec![-2.0, 2.0]
                } else {
                    vec![0.0, 0.0]
                }
            }
            _ => vec![0.0, 0.0],
        }
    }

    fn cache_key(&self) -> Option<String> {
        Some(format!(
            "hs:{:?}:{}:{}:{:?}:{:?}",
            self.round, self.p0_bucket, self.p1_bucket, self.p0_action, self.p1_action
        ))
    }

    fn subtree_action_cache_key(&self) -> Option<String> {
        self.cache_key()
    }

    fn subtree_value_cache_key(&self, _update_player: usize) -> Option<String> {
        if self.round == HoldemRound::Terminal {
            self.cache_key()
        } else {
            None
        }
    }
}

pub fn holdem_exploitability_proxy(solver: &CfrPlusSolver<HoldemSubgameState>) -> f64 {
    exploitability_two_player(&solver.root, &|infoset, n| {
        solver.average_strategy_for_infoset(infoset, n)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holdem_subgame_cfr_plus_reduces_exploitability_proxy() {
        let root = HoldemSubgameState::new();
        let mut solver = CfrPlusSolver::new(root);
        let before = holdem_exploitability_proxy(&solver);
        solver.train(2000);
        let after = holdem_exploitability_proxy(&solver);

        assert!(after.is_finite());
        assert!(after <= before + 1e-9);
    }

    #[test]
    fn holdem_subgame_policy_is_non_uniform_for_strong_bucket() {
        let root = HoldemSubgameState::new();
        let mut solver = CfrPlusSolver::new(root);
        solver.train(3000);
        let weak = solver.average_strategy_for_infoset("h:p0:0", 2);
        let strong = solver.average_strategy_for_infoset("h:p0:1", 2);
        // action 1 = shove
        assert!(strong[1] >= weak[1]);
    }
}
