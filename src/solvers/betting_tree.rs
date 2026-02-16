use crate::solvers::betting::{Action, GameConfig, HandState, PlayerStatus};
use crate::solvers::core::{GameTree, NodeKind};

const ACT_FOLD: usize = 0;
const ACT_CHECK: usize = 1;
const ACT_CALL: usize = 2;
const ACT_BET_MIN: usize = 3;
const ACT_RAISE_MIN: usize = 4;
const ACT_ALL_IN: usize = 5;

#[derive(Debug, Clone)]
pub struct BettingGameTree {
    pub state: HandState,
    pub showdown_strengths: Vec<u32>,
}

impl BettingGameTree {
    pub fn new(config: GameConfig, dealer: usize, showdown_strengths: Vec<u32>) -> Self {
        let state = HandState::new(config, dealer);
        Self {
            state,
            showdown_strengths,
        }
    }

    fn encode_action(a: Action) -> usize {
        match a {
            Action::Fold => ACT_FOLD,
            Action::Check => ACT_CHECK,
            Action::Call => ACT_CALL,
            Action::Bet(_) => ACT_BET_MIN,
            Action::RaiseTo(_) => ACT_RAISE_MIN,
            Action::AllIn => ACT_ALL_IN,
        }
    }

    fn decode_action(&self, a: usize) -> Result<Action, String> {
        let legal = self.state.legal_actions();
        match a {
            ACT_FOLD => Ok(Action::Fold),
            ACT_CHECK => Ok(Action::Check),
            ACT_CALL => Ok(Action::Call),
            ACT_BET_MIN => legal
                .into_iter()
                .find_map(|x| match x {
                    Action::Bet(v) => Some(Action::Bet(v)),
                    _ => None,
                })
                .ok_or_else(|| "no legal min-bet action".to_string()),
            ACT_RAISE_MIN => legal
                .into_iter()
                .find_map(|x| match x {
                    Action::RaiseTo(v) => Some(Action::RaiseTo(v)),
                    _ => None,
                })
                .ok_or_else(|| "no legal min-raise action".to_string()),
            ACT_ALL_IN => Ok(Action::AllIn),
            _ => Err("unknown action id".to_string()),
        }
    }

    fn infoset_key(&self) -> String {
        let p = self.state.to_act;
        let my_strength = self.showdown_strengths.get(p).copied().unwrap_or(0);
        let facing = self.state.facing_amount(p);
        format!(
            "bet:p{}:str{}:st{:?}:cb{}:f{}:cr{:?}:stacks{:?}",
            p,
            my_strength,
            self.state.street,
            self.state.current_bet,
            facing,
            self.state.committed_round,
            self.state.stacks
        )
    }

    fn state_key(&self) -> String {
        format!(
            "bt:st{:?}:to{}:cb{}:cr{:?}:ct{:?}:s{:?}:st{:?}:term{}",
            self.state.street,
            self.state.to_act,
            self.state.current_bet,
            self.state.committed_round,
            self.state.committed_total,
            self.state.status,
            self.state.stacks,
            self.state.terminal
        )
    }

    fn terminal_chip_utility(&self) -> Vec<f64> {
        let n = self.state.config.num_players;
        let mut payouts = vec![0_u64; n];
        let pot = self.state.committed_total.iter().sum::<u64>();
        let contenders = self
            .state
            .status
            .iter()
            .enumerate()
            .filter_map(|(i, st)| {
                if *st != PlayerStatus::Folded {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if contenders.is_empty() {
            return vec![0.0; n];
        }

        if contenders.len() == 1 {
            payouts[contenders[0]] = pot;
        } else {
            let max_s = contenders
                .iter()
                .map(|i| self.showdown_strengths.get(*i).copied().unwrap_or(0))
                .max()
                .unwrap_or(0);
            let winners = contenders
                .iter()
                .copied()
                .filter(|i| self.showdown_strengths.get(*i).copied().unwrap_or(0) == max_s)
                .collect::<Vec<_>>();
            let share = pot / winners.len() as u64;
            let rem = pot % winners.len() as u64;
            for (k, i) in winners.iter().enumerate() {
                payouts[*i] += share + if k == 0 { rem } else { 0 };
            }
        }

        (0..n)
            .map(|i| payouts[i] as f64 - self.state.committed_total[i] as f64)
            .collect()
    }
}

impl GameTree for BettingGameTree {
    fn num_players(&self) -> usize {
        self.state.config.num_players
    }

    fn node_kind(&self) -> NodeKind {
        if self.state.terminal {
            NodeKind::Terminal
        } else {
            NodeKind::Decision {
                player: self.state.to_act,
                infoset: self.infoset_key(),
            }
        }
    }

    fn legal_actions(&self) -> Vec<usize> {
        self.state
            .legal_actions()
            .into_iter()
            .map(Self::encode_action)
            .collect()
    }

    fn apply_action(&self, action: usize) -> Self {
        let mut next = self.clone();
        if let Ok(a) = next.decode_action(action) {
            let _ = next.state.apply_action(a);
        }
        next
    }

    fn chance_outcomes(&self) -> Vec<(f64, Self)> {
        Vec::new()
    }

    fn terminal_utility(&self) -> Vec<f64> {
        if self.state.terminal {
            self.terminal_chip_utility()
        } else {
            vec![0.0; self.num_players()]
        }
    }

    fn cache_key(&self) -> Option<String> {
        Some(self.state_key())
    }

    fn subtree_action_cache_key(&self) -> Option<String> {
        self.cache_key()
    }

    fn subtree_value_cache_key(&self, _update_player: usize) -> Option<String> {
        if self.state.terminal {
            self.cache_key()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::core::CfrPlusSolver;

    #[test]
    fn terminal_utility_is_zero_sum() {
        let cfg = GameConfig::no_limit(3, 100, 1, 2);
        let mut g = BettingGameTree::new(cfg, 0, vec![2, 1, 0]);
        g.state.terminal = true;
        g.state.status = vec![
            PlayerStatus::Active,
            PlayerStatus::Active,
            PlayerStatus::Folded,
        ];
        g.state.committed_total = vec![10, 10, 0];
        let u = g.terminal_utility();
        let s: f64 = u.iter().sum();
        assert!(s.abs() < 1e-9);
    }

    #[test]
    fn cfr_plus_runs_on_three_player_betting_tree() {
        let cfg = GameConfig::no_limit(3, 6, 1, 2);
        let root = BettingGameTree::new(cfg, 0, vec![3, 2, 1]);
        let mut solver = CfrPlusSolver::new(root);
        solver.train(5);
        assert_eq!(solver.iteration, 5);
        assert!(!solver.table.is_empty());
    }
}
