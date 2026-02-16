use crate::solvers::core::{exploitability_two_player, CfrPlusSolver, GameTree, NodeKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PokerFamily {
    Holdem,
    Stud,
    Draw,
}

#[derive(Debug, Clone, Copy)]
struct FamilyPayoff {
    p0_fold: f64,
    p1_fold_to_aggr: f64,
    showdown_win: f64,
}

impl PokerFamily {
    fn payoff(self) -> FamilyPayoff {
        match self {
            PokerFamily::Holdem => FamilyPayoff {
                p0_fold: -0.5,
                p1_fold_to_aggr: 1.0,
                showdown_win: 2.0,
            },
            PokerFamily::Stud => FamilyPayoff {
                p0_fold: -0.5,
                p1_fold_to_aggr: 1.0,
                showdown_win: 2.5,
            },
            PokerFamily::Draw => FamilyPayoff {
                p0_fold: -0.4,
                p1_fold_to_aggr: 0.9,
                showdown_win: 1.8,
            },
        }
    }

    fn key_prefix(self) -> &'static str {
        match self {
            PokerFamily::Holdem => "fh",
            PokerFamily::Stud => "fs",
            PokerFamily::Draw => "fd",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FamilyRound {
    ChanceDeal,
    StudBringIn,
    P0Decision,
    P1Decision,
    ChanceTransition,
    DrawDiscardP0,
    DrawDiscardP1,
    DrawResolveChance,
    P0FinalDecision,
    P1FinalDecision,
    Terminal,
}

#[derive(Debug, Clone)]
pub struct FamilySubgameState {
    pub family: PokerFamily,
    round: FamilyRound,
    p0_bucket: u8,
    p1_bucket: u8,
    p0_bucket_final: u8,
    p1_bucket_final: u8,
    bring_in_player: usize,
    bring_in_action: Option<usize>, // 0 complete small, 1 complete big
    p0_action: Option<usize>,       // 0 fold/pass, 1 bet/push
    p1_action: Option<usize>,       // 0 fold, 1 continue
    p0_draw_action: Option<usize>,  // 0 stand pat, 1 draw one, 2 draw two
    p1_draw_action: Option<usize>,  // 0 stand pat, 1 draw one, 2 draw two
    p0_final_action: Option<usize>, // 0 check, 1 bet
    p1_final_action: Option<usize>, // 0 check/fold, 1 call/bet
}

impl FamilySubgameState {
    pub fn new(family: PokerFamily) -> Self {
        Self {
            family,
            round: FamilyRound::ChanceDeal,
            p0_bucket: 0,
            p1_bucket: 0,
            p0_bucket_final: 0,
            p1_bucket_final: 0,
            bring_in_player: 0,
            bring_in_action: None,
            p0_action: None,
            p1_action: None,
            p0_draw_action: None,
            p1_draw_action: None,
            p0_final_action: None,
            p1_final_action: None,
        }
    }

    fn supports_final_round(&self) -> bool {
        matches!(self.family, PokerFamily::Stud | PokerFamily::Draw)
    }

    fn advance_after_p1_decision(&mut self) {
        if self.supports_final_round() && self.p0_action == Some(1) && self.p1_action == Some(1) {
            self.round = FamilyRound::ChanceTransition;
        } else {
            self.round = FamilyRound::Terminal;
        }
    }

    fn showdown_buckets(&self) -> (u8, u8) {
        if self.supports_final_round() && self.p0_action == Some(1) && self.p1_action == Some(1) {
            (self.p0_bucket_final, self.p1_bucket_final)
        } else {
            (self.p0_bucket, self.p1_bucket)
        }
    }
}

impl GameTree for FamilySubgameState {
    fn num_players(&self) -> usize {
        2
    }

    fn node_kind(&self) -> NodeKind {
        match self.round {
            FamilyRound::ChanceDeal => NodeKind::Chance,
            FamilyRound::StudBringIn => NodeKind::Decision {
                player: self.bring_in_player,
                infoset: format!(
                    "{}:bi:{}:{}",
                    self.family.key_prefix(),
                    self.p0_bucket,
                    self.p1_bucket
                ),
            },
            FamilyRound::P0Decision => NodeKind::Decision {
                player: 0,
                infoset: format!("{}:p0:{}", self.family.key_prefix(), self.p0_bucket),
            },
            FamilyRound::P1Decision => NodeKind::Decision {
                player: 1,
                infoset: format!("{}:p1:{}", self.family.key_prefix(), self.p1_bucket),
            },
            FamilyRound::ChanceTransition => NodeKind::Chance,
            FamilyRound::DrawDiscardP0 => NodeKind::Decision {
                player: 0,
                infoset: format!(
                    "{}:d0:{}:{}",
                    self.family.key_prefix(),
                    self.p0_bucket,
                    self.p0_bucket_final
                ),
            },
            FamilyRound::DrawDiscardP1 => NodeKind::Decision {
                player: 1,
                infoset: format!(
                    "{}:d1:{}:{}",
                    self.family.key_prefix(),
                    self.p1_bucket,
                    self.p1_bucket_final
                ),
            },
            FamilyRound::DrawResolveChance => NodeKind::Chance,
            FamilyRound::P0FinalDecision => NodeKind::Decision {
                player: 0,
                infoset: format!(
                    "{}:p0f:{}:{}",
                    self.family.key_prefix(),
                    self.p0_bucket_final,
                    self.p0_bucket
                ),
            },
            FamilyRound::P1FinalDecision => NodeKind::Decision {
                player: 1,
                infoset: format!(
                    "{}:p1f:{}:{}",
                    self.family.key_prefix(),
                    self.p1_bucket_final,
                    self.p1_bucket
                ),
            },
            FamilyRound::Terminal => NodeKind::Terminal,
        }
    }

    fn legal_actions(&self) -> Vec<usize> {
        match self.round {
            FamilyRound::StudBringIn => vec![0, 1],
            FamilyRound::DrawDiscardP0 | FamilyRound::DrawDiscardP1 => vec![0, 1, 2],
            FamilyRound::P0Decision
            | FamilyRound::P1Decision
            | FamilyRound::P0FinalDecision
            | FamilyRound::P1FinalDecision => vec![0, 1],
            _ => Vec::new(),
        }
    }

    fn apply_action(&self, action: usize) -> Self {
        match self.round {
            FamilyRound::StudBringIn => {
                let mut next = self.clone();
                next.bring_in_action = Some(action);
                next.round = FamilyRound::P0Decision;
                next
            }
            FamilyRound::P0Decision => {
                let mut next = self.clone();
                next.p0_action = Some(action);
                next.round = if action == 0 {
                    FamilyRound::Terminal
                } else {
                    FamilyRound::P1Decision
                };
                next
            }
            FamilyRound::P1Decision => {
                let mut next = self.clone();
                next.p1_action = Some(action);
                next.advance_after_p1_decision();
                next
            }
            FamilyRound::DrawDiscardP0 => {
                let mut next = self.clone();
                next.p0_draw_action = Some(action);
                next.round = FamilyRound::DrawDiscardP1;
                next
            }
            FamilyRound::DrawDiscardP1 => {
                let mut next = self.clone();
                next.p1_draw_action = Some(action);
                next.round = FamilyRound::DrawResolveChance;
                next
            }
            FamilyRound::P0FinalDecision => {
                let mut next = self.clone();
                next.p0_final_action = Some(action);
                next.round = FamilyRound::P1FinalDecision;
                next
            }
            FamilyRound::P1FinalDecision => {
                let mut next = self.clone();
                next.p1_final_action = Some(action);
                next.round = FamilyRound::Terminal;
                next
            }
            _ => self.clone(),
        }
    }

    fn chance_outcomes(&self) -> Vec<(f64, Self)> {
        if self.round != FamilyRound::ChanceDeal {
            if self.round != FamilyRound::ChanceTransition {
                if self.round != FamilyRound::DrawResolveChance {
                    return Vec::new();
                }
                let p0_draw = self.p0_draw_action.unwrap_or(0).min(2) as u8;
                let p1_draw = self.p1_draw_action.unwrap_or(0).min(2) as u8;
                let outcomes = vec![(0_u8, 0_u8), (1, 0), (0, 1), (1, 1)];
                let p = 1.0 / outcomes.len() as f64;
                return outcomes
                    .into_iter()
                    .map(|(d0, d1)| {
                        let mut s = self.clone();
                        let improve0 = (3_u8.saturating_sub(p0_draw)) / 2 + d0;
                        let improve1 = (3_u8.saturating_sub(p1_draw)) / 2 + d1;
                        s.p0_bucket_final = (s.p0_bucket as u16 + improve0 as u16).min(3) as u8;
                        s.p1_bucket_final = (s.p1_bucket as u16 + improve1 as u16).min(3) as u8;
                        s.round = FamilyRound::P0FinalDecision;
                        (p, s)
                    })
                    .collect();
            }
            let transitions = match self.family {
                PokerFamily::Holdem => vec![(0_u8, 0_u8)],
                PokerFamily::Stud => vec![(0_u8, 0_u8), (1, 0), (0, 1), (1, 1)],
                PokerFamily::Draw => vec![(0_u8, 0_u8), (2, 0), (0, 2), (1, 1)],
            };
            let p = 1.0 / transitions.len() as f64;
            return transitions
                .into_iter()
                .map(|(d0, d1)| {
                    let mut s = self.clone();
                    s.p0_bucket_final = (s.p0_bucket as u16 + d0 as u16).min(3) as u8;
                    s.p1_bucket_final = (s.p1_bucket as u16 + d1 as u16).min(3) as u8;
                    s.round = if s.family == PokerFamily::Draw {
                        FamilyRound::DrawDiscardP0
                    } else {
                        FamilyRound::P0FinalDecision
                    };
                    (p, s)
                })
                .collect();
        }
        let outcomes = vec![
            (0_u8, 0_u8),
            (0_u8, 1_u8),
            (1_u8, 0_u8),
            (1_u8, 1_u8),
            (2_u8, 1_u8),
            (1_u8, 2_u8),
        ];
        let p = 1.0 / outcomes.len() as f64;
        outcomes
            .into_iter()
            .map(|(p0, p1)| {
                let mut s = self.clone();
                s.p0_bucket = p0;
                s.p1_bucket = p1;
                s.p0_bucket_final = p0;
                s.p1_bucket_final = p1;
                s.bring_in_player = if s.family == PokerFamily::Stud {
                    if p0 <= p1 {
                        0
                    } else {
                        1
                    }
                } else {
                    0
                };
                s.round = if s.family == PokerFamily::Stud {
                    FamilyRound::StudBringIn
                } else {
                    FamilyRound::P0Decision
                };
                (p, s)
            })
            .collect()
    }

    fn terminal_utility(&self) -> Vec<f64> {
        if self.round != FamilyRound::Terminal {
            return vec![0.0, 0.0];
        }
        let payoff = self.family.payoff();
        let bring_in_adj = if self.family == PokerFamily::Stud {
            match self.bring_in_action {
                Some(0) => 0.2,
                Some(1) => 0.4,
                _ => 0.0,
            }
        } else {
            0.0
        };
        match (self.p0_action, self.p1_action) {
            (Some(0), _) => vec![
                payoff.p0_fold - bring_in_adj,
                -(payoff.p0_fold - bring_in_adj),
            ],
            (Some(1), Some(0)) => vec![
                payoff.p1_fold_to_aggr + bring_in_adj,
                -(payoff.p1_fold_to_aggr + bring_in_adj),
            ],
            (Some(1), Some(1)) => {
                let (b0, b1) = self.showdown_buckets();
                let mut swing = payoff.showdown_win;
                if self.supports_final_round() {
                    if self.p0_final_action == Some(1) && self.p1_final_action == Some(0) {
                        return vec![swing + 0.5, -(swing + 0.5)];
                    }
                    if self.p0_final_action == Some(1) && self.p1_final_action == Some(1) {
                        swing += 0.75;
                    }
                    if self.p0_final_action == Some(0) && self.p1_final_action == Some(1) {
                        swing += 0.25;
                    }
                    if self.family == PokerFamily::Draw {
                        let draw0 = self.p0_draw_action.unwrap_or(0) as f64;
                        let draw1 = self.p1_draw_action.unwrap_or(0) as f64;
                        swing += (draw1 - draw0) * 0.1;
                    }
                }
                if b0 > b1 {
                    vec![swing, -swing]
                } else if b1 > b0 {
                    vec![-swing, swing]
                } else {
                    vec![0.0, 0.0]
                }
            }
            _ => vec![0.0, 0.0],
        }
    }

    fn cache_key(&self) -> Option<String> {
        Some(format!(
            "fa:{:?}:{:?}:{}:{}:{}:{}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}",
            self.family,
            self.round,
            self.p0_bucket,
            self.p1_bucket,
            self.p0_bucket_final,
            self.p1_bucket_final,
            self.bring_in_action,
            self.p0_action,
            self.p1_action,
            self.p0_draw_action,
            self.p1_draw_action,
            self.p0_final_action,
            self.p1_final_action
        ))
    }

    fn subtree_action_cache_key(&self) -> Option<String> {
        self.cache_key()
    }

    fn subtree_value_cache_key(&self, _update_player: usize) -> Option<String> {
        if self.round == FamilyRound::Terminal {
            self.cache_key()
        } else {
            None
        }
    }
}

pub trait FamilyAdapter {
    fn family(&self) -> PokerFamily;
    fn initial_state(&self) -> FamilySubgameState {
        FamilySubgameState::new(self.family())
    }
}

pub struct HoldemAdapter;
impl FamilyAdapter for HoldemAdapter {
    fn family(&self) -> PokerFamily {
        PokerFamily::Holdem
    }
}

pub struct StudAdapter;
impl FamilyAdapter for StudAdapter {
    fn family(&self) -> PokerFamily {
        PokerFamily::Stud
    }
}

pub struct DrawAdapter;
impl FamilyAdapter for DrawAdapter {
    fn family(&self) -> PokerFamily {
        PokerFamily::Draw
    }
}

pub fn family_exploitability_proxy(solver: &CfrPlusSolver<FamilySubgameState>) -> f64 {
    exploitability_two_player(&solver.root, &|infoset, n| {
        solver.average_strategy_for_infoset(infoset, n)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_family_converges<A: FamilyAdapter>(adapter: A) {
        let mut solver = CfrPlusSolver::new(adapter.initial_state());
        let before = family_exploitability_proxy(&solver);
        solver.train(3000);
        let after = family_exploitability_proxy(&solver);
        assert!(after.is_finite());
        assert!(after <= before + 1e-9);
    }

    #[test]
    fn holdem_adapter_converges() {
        assert_family_converges(HoldemAdapter);
    }

    #[test]
    fn stud_adapter_converges() {
        assert_family_converges(StudAdapter);
    }

    #[test]
    fn draw_adapter_converges() {
        assert_family_converges(DrawAdapter);
    }

    #[test]
    fn stud_and_draw_include_transition_and_final_round_nodes() {
        for fam in [PokerFamily::Stud, PokerFamily::Draw] {
            let root = FamilySubgameState::new(fam);
            let outcomes = root.chance_outcomes();
            assert!(!outcomes.is_empty());
            let mut path = outcomes[0].1.clone();

            if matches!(path.node_kind(), NodeKind::Decision { .. }) {
                path = path.apply_action(1);
            }
            if matches!(path.node_kind(), NodeKind::Decision { .. }) {
                path = path.apply_action(1);
            }
            if matches!(path.node_kind(), NodeKind::Decision { .. }) {
                path = path.apply_action(1);
            }

            assert!(matches!(path.node_kind(), NodeKind::Chance));
            let after_transition = path.chance_outcomes();
            assert!(!after_transition.is_empty());
            let final_state = after_transition[0].1.clone();
            if fam == PokerFamily::Draw {
                assert!(matches!(
                    final_state.node_kind(),
                    NodeKind::Decision { player: 0, .. }
                ));
                let discard_p0 = final_state.apply_action(1);
                let discard_p1 = discard_p0.apply_action(2);
                assert!(matches!(discard_p1.node_kind(), NodeKind::Chance));
            } else {
                assert!(matches!(
                    final_state.node_kind(),
                    NodeKind::Decision { player: 0, .. }
                ));
            }
        }
    }
}
