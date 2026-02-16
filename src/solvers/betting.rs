use std::cmp::min;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BettingStructure {
    NoLimit,
    PotLimit,
    FixedLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PokerVariant {
    Holdem,
    Omaha,
    ShortDeck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeckProfile {
    Standard52,
    Short36,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeProfile {
    Tight,
    Standard,
    Wide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionClass {
    Early,
    Middle,
    Late,
    Blinds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
    Showdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatus {
    Active,
    Folded,
    AllIn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Fold,
    Check,
    Call,
    Bet(u64),
    RaiseTo(u64),
    AllIn,
}

#[derive(Debug, Clone)]
pub struct StreetSizeProfile {
    pub preflop: Vec<f64>,
    pub flop: Vec<f64>,
    pub turn: Vec<f64>,
    pub river: Vec<f64>,
}

impl StreetSizeProfile {
    fn for_street(&self, street: Street) -> &[f64] {
        match street {
            Street::Preflop => &self.preflop,
            Street::Flop => &self.flop,
            Street::Turn => &self.turn,
            Street::River | Street::Showdown => &self.river,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PositionSizeProfile {
    pub early: Vec<f64>,
    pub middle: Vec<f64>,
    pub late: Vec<f64>,
    pub blinds: Vec<f64>,
}

impl PositionSizeProfile {
    fn for_position(&self, pos: PositionClass) -> &[f64] {
        match pos {
            PositionClass::Early => &self.early,
            PositionClass::Middle => &self.middle,
            PositionClass::Late => &self.late,
            PositionClass::Blinds => &self.blinds,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActionAbstraction {
    pub preflop_open_sizes: PositionSizeProfile,
    pub preflop_3bet_sizes: PositionSizeProfile,
    pub preflop_4bet_sizes: PositionSizeProfile,
    pub preflop_5bet_sizes: PositionSizeProfile,
    pub allow_overbet: bool,
    pub allow_donk_bet: bool,
    pub max_raises_per_street: [usize; 4],
}

impl TreeProfile {
    fn action_abstraction(self, variant: PokerVariant) -> ActionAbstraction {
        let mk =
            |early: &[f64], middle: &[f64], late: &[f64], blinds: &[f64]| PositionSizeProfile {
                early: early.to_vec(),
                middle: middle.to_vec(),
                late: late.to_vec(),
                blinds: blinds.to_vec(),
            };
        match (variant, self) {
            (_, TreeProfile::Tight) => ActionAbstraction {
                preflop_open_sizes: mk(&[2.2], &[2.3], &[2.5], &[3.0]),
                preflop_3bet_sizes: mk(&[2.8], &[3.0], &[3.2], &[3.4]),
                preflop_4bet_sizes: mk(&[2.2], &[2.3], &[2.4], &[2.5]),
                preflop_5bet_sizes: mk(&[2.0], &[2.1], &[2.2], &[2.3]),
                allow_overbet: false,
                allow_donk_bet: false,
                max_raises_per_street: [2, 2, 2, 2],
            },
            (_, TreeProfile::Standard) => ActionAbstraction {
                preflop_open_sizes: mk(&[2.2, 2.5], &[2.3, 2.7], &[2.5, 3.0], &[3.0, 3.5]),
                preflop_3bet_sizes: mk(&[2.8, 3.2], &[3.0, 3.5], &[3.2, 3.8], &[3.5, 4.0]),
                preflop_4bet_sizes: mk(&[2.1, 2.3], &[2.2, 2.4], &[2.3, 2.5], &[2.4, 2.6]),
                preflop_5bet_sizes: mk(&[2.0, 2.2], &[2.1, 2.3], &[2.2, 2.4], &[2.3, 2.5]),
                allow_overbet: true,
                allow_donk_bet: true,
                max_raises_per_street: [3, 2, 2, 2],
            },
            (_, TreeProfile::Wide) => ActionAbstraction {
                preflop_open_sizes: mk(
                    &[2.0, 2.3, 2.6],
                    &[2.2, 2.5, 2.8],
                    &[2.4, 2.8, 3.2],
                    &[3.0, 3.5, 4.0],
                ),
                preflop_3bet_sizes: mk(
                    &[2.6, 3.0, 3.4],
                    &[2.8, 3.2, 3.8],
                    &[3.0, 3.5, 4.2],
                    &[3.3, 3.8, 4.5],
                ),
                preflop_4bet_sizes: mk(
                    &[2.0, 2.2, 2.4],
                    &[2.1, 2.3, 2.5],
                    &[2.2, 2.4, 2.7],
                    &[2.3, 2.6, 2.9],
                ),
                preflop_5bet_sizes: mk(
                    &[1.9, 2.1, 2.3],
                    &[2.0, 2.2, 2.4],
                    &[2.1, 2.3, 2.6],
                    &[2.2, 2.5, 2.8],
                ),
                allow_overbet: true,
                allow_donk_bet: true,
                max_raises_per_street: [4, 3, 3, 3],
            },
        }
    }
}

impl TreeProfile {
    fn defaults(self, variant: PokerVariant) -> (StreetSizeProfile, StreetSizeProfile) {
        match (variant, self) {
            (PokerVariant::Holdem, TreeProfile::Tight) => (
                StreetSizeProfile {
                    preflop: vec![2.2],
                    flop: vec![0.5],
                    turn: vec![0.5],
                    river: vec![0.75],
                },
                StreetSizeProfile {
                    preflop: vec![2.2],
                    flop: vec![2.0],
                    turn: vec![2.0],
                    river: vec![2.5],
                },
            ),
            (PokerVariant::Holdem, TreeProfile::Standard) => (
                StreetSizeProfile {
                    preflop: vec![2.2, 3.0],
                    flop: vec![0.5, 0.75],
                    turn: vec![0.66, 1.0],
                    river: vec![0.75, 1.25],
                },
                StreetSizeProfile {
                    preflop: vec![2.5, 3.5],
                    flop: vec![2.0, 3.0],
                    turn: vec![2.0, 3.0],
                    river: vec![2.5, 4.0],
                },
            ),
            (PokerVariant::Holdem, TreeProfile::Wide) => (
                StreetSizeProfile {
                    preflop: vec![2.0, 2.5, 3.0, 4.0],
                    flop: vec![0.33, 0.5, 0.75, 1.25],
                    turn: vec![0.5, 0.75, 1.0, 1.5],
                    river: vec![0.5, 1.0, 1.5, 2.0],
                },
                StreetSizeProfile {
                    preflop: vec![2.2, 3.0, 4.0],
                    flop: vec![2.0, 3.0, 4.5],
                    turn: vec![2.0, 3.0, 4.5],
                    river: vec![2.5, 4.0, 6.0],
                },
            ),
            (PokerVariant::Omaha, TreeProfile::Tight) => (
                StreetSizeProfile {
                    preflop: vec![2.5],
                    flop: vec![0.5],
                    turn: vec![0.66],
                    river: vec![0.75],
                },
                StreetSizeProfile {
                    preflop: vec![2.5],
                    flop: vec![2.0],
                    turn: vec![2.5],
                    river: vec![3.0],
                },
            ),
            (PokerVariant::Omaha, TreeProfile::Standard) => (
                StreetSizeProfile {
                    preflop: vec![2.5, 3.5],
                    flop: vec![0.5, 0.75],
                    turn: vec![0.66, 1.0],
                    river: vec![0.75, 1.25],
                },
                StreetSizeProfile {
                    preflop: vec![2.5, 3.5],
                    flop: vec![2.0, 3.0],
                    turn: vec![2.5, 3.5],
                    river: vec![3.0, 4.5],
                },
            ),
            (PokerVariant::Omaha, TreeProfile::Wide) => (
                StreetSizeProfile {
                    preflop: vec![2.2, 2.8, 3.5, 4.5],
                    flop: vec![0.33, 0.5, 0.75, 1.0],
                    turn: vec![0.5, 0.75, 1.0, 1.5],
                    river: vec![0.5, 1.0, 1.5, 2.0],
                },
                StreetSizeProfile {
                    preflop: vec![2.5, 3.5, 4.5],
                    flop: vec![2.0, 3.0, 4.0],
                    turn: vec![2.5, 3.5, 5.0],
                    river: vec![3.0, 4.5, 6.0],
                },
            ),
            (PokerVariant::ShortDeck, TreeProfile::Tight) => (
                StreetSizeProfile {
                    preflop: vec![2.5],
                    flop: vec![0.5],
                    turn: vec![0.75],
                    river: vec![1.0],
                },
                StreetSizeProfile {
                    preflop: vec![2.5],
                    flop: vec![2.0],
                    turn: vec![2.5],
                    river: vec![3.0],
                },
            ),
            (PokerVariant::ShortDeck, TreeProfile::Standard) => (
                StreetSizeProfile {
                    preflop: vec![2.5, 3.5],
                    flop: vec![0.5, 0.75],
                    turn: vec![0.75, 1.25],
                    river: vec![1.0, 1.5],
                },
                StreetSizeProfile {
                    preflop: vec![2.5, 3.5],
                    flop: vec![2.0, 3.0],
                    turn: vec![2.5, 3.5],
                    river: vec![3.0, 4.5],
                },
            ),
            (PokerVariant::ShortDeck, TreeProfile::Wide) => (
                StreetSizeProfile {
                    preflop: vec![2.2, 2.8, 3.5, 4.5],
                    flop: vec![0.33, 0.5, 0.75, 1.25],
                    turn: vec![0.5, 0.75, 1.25, 1.75],
                    river: vec![0.75, 1.25, 2.0, 3.0],
                },
                StreetSizeProfile {
                    preflop: vec![2.5, 3.5, 4.5],
                    flop: vec![2.0, 3.0, 4.0],
                    turn: vec![2.5, 3.5, 5.0],
                    river: vec![3.0, 4.5, 6.0],
                },
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub variant: PokerVariant,
    pub deck_profile: DeckProfile,
    pub num_players: usize,
    pub small_blind: u64,
    pub big_blind: u64,
    pub ante: u64,
    pub min_bet: u64,
    pub tree_profile: TreeProfile,
    pub action_abstraction: Option<ActionAbstraction>,
    pub bet_size_fracs_by_street: Option<StreetSizeProfile>,
    pub raise_size_fracs_by_street: Option<StreetSizeProfile>,
    pub bet_size_fracs: Vec<f64>,
    pub raise_size_fracs: Vec<f64>,
    pub raise_cap: Option<usize>,
    pub betting_structure: BettingStructure,
    pub stack_start: u64,
    pub chance_scenarios: usize,
    pub chance_stride: usize,
}

impl GameConfig {
    pub fn no_limit(
        num_players: usize,
        stack_start: u64,
        small_blind: u64,
        big_blind: u64,
    ) -> Self {
        Self {
            variant: PokerVariant::Holdem,
            deck_profile: DeckProfile::Standard52,
            num_players,
            small_blind,
            big_blind,
            ante: 0,
            min_bet: big_blind,
            tree_profile: TreeProfile::Standard,
            action_abstraction: None,
            bet_size_fracs_by_street: None,
            raise_size_fracs_by_street: None,
            bet_size_fracs: vec![0.5, 1.0],
            raise_size_fracs: vec![1.0, 2.0],
            raise_cap: None,
            betting_structure: BettingStructure::NoLimit,
            stack_start,
            chance_scenarios: 6,
            chance_stride: 7,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidePot {
    pub amount: u64,
    pub eligible_players: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct HandState {
    pub config: GameConfig,
    pub street: Street,
    pub dealer: usize,
    pub to_act: usize,
    pub stacks: Vec<u64>,
    pub status: Vec<PlayerStatus>,
    pub committed_round: Vec<u64>,
    pub committed_total: Vec<u64>,
    pub current_bet: u64,
    pub last_full_raise: u64,
    pub raises_in_round: usize,
    pub street_aggressor: Option<usize>,
    pub prev_street_aggressor: Option<usize>,
    pub terminal: bool,
}

impl HandState {
    pub fn new(config: GameConfig, dealer: usize) -> Self {
        let n = config.num_players;
        let mut s = Self {
            config: config.clone(),
            street: Street::Preflop,
            dealer,
            to_act: 0,
            stacks: vec![config.stack_start; n],
            status: vec![PlayerStatus::Active; n],
            committed_round: vec![0; n],
            committed_total: vec![0; n],
            current_bet: 0,
            last_full_raise: config.big_blind,
            raises_in_round: 0,
            street_aggressor: None,
            prev_street_aggressor: None,
            terminal: false,
        };

        if config.ante > 0 {
            for i in 0..n {
                let paid = min(config.ante, s.stacks[i]);
                s.stacks[i] -= paid;
                s.committed_total[i] += paid;
            }
        }

        let sb = (dealer + 1) % n;
        let bb = (dealer + 2) % n;
        s.post_blind(sb, config.small_blind);
        s.post_blind(bb, config.big_blind);
        s.current_bet = config.big_blind;
        s.street_aggressor = Some(bb);
        s.to_act = s.next_active_from((bb + 1) % n);
        s
    }

    pub fn facing_amount(&self, player: usize) -> u64 {
        self.current_bet
            .saturating_sub(self.committed_round[player])
    }

    pub fn active_players(&self) -> Vec<usize> {
        self.status
            .iter()
            .enumerate()
            .filter_map(|(i, st)| {
                if *st != PlayerStatus::Folded {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn legal_actions(&self) -> Vec<Action> {
        if self.terminal || self.status[self.to_act] != PlayerStatus::Active {
            return Vec::new();
        }

        let p = self.to_act;
        let stack = self.stacks[p];
        let facing = self.facing_amount(p);
        let mut actions = Vec::new();

        if facing > 0 {
            actions.push(Action::Fold);
            actions.push(Action::Call);
            if stack > facing && self.can_raise() {
                let min_to = self.current_bet + self.last_full_raise.max(self.config.min_bet);
                let max_to = self.committed_round[p] + stack;
                if min_to <= max_to {
                    actions.push(Action::RaiseTo(min_to));
                    let pot = self.current_pot().saturating_add(facing).max(1);
                    for to in self.raise_to_sizes_from_fracs(pot, min_to, max_to) {
                        actions.push(Action::RaiseTo(to));
                    }
                }
            }
            actions.push(Action::AllIn);
        } else {
            actions.push(Action::Check);
            if stack > 0 {
                if self.can_lead_bet() {
                    let min_bet = self.config.min_bet.max(self.config.big_blind);
                    if min_bet <= stack {
                        actions.push(Action::Bet(min_bet));
                    }
                    let pot = self.current_pot().max(1);
                    for amount in self.bet_sizes_from_fracs(pot, min_bet, stack) {
                        actions.push(Action::Bet(amount));
                    }
                }
                actions.push(Action::AllIn);
            }
        }

        dedup_actions(actions)
    }

    fn current_pot(&self) -> u64 {
        self.committed_total.iter().sum()
    }

    fn abstraction(&self) -> ActionAbstraction {
        self.config.action_abstraction.clone().unwrap_or_else(|| {
            self.config
                .tree_profile
                .action_abstraction(self.config.variant)
        })
    }

    fn position_class(&self, player: usize) -> PositionClass {
        let n = self.config.num_players.max(2);
        let sb = (self.dealer + 1) % n;
        let bb = (self.dealer + 2) % n;
        if player == sb || player == bb {
            return PositionClass::Blinds;
        }
        let rel = (player + n - self.dealer) % n;
        if rel <= n / 3 {
            PositionClass::Early
        } else if rel <= (2 * n) / 3 {
            PositionClass::Middle
        } else {
            PositionClass::Late
        }
    }

    fn can_lead_bet(&self) -> bool {
        if self.street == Street::Preflop {
            return true;
        }
        let abs = self.abstraction();
        if abs.allow_donk_bet {
            return true;
        }
        self.prev_street_aggressor
            .map(|agg| agg == self.to_act)
            .unwrap_or(true)
    }

    fn bet_sizes_from_fracs(&self, pot: u64, min_bet: u64, max_bet: u64) -> Vec<u64> {
        if min_bet > max_bet {
            return Vec::new();
        }
        let mut out = Vec::new();
        for frac in self.bet_size_fracs_for_current_street() {
            if frac <= 0.0 {
                continue;
            }
            let raw = (frac * pot as f64).ceil() as u64;
            let size = raw.max(min_bet).min(max_bet);
            out.push(size);
        }
        out.sort_unstable();
        out.dedup();
        if !self.abstraction().allow_overbet {
            let cap = pot.max(min_bet).min(max_bet);
            out.retain(|v| *v <= cap);
            if out.is_empty() && min_bet <= max_bet {
                out.push(min_bet);
            }
        }
        out
    }

    fn raise_to_sizes_from_fracs(&self, pot: u64, min_to: u64, max_to: u64) -> Vec<u64> {
        if min_to > max_to {
            return Vec::new();
        }
        let mut out = Vec::new();
        for frac in self.raise_size_fracs_for_current_street() {
            if frac <= 0.0 {
                continue;
            }
            let raise_by = (frac * pot as f64).ceil() as u64;
            let to_amount = self.current_bet.saturating_add(raise_by);
            let size = to_amount.max(min_to).min(max_to);
            out.push(size);
        }
        out.sort_unstable();
        out.dedup();
        if !self.abstraction().allow_overbet {
            let cap = self.current_bet.saturating_add(pot).min(max_to);
            out.retain(|v| *v <= cap);
            if out.is_empty() && min_to <= max_to {
                out.push(min_to);
            }
        }
        out
    }

    fn bet_size_fracs_for_current_street(&self) -> Vec<f64> {
        if self.street == Street::Preflop {
            let abs = self.abstraction();
            let pos = self.position_class(self.to_act);
            return match self.raises_in_round {
                0 => abs.preflop_open_sizes.for_position(pos).to_vec(),
                1 => abs.preflop_3bet_sizes.for_position(pos).to_vec(),
                2 => abs.preflop_4bet_sizes.for_position(pos).to_vec(),
                _ => abs.preflop_5bet_sizes.for_position(pos).to_vec(),
            };
        }
        if let Some(profile) = &self.config.bet_size_fracs_by_street {
            return profile.for_street(self.street).to_vec();
        }
        let (bet_profile, _) = self.config.tree_profile.defaults(self.config.variant);
        let by_street = bet_profile.for_street(self.street);
        if by_street.is_empty() {
            self.config.bet_size_fracs.clone()
        } else {
            by_street.to_vec()
        }
    }

    fn raise_size_fracs_for_current_street(&self) -> Vec<f64> {
        if self.street == Street::Preflop {
            let abs = self.abstraction();
            let pos = self.position_class(self.to_act);
            return match self.raises_in_round {
                0 => abs.preflop_open_sizes.for_position(pos).to_vec(),
                1 => abs.preflop_3bet_sizes.for_position(pos).to_vec(),
                2 => abs.preflop_4bet_sizes.for_position(pos).to_vec(),
                _ => abs.preflop_5bet_sizes.for_position(pos).to_vec(),
            };
        }
        if let Some(profile) = &self.config.raise_size_fracs_by_street {
            return profile.for_street(self.street).to_vec();
        }
        let (_, raise_profile) = self.config.tree_profile.defaults(self.config.variant);
        let by_street = raise_profile.for_street(self.street);
        if by_street.is_empty() {
            self.config.raise_size_fracs.clone()
        } else {
            by_street.to_vec()
        }
    }

    pub fn apply_action(&mut self, action: Action) -> Result<(), String> {
        if self.terminal {
            return Err("hand already terminal".to_string());
        }
        let p = self.to_act;
        if self.status[p] != PlayerStatus::Active {
            return Err("player to act is not active".to_string());
        }

        let facing = self.facing_amount(p);
        let prev_bet = self.current_bet;
        let mut is_raise = false;

        match action {
            Action::Fold => {
                if facing == 0 {
                    return Err("cannot fold when checking is available".to_string());
                }
                self.status[p] = PlayerStatus::Folded;
            }
            Action::Check => {
                if facing != 0 {
                    return Err("cannot check while facing a bet".to_string());
                }
            }
            Action::Call => {
                if facing == 0 {
                    return Err("cannot call without facing a bet".to_string());
                }
                self.pay(p, min(facing, self.stacks[p]));
            }
            Action::Bet(amount) => {
                if facing != 0 {
                    return Err("cannot bet while facing a bet".to_string());
                }
                if amount < self.config.min_bet {
                    return Err("bet below minimum".to_string());
                }
                if amount > self.stacks[p] {
                    return Err("bet exceeds stack".to_string());
                }
                self.pay(p, amount);
                self.current_bet = self.committed_round[p];
                self.last_full_raise = amount;
                self.street_aggressor = Some(p);
                is_raise = true;
            }
            Action::RaiseTo(to_amount) => {
                if facing == 0 {
                    return Err("raise requires facing a bet".to_string());
                }
                let max_to = self.committed_round[p] + self.stacks[p];
                if to_amount > max_to || to_amount <= self.current_bet {
                    return Err("invalid raise size".to_string());
                }
                let min_to = self.current_bet + self.last_full_raise.max(self.config.min_bet);
                if to_amount < min_to {
                    return Err("raise below minimum".to_string());
                }
                self.pay(p, to_amount - self.committed_round[p]);
                self.current_bet = to_amount;
                self.last_full_raise = to_amount - prev_bet;
                self.street_aggressor = Some(p);
                is_raise = true;
            }
            Action::AllIn => {
                if self.stacks[p] == 0 {
                    return Err("cannot all-in with zero stack".to_string());
                }
                let new_to = self.committed_round[p] + self.stacks[p];
                self.pay(p, self.stacks[p]);
                if new_to > self.current_bet {
                    let raise_size = new_to - self.current_bet;
                    self.current_bet = new_to;
                    if raise_size >= self.last_full_raise {
                        self.last_full_raise = raise_size;
                    }
                    self.street_aggressor = Some(p);
                    is_raise = true;
                }
            }
        }

        if self.active_players().len() <= 1 {
            self.terminal = true;
            self.street = Street::Showdown;
            return Ok(());
        }

        if is_raise {
            self.raises_in_round += 1;
        }

        if self.round_complete() {
            self.advance_street_or_showdown();
        } else {
            self.to_act = self.next_active_from((p + 1) % self.config.num_players);
        }
        Ok(())
    }

    pub fn side_pots(&self) -> Vec<SidePot> {
        let mut levels: Vec<u64> = self
            .committed_total
            .iter()
            .copied()
            .filter(|v| *v > 0)
            .collect();
        levels.sort_unstable();
        levels.dedup();

        let mut pots = Vec::new();
        let mut prev = 0_u64;
        for level in levels {
            let contributors = self.committed_total.iter().filter(|c| **c >= level).count() as u64;
            if contributors == 0 {
                continue;
            }
            let delta = level - prev;
            let amount = delta * contributors;
            if amount == 0 {
                prev = level;
                continue;
            }
            let eligible_players = self
                .status
                .iter()
                .enumerate()
                .filter_map(|(i, st)| {
                    if *st != PlayerStatus::Folded && self.committed_total[i] >= level {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            pots.push(SidePot {
                amount,
                eligible_players,
            });
            prev = level;
        }
        pots
    }

    fn post_blind(&mut self, player: usize, blind: u64) {
        let paid = min(blind, self.stacks[player]);
        self.stacks[player] -= paid;
        self.committed_round[player] += paid;
        self.committed_total[player] += paid;
        if self.stacks[player] == 0 {
            self.status[player] = PlayerStatus::AllIn;
        }
    }

    fn pay(&mut self, player: usize, amount: u64) {
        let paid = min(amount, self.stacks[player]);
        self.stacks[player] -= paid;
        self.committed_round[player] += paid;
        self.committed_total[player] += paid;
        if self.stacks[player] == 0 {
            self.status[player] = PlayerStatus::AllIn;
        }
    }

    fn can_raise(&self) -> bool {
        let by_global_cap = self
            .config
            .raise_cap
            .map(|cap| self.raises_in_round < cap)
            .unwrap_or(true);
        if !by_global_cap {
            return false;
        }
        let street_idx = match self.street {
            Street::Preflop => 0,
            Street::Flop => 1,
            Street::Turn => 2,
            Street::River | Street::Showdown => 3,
        };
        self.raises_in_round < self.abstraction().max_raises_per_street[street_idx]
    }

    fn next_active_from(&self, mut idx: usize) -> usize {
        for _ in 0..self.config.num_players {
            if self.status[idx] == PlayerStatus::Active {
                return idx;
            }
            idx = (idx + 1) % self.config.num_players;
        }
        idx
    }

    fn round_complete(&self) -> bool {
        for (i, st) in self.status.iter().enumerate() {
            if *st == PlayerStatus::Active && self.committed_round[i] != self.current_bet {
                return false;
            }
        }
        true
    }

    fn advance_street_or_showdown(&mut self) {
        self.committed_round.fill(0);
        self.current_bet = 0;
        self.last_full_raise = self.config.big_blind.max(self.config.min_bet);
        self.raises_in_round = 0;
        self.prev_street_aggressor = self.street_aggressor;
        self.street_aggressor = None;

        self.street = match self.street {
            Street::Preflop => Street::Flop,
            Street::Flop => Street::Turn,
            Street::Turn => Street::River,
            Street::River => Street::Showdown,
            Street::Showdown => Street::Showdown,
        };

        if self.street == Street::Showdown {
            self.terminal = true;
            return;
        }
        self.to_act = self.next_active_from((self.dealer + 1) % self.config.num_players);
    }
}

fn dedup_actions(mut actions: Vec<Action>) -> Vec<Action> {
    let mut folded = false;
    let mut checked = false;
    let mut called = false;
    let mut allin = false;
    let mut bets = Vec::new();
    let mut raises = Vec::new();

    for a in actions.drain(..) {
        match a {
            Action::Fold => folded = true,
            Action::Check => checked = true,
            Action::Call => called = true,
            Action::AllIn => allin = true,
            Action::Bet(v) => bets.push(v),
            Action::RaiseTo(v) => raises.push(v),
        }
    }
    bets.sort_unstable();
    bets.dedup();
    raises.sort_unstable();
    raises.dedup();

    let mut out = Vec::new();
    if folded {
        out.push(Action::Fold);
    }
    if checked {
        out.push(Action::Check);
    }
    if called {
        out.push(Action::Call);
    }
    for v in bets {
        out.push(Action::Bet(v));
    }
    for v in raises {
        out.push(Action::RaiseTo(v));
    }
    if allin {
        out.push(Action::AllIn);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_actions_include_fold_call_raise_when_facing_bet() {
        let cfg = GameConfig::no_limit(3, 100, 1, 2);
        let s = HandState::new(cfg, 0);
        let acts = s.legal_actions();
        assert!(acts.contains(&Action::Fold));
        assert!(acts.contains(&Action::Call));
        assert!(acts.iter().any(|a| matches!(a, Action::RaiseTo(_))));
    }

    #[test]
    fn apply_call_advances_turn_and_updates_commits() {
        let cfg = GameConfig::no_limit(3, 100, 1, 2);
        let mut s = HandState::new(cfg, 0);
        let p = s.to_act;
        let before_stack = s.stacks[p];
        s.apply_action(Action::Call).unwrap();
        assert_eq!(s.committed_round[p], 2);
        assert_eq!(s.stacks[p], before_stack - 2);
    }

    #[test]
    fn side_pots_three_way_allin_levels() {
        let cfg = GameConfig::no_limit(3, 0, 0, 0);
        let mut s = HandState::new(cfg, 0);
        s.committed_total = vec![100, 50, 25];
        s.status = vec![
            PlayerStatus::Active,
            PlayerStatus::Active,
            PlayerStatus::Active,
        ];
        let pots = s.side_pots();
        assert_eq!(pots.len(), 3);
        assert_eq!(pots[0].amount, 75);
        assert_eq!(pots[1].amount, 50);
        assert_eq!(pots[2].amount, 50);
    }

    #[test]
    fn legal_actions_include_multiple_bet_sizes_when_checked_to() {
        let mut cfg = GameConfig::no_limit(3, 100, 1, 2);
        cfg.bet_size_fracs = vec![0.5, 1.0, 2.0];
        cfg.tree_profile = TreeProfile::Wide;
        let mut s = HandState::new(cfg, 0);
        s.current_bet = 0;
        s.committed_round.fill(0);
        let acts = s.legal_actions();
        let bet_sizes = acts
            .into_iter()
            .filter_map(|a| match a {
                Action::Bet(v) => Some(v),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(bet_sizes.len() >= 2);
    }

    #[test]
    fn profile_is_variant_and_street_aware() {
        let mut cfg = GameConfig::no_limit(3, 100, 1, 2);
        cfg.variant = PokerVariant::ShortDeck;
        cfg.tree_profile = TreeProfile::Tight;
        let mut s = HandState::new(cfg, 0);
        s.street = Street::River;
        s.current_bet = 0;
        s.committed_round.fill(0);
        let river_bets = s
            .legal_actions()
            .into_iter()
            .filter_map(|a| match a {
                Action::Bet(v) => Some(v),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!river_bets.is_empty());
    }

    #[test]
    fn explicit_street_overrides_take_priority() {
        let mut cfg = GameConfig::no_limit(3, 100, 1, 2);
        cfg.bet_size_fracs_by_street = Some(StreetSizeProfile {
            preflop: vec![4.0],
            flop: vec![0.25],
            turn: vec![0.5],
            river: vec![1.0],
        });
        let mut s = HandState::new(cfg, 0);
        s.current_bet = 0;
        s.committed_round.fill(0);
        let preflop_bets = s
            .legal_actions()
            .into_iter()
            .filter_map(|a| match a {
                Action::Bet(v) => Some(v),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(preflop_bets.iter().all(|v| *v >= s.config.big_blind));
    }

    #[test]
    fn donk_bet_can_be_disabled() {
        let mut cfg = GameConfig::no_limit(3, 100, 1, 2);
        cfg.action_abstraction = Some(ActionAbstraction {
            preflop_open_sizes: PositionSizeProfile {
                early: vec![2.2],
                middle: vec![2.5],
                late: vec![2.8],
                blinds: vec![3.0],
            },
            preflop_3bet_sizes: PositionSizeProfile {
                early: vec![3.0],
                middle: vec![3.2],
                late: vec![3.5],
                blinds: vec![3.8],
            },
            preflop_4bet_sizes: PositionSizeProfile {
                early: vec![2.2],
                middle: vec![2.3],
                late: vec![2.4],
                blinds: vec![2.5],
            },
            preflop_5bet_sizes: PositionSizeProfile {
                early: vec![2.0],
                middle: vec![2.1],
                late: vec![2.2],
                blinds: vec![2.3],
            },
            allow_overbet: true,
            allow_donk_bet: false,
            max_raises_per_street: [3, 2, 2, 2],
        });
        let mut s = HandState::new(cfg, 0);
        s.street = Street::Flop;
        s.current_bet = 0;
        s.committed_round.fill(0);
        s.prev_street_aggressor = Some((s.to_act + 1) % s.config.num_players);
        let acts = s.legal_actions();
        assert!(!acts.iter().any(|a| matches!(a, Action::Bet(_))));
    }

    #[test]
    fn overbet_can_be_disabled() {
        let mut cfg = GameConfig::no_limit(3, 100, 1, 2);
        cfg.action_abstraction = Some(ActionAbstraction {
            preflop_open_sizes: PositionSizeProfile {
                early: vec![2.2],
                middle: vec![2.5],
                late: vec![2.8],
                blinds: vec![3.0],
            },
            preflop_3bet_sizes: PositionSizeProfile {
                early: vec![3.0],
                middle: vec![3.2],
                late: vec![3.5],
                blinds: vec![3.8],
            },
            preflop_4bet_sizes: PositionSizeProfile {
                early: vec![2.2],
                middle: vec![2.3],
                late: vec![2.4],
                blinds: vec![2.5],
            },
            preflop_5bet_sizes: PositionSizeProfile {
                early: vec![2.0],
                middle: vec![2.1],
                late: vec![2.2],
                blinds: vec![2.3],
            },
            allow_overbet: false,
            allow_donk_bet: true,
            max_raises_per_street: [3, 2, 2, 2],
        });
        cfg.bet_size_fracs_by_street = Some(StreetSizeProfile {
            preflop: vec![2.0],
            flop: vec![2.0],
            turn: vec![2.0],
            river: vec![2.0],
        });
        let mut s = HandState::new(cfg, 0);
        s.street = Street::Flop;
        s.current_bet = 0;
        s.committed_round.fill(0);
        let pot = s.committed_total.iter().sum::<u64>().max(1);
        let acts = s.legal_actions();
        let bets = acts
            .into_iter()
            .filter_map(|a| match a {
                Action::Bet(v) => Some(v),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(bets.iter().all(|v| *v <= pot));
    }

    #[test]
    fn preflop_5bet_sizes_are_used_after_four_raises() {
        let mut cfg = GameConfig::no_limit(3, 100, 1, 2);
        cfg.action_abstraction = Some(ActionAbstraction {
            preflop_open_sizes: PositionSizeProfile {
                early: vec![2.2],
                middle: vec![2.2],
                late: vec![2.2],
                blinds: vec![2.2],
            },
            preflop_3bet_sizes: PositionSizeProfile {
                early: vec![3.0],
                middle: vec![3.0],
                late: vec![3.0],
                blinds: vec![3.0],
            },
            preflop_4bet_sizes: PositionSizeProfile {
                early: vec![2.4],
                middle: vec![2.4],
                late: vec![2.4],
                blinds: vec![2.4],
            },
            preflop_5bet_sizes: PositionSizeProfile {
                early: vec![1.9],
                middle: vec![1.9],
                late: vec![1.9],
                blinds: vec![1.9],
            },
            allow_overbet: true,
            allow_donk_bet: true,
            max_raises_per_street: [6, 2, 2, 2],
        });
        let mut s = HandState::new(cfg, 0);
        s.street = Street::Preflop;
        s.raises_in_round = 3;
        let fracs = s.raise_size_fracs_for_current_street();
        assert_eq!(fracs, vec![1.9]);
    }
}
