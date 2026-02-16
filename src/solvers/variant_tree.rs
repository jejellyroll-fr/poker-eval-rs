use crate::solvers::betting::{DeckProfile, GameConfig, PokerVariant};
use crate::solvers::betting_tree::BettingGameTree;
use crate::solvers::core::{CfrPlusSolver, GameTree, NodeKind};
use crate::{
    deck::{StdDeck, StdDeckCardMask},
    evaluators::{HandEvaluator, HoldemEvaluator, OmahaHiEvaluator, ShortDeckEvaluator},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VariantPhase {
    ChanceDeal,
    Play,
}

/// Variant-aware game tree:
/// - Root chance node generates private-strength scenarios.
/// - Decision/terminal flow is delegated to `BettingGameTree`.
#[derive(Debug, Clone)]
pub struct VariantGameTree {
    pub config: GameConfig,
    pub dealer: usize,
    phase: VariantPhase,
    pub play: Option<BettingGameTree>,
    cached_strength_scenarios: Option<Vec<Vec<u32>>>,
}

impl VariantGameTree {
    pub fn new(config: GameConfig, dealer: usize) -> Self {
        let mut s = Self {
            config,
            dealer,
            phase: VariantPhase::ChanceDeal,
            play: None,
            cached_strength_scenarios: None,
        };
        s.cached_strength_scenarios = Some(s.strength_scenarios_uncached());
        s
    }

    fn strength_scenarios_uncached(&self) -> Vec<Vec<u32>> {
        self.sampled_deals()
            .into_iter()
            .map(|d| self.evaluate_strengths(&d.holes, &d.board))
            .collect()
    }

    fn strength_scenarios(&self) -> Vec<Vec<u32>> {
        self.cached_strength_scenarios
            .clone()
            .unwrap_or_else(|| self.strength_scenarios_uncached())
    }

    fn sampled_deals(&self) -> Vec<DealScenario> {
        let n = self.config.num_players;
        if n == 0 {
            return vec![DealScenario {
                holes: Vec::new(),
                board: StdDeckCardMask::new(),
            }];
        }

        let hole_cards = match self.config.variant {
            PokerVariant::Holdem | PokerVariant::ShortDeck => 2,
            PokerVariant::Omaha => 4,
        };
        let board_cards = 5;
        let need = n * hole_cards + board_cards;
        let deck = self.variant_deck_indices();

        if deck.is_empty() || need > deck.len() {
            return vec![DealScenario {
                holes: vec![StdDeckCardMask::new(); n],
                board: StdDeckCardMask::new(),
            }];
        }

        let n_scenarios = self.config.chance_scenarios.max(1);
        let stride = self.config.chance_stride.max(1) % deck.len().max(1);
        let mut out = Vec::with_capacity(n_scenarios);
        for k in 0..n_scenarios {
            let start = (k * stride) % deck.len();
            out.push(self.build_deal_from_offset(start, hole_cards, board_cards));
        }
        out
    }

    fn variant_deck_indices(&self) -> Vec<usize> {
        match self.config.variant {
            PokerVariant::Holdem | PokerVariant::Omaha => (0..52).collect(),
            PokerVariant::ShortDeck => (0..52)
                .filter(|idx| StdDeck::rank(*idx).as_usize() >= 4)
                .collect(),
        }
    }

    fn build_deal_from_offset(
        &self,
        start: usize,
        hole_cards: usize,
        board_cards: usize,
    ) -> DealScenario {
        let deck = self.variant_deck_indices();
        let n = self.config.num_players;
        let need = n * hole_cards + board_cards;
        let mut cards = Vec::with_capacity(need);
        for k in 0..need {
            cards.push(deck[(start + k) % deck.len()]);
        }

        let mut ptr = 0;
        let mut holes = vec![StdDeckCardMask::new(); n];
        for h in holes.iter_mut().take(n) {
            for _ in 0..hole_cards {
                h.set(cards[ptr]);
                ptr += 1;
            }
        }
        let mut board = StdDeckCardMask::new();
        for _ in 0..board_cards {
            board.set(cards[ptr]);
            ptr += 1;
        }
        DealScenario { holes, board }
    }

    fn evaluate_strengths(&self, holes: &[StdDeckCardMask], board: &StdDeckCardMask) -> Vec<u32> {
        let n = self.config.num_players;
        let mut strengths = vec![0_u32; n];
        for (p, s) in strengths.iter_mut().enumerate().take(n) {
            let hole = holes.get(p).copied().unwrap_or_else(StdDeckCardMask::new);
            *s = match self.config.variant {
                PokerVariant::Holdem => HoldemEvaluator::evaluate_hand(&hole, board)
                    .map(|v| v.value)
                    .unwrap_or(0),
                PokerVariant::Omaha => OmahaHiEvaluator::evaluate_hand(&hole, board)
                    .ok()
                    .flatten()
                    .map(|v| v.value)
                    .unwrap_or(0),
                PokerVariant::ShortDeck => ShortDeckEvaluator::evaluate_hand(&hole, board)
                    .map(|v| v.value)
                    .unwrap_or(0),
            };
        }
        strengths
    }

    pub fn exploitability_proxy(solver: &CfrPlusSolver<VariantGameTree>) -> f64 {
        crate::solvers::core::exploitability_two_player(&solver.root, &|infoset, n| {
            solver.average_strategy_for_infoset(infoset, n)
        })
    }
}

#[derive(Debug, Clone)]
struct DealScenario {
    holes: Vec<StdDeckCardMask>,
    board: StdDeckCardMask,
}

impl GameTree for VariantGameTree {
    fn num_players(&self) -> usize {
        self.config.num_players
    }

    fn node_kind(&self) -> NodeKind {
        match self.phase {
            VariantPhase::ChanceDeal => NodeKind::Chance,
            VariantPhase::Play => self
                .play
                .as_ref()
                .map(|g| g.node_kind())
                .unwrap_or(NodeKind::Terminal),
        }
    }

    fn legal_actions(&self) -> Vec<usize> {
        match self.phase {
            VariantPhase::ChanceDeal => Vec::new(),
            VariantPhase::Play => self
                .play
                .as_ref()
                .map(|g| g.legal_actions())
                .unwrap_or_default(),
        }
    }

    fn apply_action(&self, action: usize) -> Self {
        match self.phase {
            VariantPhase::ChanceDeal => self.clone(),
            VariantPhase::Play => {
                let mut next = self.clone();
                if let Some(g) = &self.play {
                    next.play = Some(g.apply_action(action));
                }
                next
            }
        }
    }

    fn chance_outcomes(&self) -> Vec<(f64, Self)> {
        if self.phase != VariantPhase::ChanceDeal {
            return Vec::new();
        }

        let scenarios = self.strength_scenarios();
        let p = 1.0 / scenarios.len() as f64;
        scenarios
            .into_iter()
            .map(|strengths| {
                let mut cfg = self.config.clone();
                // Ensure deck profile is coherent with variant default
                if cfg.variant == PokerVariant::ShortDeck {
                    cfg.deck_profile = DeckProfile::Short36;
                }
                let play = BettingGameTree::new(cfg, self.dealer, strengths);
                (
                    p,
                    Self {
                        config: self.config.clone(),
                        dealer: self.dealer,
                        phase: VariantPhase::Play,
                        play: Some(play),
                        cached_strength_scenarios: self.cached_strength_scenarios.clone(),
                    },
                )
            })
            .collect()
    }

    fn terminal_utility(&self) -> Vec<f64> {
        match self.phase {
            VariantPhase::ChanceDeal => vec![0.0; self.config.num_players],
            VariantPhase::Play => self
                .play
                .as_ref()
                .map(|g| g.terminal_utility())
                .unwrap_or_else(|| vec![0.0; self.config.num_players]),
        }
    }

    fn cache_key(&self) -> Option<String> {
        match self.phase {
            VariantPhase::ChanceDeal => Some(format!(
                "vt:chance:v{:?}:p{}:d{}:n{}:stride{}",
                self.config.variant,
                self.config.num_players,
                self.dealer,
                self.config.chance_scenarios,
                self.config.chance_stride
            )),
            VariantPhase::Play => self.play.as_ref().and_then(|p| p.cache_key()),
        }
    }

    fn subtree_action_cache_key(&self) -> Option<String> {
        self.cache_key()
    }

    fn subtree_value_cache_key(&self, update_player: usize) -> Option<String> {
        match self.phase {
            VariantPhase::ChanceDeal => None,
            VariantPhase::Play => self
                .play
                .as_ref()
                .and_then(|p| p.subtree_value_cache_key(update_player)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chance_outcomes_exist_for_holdem_omaha_shortdeck() {
        for v in [
            PokerVariant::Holdem,
            PokerVariant::Omaha,
            PokerVariant::ShortDeck,
        ] {
            let mut cfg = GameConfig::no_limit(3, 20, 1, 2);
            cfg.variant = v;
            if v == PokerVariant::ShortDeck {
                cfg.deck_profile = DeckProfile::Short36;
            }
            let root = VariantGameTree::new(cfg, 0);
            let outcomes = root.chance_outcomes();
            assert!(!outcomes.is_empty());
            let p_sum: f64 = outcomes.iter().map(|(p, _)| *p).sum();
            assert!((p_sum - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn cfr_plus_runs_on_variant_tree() {
        let mut cfg = GameConfig::no_limit(3, 8, 1, 2);
        cfg.variant = PokerVariant::Omaha;
        let root = VariantGameTree::new(cfg, 0);
        let mut solver = CfrPlusSolver::new(root);
        solver.train(6);
        assert_eq!(solver.iteration, 6);
        assert!(!solver.table.is_empty());
    }

    #[test]
    fn sampled_deals_are_disjoint_and_card_counts_match_variant() {
        for v in [
            PokerVariant::Holdem,
            PokerVariant::Omaha,
            PokerVariant::ShortDeck,
        ] {
            let mut cfg = GameConfig::no_limit(3, 20, 1, 2);
            cfg.variant = v;
            if v == PokerVariant::ShortDeck {
                cfg.deck_profile = DeckProfile::Short36;
            }
            let root = VariantGameTree::new(cfg, 0);
            let deals = root.sampled_deals();
            assert!(!deals.is_empty());
            let hole_cards = if v == PokerVariant::Omaha { 4 } else { 2 };
            for d in deals {
                let mut used = StdDeckCardMask::new();
                for h in &d.holes {
                    assert_eq!(h.num_cards(), hole_cards);
                    let overlap = used.as_raw() & h.as_raw();
                    assert_eq!(overlap, 0);
                    used.or(h);
                }
                assert_eq!(d.board.num_cards(), 5);
                assert_eq!(used.as_raw() & d.board.as_raw(), 0);
            }
        }
    }
}
