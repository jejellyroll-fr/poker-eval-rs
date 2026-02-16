use crate::solvers::core::{exploitability_two_player, CfrPlusSolver, GameTree, NodeKind};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudVariant {
    StudHi,
    Stud8,
    Razz,
}

#[derive(Debug, Clone)]
pub struct StudRules {
    pub variant: StudVariant,
    pub ante: f64,
    pub bring_in: f64,
    pub complete: f64,
    pub small_bet: f64,
    pub big_bet: f64,
    pub max_raises_per_street: [u8; 5],
    pub low_qualifier_bucket_max: u8,
}

impl StudRules {
    pub fn stud_hi() -> Self {
        Self {
            variant: StudVariant::StudHi,
            ante: 1.0,
            bring_in: 0.5,
            complete: 1.0,
            small_bet: 1.0,
            big_bet: 2.0,
            max_raises_per_street: [3, 3, 3, 3, 3],
            low_qualifier_bucket_max: 2,
        }
    }

    pub fn stud8() -> Self {
        Self {
            variant: StudVariant::Stud8,
            ante: 1.0,
            bring_in: 0.5,
            complete: 1.0,
            small_bet: 1.0,
            big_bet: 2.0,
            max_raises_per_street: [3, 3, 3, 3, 3],
            low_qualifier_bucket_max: 2,
        }
    }

    pub fn razz() -> Self {
        Self {
            variant: StudVariant::Razz,
            ante: 1.0,
            bring_in: 0.5,
            complete: 1.0,
            small_bet: 1.0,
            big_bet: 2.0,
            max_raises_per_street: [3, 3, 3, 3, 3],
            low_qualifier_bucket_max: 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawVariant {
    DeuceToSevenTriple,
    AceToFiveTriple,
}

#[derive(Debug, Clone)]
pub struct DrawRules {
    pub variant: DrawVariant,
    pub ante: f64,
    pub small_bet: f64,
    pub big_bet: f64,
    pub rounds: u8,
    pub max_raises_per_round: u8,
}

impl DrawRules {
    pub fn deuce_to_seven_triple() -> Self {
        Self {
            variant: DrawVariant::DeuceToSevenTriple,
            ante: 1.0,
            small_bet: 1.0,
            big_bet: 2.0,
            rounds: 3,
            max_raises_per_round: 3,
        }
    }

    pub fn ace_to_five_triple() -> Self {
        Self {
            variant: DrawVariant::AceToFiveTriple,
            ante: 1.0,
            small_bet: 1.0,
            big_bet: 2.0,
            rounds: 3,
            max_raises_per_round: 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StudPhase {
    ChanceDeal,
    BringIn,
    StreetAction,
    StreetResponse,
    NextStreetChance,
    Terminal,
}

#[derive(Debug, Clone)]
pub struct StudFullGameState {
    pub rules: StudRules,
    phase: StudPhase,
    street: u8, // 0..4 => 3rd..7th
    actor: usize,
    p0_bucket: u8,
    p1_bucket: u8,
    bring_in_player: usize,
    aggressor: Option<usize>,
    raises_this_street: u8,
    contrib: [f64; 2],
    winner_by_fold: Option<usize>,
}

impl StudFullGameState {
    pub fn new() -> Self {
        Self::with_rules(StudRules::stud_hi())
    }

    pub fn with_rules(rules: StudRules) -> Self {
        Self {
            rules: rules.clone(),
            phase: StudPhase::ChanceDeal,
            street: 0,
            actor: 0,
            p0_bucket: 0,
            p1_bucket: 0,
            bring_in_player: 0,
            aggressor: None,
            raises_this_street: 0,
            // Exact antes posted by both players.
            contrib: [rules.ante, rules.ante],
            winner_by_fold: None,
        }
    }

    fn stake(&self) -> f64 {
        if self.street <= 1 {
            self.rules.small_bet
        } else {
            self.rules.big_bet
        }
    }

    fn advance_street_or_terminal(mut self) -> Self {
        if self.street >= 4 {
            self.phase = StudPhase::Terminal;
            return self;
        }
        self.phase = StudPhase::NextStreetChance;
        self
    }

    fn showdown_utility(&self) -> Vec<f64> {
        let pot = self.contrib[0] + self.contrib[1];
        let (w0, w1) = if let Some(w) = self.winner_by_fold {
            if w == 0 {
                (pot, 0.0)
            } else {
                (0.0, pot)
            }
        } else if self.rules.variant == StudVariant::Stud8 {
            let p0_low = self.p0_bucket <= self.rules.low_qualifier_bucket_max;
            let p1_low = self.p1_bucket <= self.rules.low_qualifier_bucket_max;

            let hi_winner = if self.p0_bucket > self.p1_bucket {
                0
            } else if self.p1_bucket > self.p0_bucket {
                1
            } else {
                2
            };
            let lo_winner = if p0_low && !p1_low {
                0
            } else if p1_low && !p0_low {
                1
            } else if p0_low && p1_low {
                if self.p0_bucket < self.p1_bucket {
                    0
                } else if self.p1_bucket < self.p0_bucket {
                    1
                } else {
                    2
                }
            } else {
                3 // no low qualifier
            };

            let mut chips = [0.0_f64, 0.0_f64];
            let hi_pot = if lo_winner == 3 { pot } else { pot * 0.5 };
            match hi_winner {
                0 => chips[0] += hi_pot,
                1 => chips[1] += hi_pot,
                _ => {
                    chips[0] += hi_pot * 0.5;
                    chips[1] += hi_pot * 0.5;
                }
            }
            if lo_winner != 3 {
                let lo_pot = pot * 0.5;
                match lo_winner {
                    0 => chips[0] += lo_pot,
                    1 => chips[1] += lo_pot,
                    _ => {
                        chips[0] += lo_pot * 0.5;
                        chips[1] += lo_pot * 0.5;
                    }
                }
            }
            (chips[0], chips[1])
        } else if self.rules.variant == StudVariant::Razz {
            if self.p0_bucket < self.p1_bucket {
                (pot, 0.0)
            } else if self.p1_bucket < self.p0_bucket {
                (0.0, pot)
            } else {
                (pot * 0.5, pot * 0.5)
            }
        } else if self.p0_bucket > self.p1_bucket {
            (pot, 0.0)
        } else if self.p1_bucket > self.p0_bucket {
            (0.0, pot)
        } else {
            (pot * 0.5, pot * 0.5)
        };
        vec![w0 - self.contrib[0], w1 - self.contrib[1]]
    }
}

impl Default for StudFullGameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameTree for StudFullGameState {
    fn num_players(&self) -> usize {
        2
    }

    fn node_kind(&self) -> NodeKind {
        match self.phase {
            StudPhase::ChanceDeal | StudPhase::NextStreetChance => NodeKind::Chance,
            StudPhase::BringIn => NodeKind::Decision {
                player: self.bring_in_player,
                infoset: format!("stud:bi:b{}:{}", self.p0_bucket, self.p1_bucket),
            },
            StudPhase::StreetAction => NodeKind::Decision {
                player: self.actor,
                infoset: format!(
                    "stud:s{}:a{}:rs{}:b{}:{}",
                    self.street,
                    self.actor,
                    self.raises_this_street,
                    self.p0_bucket,
                    self.p1_bucket
                ),
            },
            StudPhase::StreetResponse => NodeKind::Decision {
                player: 1 - self.actor,
                infoset: format!(
                    "stud:s{}:r{}:rs{}:b{}:{}",
                    self.street,
                    1 - self.actor,
                    self.raises_this_street,
                    self.p0_bucket,
                    self.p1_bucket
                ),
            },
            StudPhase::Terminal => NodeKind::Terminal,
        }
    }

    fn legal_actions(&self) -> Vec<usize> {
        match self.phase {
            StudPhase::BringIn => vec![0, 1],      // bring-in small or complete
            StudPhase::StreetAction => vec![0, 1], // check or bet
            StudPhase::StreetResponse => {
                let mut acts = vec![0, 1]; // fold or call
                let cap = self.rules.max_raises_per_street[self.street as usize];
                if self.raises_this_street < cap {
                    acts.push(2); // raise
                }
                acts
            }
            _ => Vec::new(),
        }
    }

    fn apply_action(&self, action: usize) -> Self {
        let mut n = self.clone();
        match self.phase {
            StudPhase::BringIn => {
                let p = self.bring_in_player;
                n.contrib[p] += if action == 0 {
                    self.rules.bring_in
                } else {
                    self.rules.complete
                };
                n.actor = 1 - p;
                n.phase = StudPhase::StreetAction;
            }
            StudPhase::StreetAction => {
                if action == 0 {
                    n = n.advance_street_or_terminal();
                } else {
                    let p = self.actor;
                    n.contrib[p] += self.stake();
                    n.aggressor = Some(p);
                    n.raises_this_street = 0;
                    n.phase = StudPhase::StreetResponse;
                }
            }
            StudPhase::StreetResponse => {
                let responder = 1 - self.actor;
                if action == 0 {
                    n.winner_by_fold = self.aggressor;
                    n.phase = StudPhase::Terminal;
                } else if action == 1 {
                    n.contrib[responder] += self.stake();
                    n = n.advance_street_or_terminal();
                } else {
                    // Fixed-limit style raise: responder becomes new aggressor.
                    n.contrib[responder] += self.stake();
                    n.actor = responder;
                    n.aggressor = Some(responder);
                    n.raises_this_street = self.raises_this_street.saturating_add(1);
                    n.phase = StudPhase::StreetResponse;
                }
            }
            _ => {}
        }
        n
    }

    fn chance_outcomes(&self) -> Vec<(f64, Self)> {
        match self.phase {
            StudPhase::ChanceDeal => {
                let outcomes = vec![
                    (0_u8, 1_u8),
                    (1_u8, 0_u8),
                    (1_u8, 2_u8),
                    (2_u8, 1_u8),
                    (2_u8, 3_u8),
                    (3_u8, 2_u8),
                ];
                let p = 1.0 / outcomes.len() as f64;
                outcomes
                    .into_iter()
                    .map(|(b0, b1)| {
                        let mut s = self.clone();
                        s.p0_bucket = b0;
                        s.p1_bucket = b1;
                        // Lower up-card posts bring-in.
                        s.bring_in_player = if b0 <= b1 { 0 } else { 1 };
                        s.phase = StudPhase::BringIn;
                        (p, s)
                    })
                    .collect()
            }
            StudPhase::NextStreetChance => {
                let p = 0.25;
                let mut out = Vec::new();
                for (d0, d1) in [(0_u8, 0_u8), (1, 0), (0, 1), (1, 1)] {
                    let mut s = self.clone();
                    s.street += 1;
                    s.p0_bucket = (s.p0_bucket as u16 + d0 as u16).min(6) as u8;
                    s.p1_bucket = (s.p1_bucket as u16 + d1 as u16).min(6) as u8;
                    s.actor = if s.p0_bucket >= s.p1_bucket { 0 } else { 1 };
                    s.raises_this_street = 0;
                    s.phase = StudPhase::StreetAction;
                    out.push((p, s));
                }
                out
            }
            _ => Vec::new(),
        }
    }

    fn terminal_utility(&self) -> Vec<f64> {
        if self.phase == StudPhase::Terminal {
            self.showdown_utility()
        } else {
            vec![0.0, 0.0]
        }
    }

    fn cache_key(&self) -> Option<String> {
        Some(format!(
            "stud:{:?}:ph{:?}:st{}:a{}:b{}:{}:bi{}:c{:.2}:{:.2}:wf{:?}",
            self.rules.variant,
            self.phase,
            self.street,
            self.actor,
            self.p0_bucket,
            self.p1_bucket,
            self.bring_in_player,
            self.contrib[0],
            self.contrib[1],
            self.winner_by_fold
        ))
    }

    fn subtree_action_cache_key(&self) -> Option<String> {
        self.cache_key()
    }

    fn subtree_value_cache_key(&self, _update_player: usize) -> Option<String> {
        if self.phase == StudPhase::Terminal {
            self.cache_key()
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DrawPhase {
    ChanceDeal,
    BetAction,
    BetResponse,
    DrawP0,
    DrawP1,
    DrawResolveChance,
    Terminal,
}

#[derive(Debug, Clone)]
pub struct DrawFullGameState {
    pub rules: DrawRules,
    phase: DrawPhase,
    round: u8, // 0..2 for triple draw
    actor: usize,
    p0_bucket: u8,
    p1_bucket: u8,
    p0_draw: u8,
    p1_draw: u8,
    aggressor: Option<usize>,
    raises_this_round: u8,
    contrib: [f64; 2],
    winner_by_fold: Option<usize>,
}

impl DrawFullGameState {
    pub fn new() -> Self {
        Self::with_rules(DrawRules::deuce_to_seven_triple())
    }

    pub fn with_rules(rules: DrawRules) -> Self {
        Self {
            rules: rules.clone(),
            phase: DrawPhase::ChanceDeal,
            round: 0,
            actor: 0,
            p0_bucket: 0,
            p1_bucket: 0,
            p0_draw: 0,
            p1_draw: 0,
            aggressor: None,
            raises_this_round: 0,
            // Exact antes posted by both players.
            contrib: [rules.ante, rules.ante],
            winner_by_fold: None,
        }
    }

    fn stake(&self) -> f64 {
        let big_from = self.rules.rounds.saturating_sub(1);
        if self.round < big_from {
            self.rules.small_bet
        } else {
            self.rules.big_bet
        }
    }

    fn showdown_utility(&self) -> Vec<f64> {
        let pot = self.contrib[0] + self.contrib[1];
        let (w0, w1) = if let Some(w) = self.winner_by_fold {
            if w == 0 {
                (pot, 0.0)
            } else {
                (0.0, pot)
            }
        } else if self.rules.variant == DrawVariant::AceToFiveTriple {
            // Lower bucket is better in A-5 style abstraction.
            if self.p0_bucket < self.p1_bucket {
                (pot, 0.0)
            } else if self.p1_bucket < self.p0_bucket {
                (0.0, pot)
            } else {
                (pot * 0.5, pot * 0.5)
            }
        } else if self.p0_bucket > self.p1_bucket {
            (pot, 0.0)
        } else if self.p1_bucket > self.p0_bucket {
            (0.0, pot)
        } else {
            (pot * 0.5, pot * 0.5)
        };
        vec![w0 - self.contrib[0], w1 - self.contrib[1]]
    }
}

impl Default for DrawFullGameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameTree for DrawFullGameState {
    fn num_players(&self) -> usize {
        2
    }

    fn node_kind(&self) -> NodeKind {
        match self.phase {
            DrawPhase::ChanceDeal | DrawPhase::DrawResolveChance => NodeKind::Chance,
            DrawPhase::BetAction => NodeKind::Decision {
                player: self.actor,
                infoset: format!(
                    "draw:r{}:a{}:rr{}:b{}:{}",
                    self.round, self.actor, self.raises_this_round, self.p0_bucket, self.p1_bucket
                ),
            },
            DrawPhase::BetResponse => NodeKind::Decision {
                player: 1 - self.actor,
                infoset: format!(
                    "draw:r{}:r{}:rr{}:b{}:{}",
                    self.round,
                    1 - self.actor,
                    self.raises_this_round,
                    self.p0_bucket,
                    self.p1_bucket
                ),
            },
            DrawPhase::DrawP0 => NodeKind::Decision {
                player: 0,
                infoset: format!(
                    "draw:r{}:d0:{}:{}",
                    self.round, self.p0_bucket, self.p1_bucket
                ),
            },
            DrawPhase::DrawP1 => NodeKind::Decision {
                player: 1,
                infoset: format!(
                    "draw:r{}:d1:{}:{}",
                    self.round, self.p0_bucket, self.p1_bucket
                ),
            },
            DrawPhase::Terminal => NodeKind::Terminal,
        }
    }

    fn legal_actions(&self) -> Vec<usize> {
        match self.phase {
            DrawPhase::BetAction => vec![0, 1], // check or bet
            DrawPhase::BetResponse => {
                let mut acts = vec![0, 1]; // fold or call
                if self.raises_this_round < self.rules.max_raises_per_round {
                    acts.push(2); // raise
                }
                acts
            }
            DrawPhase::DrawP0 | DrawPhase::DrawP1 => vec![0, 1, 2, 3], // stand pat / draw 1..3
            _ => Vec::new(),
        }
    }

    fn apply_action(&self, action: usize) -> Self {
        let mut n = self.clone();
        match self.phase {
            DrawPhase::BetAction => {
                if action == 0 {
                    if self.round + 1 >= self.rules.rounds {
                        n.phase = DrawPhase::Terminal;
                    } else {
                        n.phase = DrawPhase::DrawP0;
                    }
                } else {
                    let p = self.actor;
                    n.contrib[p] += self.stake();
                    n.aggressor = Some(p);
                    n.raises_this_round = 0;
                    n.phase = DrawPhase::BetResponse;
                }
            }
            DrawPhase::BetResponse => {
                let responder = 1 - self.actor;
                if action == 0 {
                    n.winner_by_fold = self.aggressor;
                    n.phase = DrawPhase::Terminal;
                } else if action == 1 {
                    n.contrib[responder] += self.stake();
                    if self.round + 1 >= self.rules.rounds {
                        n.phase = DrawPhase::Terminal;
                    } else {
                        n.phase = DrawPhase::DrawP0;
                    }
                } else {
                    n.contrib[responder] += self.stake();
                    n.actor = responder;
                    n.aggressor = Some(responder);
                    n.raises_this_round = self.raises_this_round.saturating_add(1);
                    n.phase = DrawPhase::BetResponse;
                }
            }
            DrawPhase::DrawP0 => {
                n.p0_draw = (action.min(3)) as u8;
                n.phase = DrawPhase::DrawP1;
            }
            DrawPhase::DrawP1 => {
                n.p1_draw = (action.min(3)) as u8;
                n.phase = DrawPhase::DrawResolveChance;
            }
            _ => {}
        }
        n
    }

    fn chance_outcomes(&self) -> Vec<(f64, Self)> {
        match self.phase {
            DrawPhase::ChanceDeal => {
                let outcomes = vec![
                    (0_u8, 0_u8),
                    (1_u8, 0_u8),
                    (0_u8, 1_u8),
                    (1_u8, 1_u8),
                    (2_u8, 1_u8),
                    (1_u8, 2_u8),
                ];
                let p = 1.0 / outcomes.len() as f64;
                outcomes
                    .into_iter()
                    .map(|(b0, b1)| {
                        let mut s = self.clone();
                        s.p0_bucket = b0;
                        s.p1_bucket = b1;
                        s.phase = DrawPhase::BetAction;
                        s.actor = 0;
                        (p, s)
                    })
                    .collect()
            }
            DrawPhase::DrawResolveChance => {
                fn dist(draw: u8) -> &'static [(u8, f64)] {
                    match draw.min(3) {
                        0 => &[(0, 0.60), (1, 0.30), (2, 0.10)],
                        1 => &[(0, 0.30), (1, 0.50), (2, 0.20)],
                        2 => &[(0, 0.20), (1, 0.50), (2, 0.30)],
                        _ => &[(0, 0.15), (1, 0.45), (2, 0.40)],
                    }
                }

                let d0 = dist(self.p0_draw);
                let d1 = dist(self.p1_draw);
                let mut out = Vec::new();
                for (inc0, p0) in d0 {
                    for (inc1, p1) in d1 {
                        let mut s = self.clone();
                        if self.rules.variant == DrawVariant::AceToFiveTriple {
                            s.p0_bucket = s.p0_bucket.saturating_sub((*inc0).min(2));
                            s.p1_bucket = s.p1_bucket.saturating_sub((*inc1).min(2));
                        } else {
                            s.p0_bucket = (s.p0_bucket as u16 + *inc0 as u16).min(6) as u8;
                            s.p1_bucket = (s.p1_bucket as u16 + *inc1 as u16).min(6) as u8;
                        }
                        s.round += 1;
                        s.phase = DrawPhase::BetAction;
                        s.actor = if s.round % 2 == 0 { 0 } else { 1 };
                        s.raises_this_round = 0;
                        s.p0_draw = 0;
                        s.p1_draw = 0;
                        out.push((p0 * p1, s));
                    }
                }
                out
            }
            _ => Vec::new(),
        }
    }

    fn terminal_utility(&self) -> Vec<f64> {
        if self.phase == DrawPhase::Terminal {
            self.showdown_utility()
        } else {
            vec![0.0, 0.0]
        }
    }

    fn cache_key(&self) -> Option<String> {
        Some(format!(
            "draw:{:?}:ph{:?}:r{}:a{}:b{}:{}:d{}:{}:rr{}:c{:.2}:{:.2}:wf{:?}",
            self.rules.variant,
            self.phase,
            self.round,
            self.actor,
            self.p0_bucket,
            self.p1_bucket,
            self.p0_draw,
            self.p1_draw,
            self.raises_this_round,
            self.contrib[0],
            self.contrib[1],
            self.winner_by_fold
        ))
    }

    fn subtree_action_cache_key(&self) -> Option<String> {
        self.cache_key()
    }

    fn subtree_value_cache_key(&self, _update_player: usize) -> Option<String> {
        if self.phase == DrawPhase::Terminal {
            self.cache_key()
        } else {
            None
        }
    }
}

pub fn stud_exploitability_proxy(solver: &CfrPlusSolver<StudFullGameState>) -> f64 {
    exploitability_two_player(&solver.root, &|infoset, n| {
        solver.average_strategy_for_infoset(infoset, n)
    })
}

pub fn draw_exploitability_proxy(solver: &CfrPlusSolver<DrawFullGameState>) -> f64 {
    exploitability_two_player(&solver.root, &|infoset, n| {
        solver.average_strategy_for_infoset(infoset, n)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stud_full_state_converges_proxy() {
        let mut solver = CfrPlusSolver::new(StudFullGameState::new());
        let before = stud_exploitability_proxy(&solver);
        solver.train(40);
        let after = stud_exploitability_proxy(&solver);
        assert!(after.is_finite());
        assert!(after <= before + 1e-9);
    }

    #[test]
    fn draw_full_state_converges_proxy() {
        let mut solver = CfrPlusSolver::new(DrawFullGameState::new());
        let before = draw_exploitability_proxy(&solver);
        solver.train(40);
        let after = draw_exploitability_proxy(&solver);
        assert!(after.is_finite());
        assert!(after <= before + 1e-9);
    }

    #[test]
    fn stud8_supports_hilo_split() {
        let mut s = StudFullGameState::with_rules(StudRules::stud8());
        s.phase = StudPhase::Terminal;
        s.p0_bucket = 4;
        s.p1_bucket = 2;
        s.contrib = [5.0, 5.0];
        // p0 wins high (higher bucket), p1 qualifies and wins low (lower bucket),
        // so split pot gives both near break-even.
        let u = s.terminal_utility();
        assert!(u[0].abs() < 1e-9);
        assert!(u[1].abs() < 1e-9);
    }

    #[test]
    fn draw_a5_uses_low_ordering() {
        let mut s = DrawFullGameState::with_rules(DrawRules::ace_to_five_triple());
        s.phase = DrawPhase::Terminal;
        s.p0_bucket = 1;
        s.p1_bucket = 4;
        s.contrib = [3.0, 3.0];
        let u = s.terminal_utility();
        assert!(u[0] > 0.0);
        assert!(u[1] < 0.0);
    }

    #[test]
    fn stud_bring_in_and_complete_amounts_are_applied() {
        let mut s = StudFullGameState::with_rules(StudRules::stud_hi());
        s.phase = StudPhase::BringIn;
        s.bring_in_player = 0;
        s.contrib = [1.0, 1.0];

        let s_bi = s.apply_action(0);
        assert!((s_bi.contrib[0] - 1.5).abs() < 1e-9);

        let s_cp = s.apply_action(1);
        assert!((s_cp.contrib[0] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn stud8_without_low_qualifier_is_hi_only() {
        let mut s = StudFullGameState::with_rules(StudRules::stud8());
        s.phase = StudPhase::Terminal;
        s.p0_bucket = 6;
        s.p1_bucket = 5;
        s.contrib = [4.0, 4.0];
        // No low (both > qualifier), whole pot to high winner p0.
        let u = s.terminal_utility();
        assert!((u[0] - 4.0).abs() < 1e-9);
        assert!((u[1] + 4.0).abs() < 1e-9);
    }

    #[test]
    fn draw_rules_round_count_is_respected() {
        let mut rules = DrawRules::deuce_to_seven_triple();
        rules.rounds = 2;
        let mut s = DrawFullGameState::with_rules(rules);
        s.phase = DrawPhase::BetAction;
        s.round = 1;
        // Check should now end immediately because round+1 >= rounds.
        let n = s.apply_action(0);
        assert!(matches!(n.phase, DrawPhase::Terminal));
    }

    #[test]
    fn stud_raise_cap_is_enforced() {
        let mut rules = StudRules::stud_hi();
        rules.max_raises_per_street = [1, 1, 1, 1, 1];
        let mut s = StudFullGameState::with_rules(rules);
        s.phase = StudPhase::StreetAction;
        s.street = 0;
        s.actor = 0;

        // Open bet, then one raise is allowed, then no further raise action.
        let s1 = s.apply_action(1);
        let s2 = s1.apply_action(2);
        let acts = s2.legal_actions();
        assert!(!acts.contains(&2));
    }

    #[test]
    fn draw_raise_cap_is_enforced() {
        let mut rules = DrawRules::deuce_to_seven_triple();
        rules.max_raises_per_round = 1;
        let mut s = DrawFullGameState::with_rules(rules);
        s.phase = DrawPhase::BetAction;
        s.round = 0;
        s.actor = 0;

        let s1 = s.apply_action(1);
        let s2 = s1.apply_action(2);
        let acts = s2.legal_actions();
        assert!(!acts.contains(&2));
    }

    #[test]
    fn razz_uses_low_wins_showdown() {
        let mut s = StudFullGameState::with_rules(StudRules::razz());
        s.phase = StudPhase::Terminal;
        s.p0_bucket = 1;
        s.p1_bucket = 4;
        s.contrib = [2.0, 2.0];
        let u = s.terminal_utility();
        assert!(u[0] > 0.0);
        assert!(u[1] < 0.0);
    }
}
