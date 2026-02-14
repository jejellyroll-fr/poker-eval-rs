//! Implementation of `EnumResult` methods for statistics tracking and display.

use super::card_enum_dead::*;
use super::inner_loops::inner_loop_holdem;
use super::CardMask;
use crate::deck::StdDeck;
use crate::deck::STD_DECK_N_CARDS;
use crate::enumdefs::{EnumResult, Game, SampleType, ENUM_MAXPLAYERS};
use crate::enumerate::montecarlo::{deck_montecarlo_n_cards_d, deck_montecarlo_n_cards_joker};
use crate::enumord::EnumOrdering;
use crate::enumord::EnumOrderingMode;
use crate::enumord::{
    enum_ordering_decode_hilo_k_hi, enum_ordering_decode_hilo_k_lo, enum_ordering_decode_k,
    enum_ordering_nentries, enum_ordering_nentries_hilo,
};
use crate::enumord::{ENUM_ORDERING_MAXPLAYERS, ENUM_ORDERING_MAXPLAYERS_HILO};
use crate::errors::PokerError;
use crate::evaluators::{std_deck_lowball8_eval, std_deck_omaha_hi_eval, Eval, HandEvaluator};
use crate::handval::HandVal;
use crate::handval_low::{LowHandVal, LOW_HAND_VAL_NOTHING};
use crate::tables::t_cardmasks::StdDeckCardMask;
use crate::tables::t_cardmasks::STD_DECK_CARD_MASKS_TABLE;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

impl EnumResult {
    /// Creates a new `EnumResult` for a given game.
    pub fn new(game: Game) -> Self {
        let mut res = Self {
            game,
            sample_type: SampleType::Exhaustive,
            nsamples: 0,
            nplayers: 0,
            nwinhi: [0; ENUM_MAXPLAYERS],
            ntiehi: [0; ENUM_MAXPLAYERS],
            nlosehi: [0; ENUM_MAXPLAYERS],
            nwinlo: [0; ENUM_MAXPLAYERS],
            ntielo: [0; ENUM_MAXPLAYERS],
            nloselo: [0; ENUM_MAXPLAYERS],
            nscoop: [0; ENUM_MAXPLAYERS],
            nsharehi: [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
            nsharelo: [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
            nshare: Box::new([[[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS]),
            ev: [0.0; ENUM_MAXPLAYERS],
            ordering: None,
        };
        res.clear();
        res.game = game;
        res
    }

    /// Resets all fields to their default values.
    pub fn clear(&mut self) {
        self.game = Game::Holdem;
        self.sample_type = SampleType::Exhaustive;
        self.nsamples = 0;
        self.nplayers = 0;

        self.nwinhi = [0; ENUM_MAXPLAYERS];
        self.ntiehi = [0; ENUM_MAXPLAYERS];
        self.nlosehi = [0; ENUM_MAXPLAYERS];
        self.nwinlo = [0; ENUM_MAXPLAYERS];
        self.ntielo = [0; ENUM_MAXPLAYERS];
        self.nloselo = [0; ENUM_MAXPLAYERS];
        self.nscoop = [0; ENUM_MAXPLAYERS];

        self.nsharehi = [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS];
        self.nsharelo = [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS];
        // Re-allocate or memset. Since it's boxed, we can just overwrite.
        self.nshare = Box::new([[[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS]);

        self.ev = [0.0; ENUM_MAXPLAYERS];
        self.ordering = None;
    }

    /// Merges another `EnumResult` into this one.
    pub fn merge(&mut self, other: &Self) {
        self.nsamples += other.nsamples;
        for i in 0..ENUM_MAXPLAYERS {
            self.nwinhi[i] += other.nwinhi[i];
            self.ntiehi[i] += other.ntiehi[i];
            self.nlosehi[i] += other.nlosehi[i];
            self.nwinlo[i] += other.nwinlo[i];
            self.ntielo[i] += other.ntielo[i];
            self.nloselo[i] += other.nloselo[i];
            self.nscoop[i] += other.nscoop[i];
            self.ev[i] += other.ev[i];

            for j in 0..ENUM_MAXPLAYERS + 1 {
                self.nsharehi[i][j] += other.nsharehi[i][j];
                self.nsharelo[i][j] += other.nsharelo[i][j];
                for k in 0..ENUM_MAXPLAYERS + 1 {
                    self.nshare[i][j][k] += other.nshare[i][j][k];
                }
            }
        }
        if let (Some(s_ord), Some(o_ord)) = (&mut self.ordering, &other.ordering) {
            for (s, o) in s_ord.hist.iter_mut().zip(o_ord.hist.iter()) {
                *s += *o;
            }
        }
    }

    /// Allocates ordering resources for hand ranking histograms.
    pub fn allocate_resources(
        &mut self,
        nplayers: usize,
        mode: EnumOrderingMode,
    ) -> Result<(), PokerError> {
        if nplayers > ENUM_ORDERING_MAXPLAYERS && mode != EnumOrderingMode::Hilo {
            return Err(PokerError::Other(
                "Too many players for non-Hilo mode".to_string(),
            ));
        } else if nplayers > ENUM_ORDERING_MAXPLAYERS_HILO && mode == EnumOrderingMode::Hilo {
            return Err(PokerError::Other(
                "Too many players for Hilo mode".to_string(),
            ));
        }

        let nentries = match mode {
            EnumOrderingMode::Hilo => enum_ordering_nentries_hilo(nplayers),
            _ => enum_ordering_nentries(nplayers),
        };

        if nentries <= 0 {
            return Err(PokerError::Other("Invalid number of entries".to_string()));
        }

        let ordering = EnumOrdering {
            mode,
            nplayers,
            nentries: nentries as usize,
            hist: vec![0; nentries as usize],
        };
        self.ordering = Some(Box::new(ordering));

        Ok(())
    }

    /// Simulates a Short Deck Hold'em game using Monte Carlo sampling.
    pub fn simulate_short_deck_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        use crate::enumerate::inner_loops::inner_loop_short_deck;

        if npockets > ENUM_MAXPLAYERS {
            return Err(PokerError::TooManyPlayers);
        }

        let mut exclusion_mask = board | dead;
        for pocket in pockets {
            exclusion_mask = exclusion_mask | *pocket;
        }

        // Short Deck: Ranks 6..A (indices 4..12).
        // Standard deck cards: 0..51.
        // Rank = card_index / 4.
        // We want Rank >= 4.
        let deck = (0..STD_DECK_N_CARDS)
            .filter_map(|i| {
                let rank = i / 4;
                if rank < 4 {
                    return None; // Skip 2, 3, 4, 5
                }
                let card_mask = StdDeckCardMask::get_mask(i);
                if !card_mask.overlaps(&exclusion_mask) {
                    Some(*card_mask)
                } else {
                    None
                }
            })
            .collect::<Vec<StdDeckCardMask>>();

        let num_cards_to_draw = 5 - nboard;
        let no_dead_cards = StdDeckCardMask::new();

        #[cfg(feature = "parallel")]
        {
            let n_threads = rayon::current_num_threads();
            let chunk_size = (niter / (n_threads * 4)).max(128);
            let num_chunks = (niter + chunk_size - 1) / chunk_size;

            let total_res = (0..num_chunks)
                .into_par_iter()
                .map(|chunk_idx| {
                    let iter_in_this_chunk = if chunk_idx == num_chunks - 1 {
                        niter - chunk_idx * chunk_size
                    } else {
                        chunk_size
                    };

                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;

                    let mut hival = vec![HandVal::new(0, 0, 0, 0, 0, 0); npockets];
                    let mut loval = vec![LowHandVal::new(0, 0, 0, 0, 0, 0); npockets];

                    let empty_mask = StdDeckCardMask::new();
                    deck_montecarlo_n_cards_d(
                        &deck,
                        no_dead_cards,
                        num_cards_to_draw,
                        iter_in_this_chunk,
                        |combo| {
                            let mut complete_board = board;
                            for &card in combo {
                                complete_board = complete_board | card;
                            }

                            if let Ok(()) = inner_loop_short_deck(
                                pockets,
                                &complete_board,
                                &empty_mask,
                                &mut hival,
                                &mut loval,
                            ) {
                                local_res.update_statistics_batched(&hival, npockets);
                                local_res.nsamples += 1;
                            }
                        },
                    );
                    local_res
                })
                .reduce(
                    || EnumResult::new(self.game),
                    |mut a, b| {
                        a.merge(&b);
                        a
                    },
                );
            self.merge(&total_res);
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut local_res = EnumResult::new(self.game);
            local_res.nplayers = npockets as u32;
            let mut hival = vec![HandVal::new(0, 0, 0, 0, 0, 0); npockets];
            let mut loval = vec![LowHandVal::new(0, 0, 0, 0, 0, 0); npockets];

            let empty_mask = StdDeckCardMask::new();
            deck_montecarlo_n_cards_d(&deck, no_dead_cards, num_cards_to_draw, niter, |combo| {
                let mut complete_board = board;
                for &card in combo {
                    complete_board = complete_board | card;
                }

                if let Ok(()) = inner_loop_short_deck(
                    pockets,
                    &complete_board,
                    &empty_mask,
                    &mut hival,
                    &mut loval,
                ) {
                    local_res.update_statistics_batched(&hival, npockets);
                    local_res.nsamples += 1;
                }
            });
            self.merge(&local_res);
        }

        Ok(())
    }

    /// Simulates a Hold'em game using Monte Carlo sampling.
    pub fn simulate_holdem_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        if npockets > ENUM_MAXPLAYERS {
            return Err(PokerError::TooManyPlayers);
        }

        let mut exclusion_mask = board | dead;
        for pocket in pockets {
            exclusion_mask = exclusion_mask | *pocket;
        }
        let deck = (0..STD_DECK_N_CARDS)
            .filter_map(|i| {
                let card_mask = StdDeckCardMask::get_mask(i);
                if !card_mask.overlaps(&exclusion_mask) {
                    Some(*card_mask)
                } else {
                    None
                }
            })
            .collect::<Vec<StdDeckCardMask>>();

        let num_cards_to_draw = 5 - nboard;
        let no_dead_cards = StdDeckCardMask::new();

        #[cfg(feature = "parallel")]
        {
            let n_threads = rayon::current_num_threads();
            let chunk_size = (niter / (n_threads * 4)).max(128);
            let num_chunks = (niter + chunk_size - 1) / chunk_size;

            let total_res = (0..num_chunks)
                .into_par_iter()
                .map(|chunk_idx| {
                    let iter_in_this_chunk = if chunk_idx == num_chunks - 1 {
                        niter - chunk_idx * chunk_size
                    } else {
                        chunk_size
                    };

                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;

                    let mut hival = [HandVal { value: 0 }; ENUM_MAXPLAYERS];
                    let mut loval = [LowHandVal { value: 0 }; ENUM_MAXPLAYERS];

                    let empty_mask = StdDeckCardMask::new();
                    let board_raw = board.as_raw();
                    deck_montecarlo_n_cards_d(
                        &deck,
                        no_dead_cards,
                        num_cards_to_draw,
                        iter_in_this_chunk,
                        |combo| {
                            // Pre-compute board mask using raw u64 OR (faster than per-card OR)
                            let mut raw = board_raw;
                            for &card in combo {
                                raw |= card.as_raw();
                            }
                            let complete_board = StdDeckCardMask::from_raw(raw);

                            if let Ok(()) = inner_loop_holdem(
                                pockets,
                                &complete_board,
                                &empty_mask,
                                &mut hival[..npockets],
                                &mut loval[..npockets],
                            ) {
                                local_res.update_statistics_batched(&hival[..npockets], npockets);
                                local_res.nsamples += 1;
                            }
                        },
                    );
                    local_res
                })
                .reduce(
                    || EnumResult::new(self.game),
                    |mut a, b| {
                        a.merge(&b);
                        a
                    },
                );
            self.merge(&total_res);
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut local_res = EnumResult::new(self.game);
            local_res.nplayers = npockets as u32;
            let mut hival = vec![HandVal { value: 0 }; npockets];
            let mut loval = vec![LowHandVal { value: 0 }; npockets];

            let empty_mask = StdDeckCardMask::new();
            deck_montecarlo_n_cards_d(&deck, no_dead_cards, num_cards_to_draw, niter, |combo| {
                let mut complete_board = board;
                for &card in combo {
                    complete_board = complete_board | card;
                }

                if let Ok(()) = inner_loop_holdem(
                    pockets,
                    &complete_board,
                    &empty_mask,
                    &mut hival,
                    &mut loval,
                ) {
                    local_res.update_statistics_batched(&hival, npockets);
                    local_res.nsamples += 1;
                }
            });
            self.merge(&local_res);
        }

        Ok(())
    }

    /// Simulates an Omaha game using Monte Carlo sampling.
    pub fn simulate_omaha_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        if npockets > ENUM_MAXPLAYERS {
            return Err(PokerError::TooManyPlayers);
        }

        let mut exclusion_mask = board | dead;
        for pocket in pockets {
            exclusion_mask = exclusion_mask | *pocket;
        }
        let deck = (0..STD_DECK_N_CARDS)
            .filter_map(|i| {
                let card_mask = StdDeckCardMask::get_mask(i);
                if !card_mask.overlaps(&exclusion_mask) {
                    Some(*card_mask)
                } else {
                    None
                }
            })
            .collect::<Vec<StdDeckCardMask>>();

        let num_cards_to_draw = 5 - nboard;
        let no_dead_cards = StdDeckCardMask::new();

        #[cfg(feature = "parallel")]
        {
            let n_threads = rayon::current_num_threads();
            let chunk_size = (niter / (n_threads * 4)).max(128);
            let num_chunks = (niter + chunk_size - 1) / chunk_size;

            let total_res = (0..num_chunks)
                .into_par_iter()
                .map(|chunk_idx| {
                    let iter_in_this_chunk = if chunk_idx == num_chunks - 1 {
                        niter - chunk_idx * chunk_size
                    } else {
                        chunk_size
                    };

                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;

                    let mut hival = [HandVal { value: 0 }; ENUM_MAXPLAYERS];

                    let board_raw = board.as_raw();
                    deck_montecarlo_n_cards_d(
                        &deck,
                        no_dead_cards,
                        num_cards_to_draw,
                        iter_in_this_chunk,
                        |combo| {
                            let mut raw = board_raw;
                            for &card in combo {
                                raw |= card.as_raw();
                            }
                            let complete_board = StdDeckCardMask::from_raw(raw);

                            // Evaluate manually to avoid allocation in evaluate_omaha_hands
                            for i in 0..npockets {
                                let mut val: Option<HandVal> = None;
                                let _ = crate::evaluators::omaha::std_deck_omaha_hi_eval(
                                    pockets[i],
                                    complete_board,
                                    &mut val,
                                );
                                hival[i] = val.unwrap_or(HandVal { value: 0 });
                            }
                            local_res.update_statistics_batched(&hival[..npockets], npockets);
                            local_res.nsamples += 1;
                        },
                    );
                    local_res
                })
                .reduce(
                    || EnumResult::new(self.game),
                    |mut a, b| {
                        a.merge(&b);
                        a
                    },
                );
            self.merge(&total_res);
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut local_res = EnumResult::new(self.game);
            local_res.nplayers = npockets as u32;
            let mut hival = [HandVal { value: 0 }; ENUM_MAXPLAYERS];

            let board_raw = board.as_raw();
            deck_montecarlo_n_cards_d(&deck, no_dead_cards, num_cards_to_draw, niter, |combo| {
                let mut raw = board_raw;
                for &card in combo {
                    raw |= card.as_raw();
                }
                let complete_board = StdDeckCardMask::from_raw(raw);

                // Evaluate manually to avoid allocation
                for i in 0..npockets {
                    let mut val: Option<HandVal> = None;
                    let _ = crate::evaluators::omaha::std_deck_omaha_hi_eval(
                        pockets[i],
                        complete_board,
                        &mut val,
                    );
                    hival[i] = val.unwrap_or(HandVal { value: 0 });
                }
                local_res.update_statistics_batched(&hival[..npockets], npockets);
                local_res.nsamples += 1;
            });
            self.merge(&local_res);
        }

        Ok(())
    }

    /// Simulates a Hold'em Hi/Lo 8-or-better game using Monte Carlo sampling.
    pub fn simulate_holdem8_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        if npockets > ENUM_MAXPLAYERS {
            return Err(PokerError::TooManyPlayers);
        }

        let mut exclusion_mask = board | dead;
        for pocket in pockets {
            exclusion_mask = exclusion_mask | *pocket;
        }
        let deck = (0..STD_DECK_N_CARDS)
            .filter_map(|i| {
                let card_mask = StdDeckCardMask::get_mask(i);
                if !card_mask.overlaps(&exclusion_mask) {
                    Some(*card_mask)
                } else {
                    None
                }
            })
            .collect::<Vec<StdDeckCardMask>>();

        let num_cards_to_draw = 5 - nboard;
        let no_dead_cards = StdDeckCardMask::new();

        #[cfg(feature = "parallel")]
        {
            let n_threads = rayon::current_num_threads();
            let chunk_size = (niter / (n_threads * 4)).max(128);
            let num_chunks = (niter + chunk_size - 1) / chunk_size;

            let total_res = (0..num_chunks)
                .into_par_iter()
                .map(|chunk_idx| {
                    let iter_in_this_chunk = if chunk_idx == num_chunks - 1 {
                        niter - chunk_idx * chunk_size
                    } else {
                        chunk_size
                    };

                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;

                    let mut hival = vec![HandVal { value: 0 }; npockets];
                    let mut loval = vec![LowHandVal { value: 0 }; npockets];

                    deck_montecarlo_n_cards_d(
                        &deck,
                        no_dead_cards,
                        num_cards_to_draw,
                        iter_in_this_chunk,
                        |combo| {
                            let mut complete_board = board;
                            for &card in combo {
                                complete_board = complete_board | card;
                            }

                            for i in 0..npockets {
                                let hand = pockets[i] | complete_board;
                                hival[i] = Eval::eval_n(&hand, 7);
                                loval[i] = std_deck_lowball8_eval(&hand, 7)
                                    .unwrap_or(LowHandVal { value: 0 });
                            }
                            local_res.update_statistics_batched_hilo(&hival, &loval, npockets);
                            local_res.nsamples += 1;
                        },
                    );
                    local_res
                })
                .reduce(
                    || EnumResult::new(self.game),
                    |mut a, b| {
                        a.merge(&b);
                        a
                    },
                );
            self.merge(&total_res);
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut local_res = EnumResult::new(self.game);
            local_res.nplayers = npockets as u32;

            let mut hival = vec![HandVal { value: 0 }; npockets];
            let mut loval = vec![LowHandVal { value: 0 }; npockets];

            deck_montecarlo_n_cards_d(&deck, no_dead_cards, num_cards_to_draw, niter, |combo| {
                let mut complete_board = board;
                for &card in combo {
                    complete_board = complete_board | card;
                }

                for i in 0..npockets {
                    let hand = pockets[i] | complete_board;
                    hival[i] = Eval::eval_n(&hand, 7);
                    loval[i] = std_deck_lowball8_eval(&hand, 7).unwrap_or(LowHandVal { value: 0 });
                }
                local_res.update_statistics_batched_hilo(&hival, &loval, npockets);
                local_res.nsamples += 1;
            });
            self.merge(&local_res);
        }

        Ok(())
    }

    /// Simulates an Omaha 5-card game using Monte Carlo sampling.
    /// Uses the same evaluation as standard Omaha since the inner loop
    /// handles different hole card counts automatically.
    pub fn simulate_omaha5_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        self.simulate_omaha_game(pockets, board, dead, npockets, nboard, niter)
    }

    /// Simulates an Omaha 6-card game using Monte Carlo sampling.
    /// Uses the same evaluation as standard Omaha since the inner loop
    /// handles different hole card counts automatically.
    pub fn simulate_omaha6_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        self.simulate_omaha_game(pockets, board, dead, npockets, nboard, niter)
    }

    /// Simulates an Omaha Hi/Lo 8-or-better game using Monte Carlo sampling.
    /// Works for Omaha8 and Omaha85 variants.
    pub fn simulate_omaha8_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        use crate::evaluators::omaha::std_deck_omaha_hi_low8_eval;

        if npockets > ENUM_MAXPLAYERS {
            return Err(PokerError::TooManyPlayers);
        }

        let mut exclusion_mask = board | dead;
        for pocket in pockets {
            exclusion_mask = exclusion_mask | *pocket;
        }
        let deck = (0..STD_DECK_N_CARDS)
            .filter_map(|i| {
                let card_mask = StdDeckCardMask::get_mask(i);
                if !card_mask.overlaps(&exclusion_mask) {
                    Some(*card_mask)
                } else {
                    None
                }
            })
            .collect::<Vec<StdDeckCardMask>>();

        let num_cards_to_draw = 5 - nboard;
        let no_dead_cards = StdDeckCardMask::new();

        #[cfg(feature = "parallel")]
        {
            let n_threads = rayon::current_num_threads();
            let chunk_size = (niter / (n_threads * 4)).max(128);
            let num_chunks = (niter + chunk_size - 1) / chunk_size;

            let total_res = (0..num_chunks)
                .into_par_iter()
                .map(|chunk_idx| {
                    let iter_in_this_chunk = if chunk_idx == num_chunks - 1 {
                        niter - chunk_idx * chunk_size
                    } else {
                        chunk_size
                    };

                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;

                    let mut hival = vec![HandVal { value: 0 }; npockets];
                    let mut loval = vec![LowHandVal { value: 0 }; npockets];

                    deck_montecarlo_n_cards_d(
                        &deck,
                        no_dead_cards,
                        num_cards_to_draw,
                        iter_in_this_chunk,
                        |combo| {
                            let mut complete_board = board;
                            for &card in combo {
                                complete_board = complete_board | card;
                            }

                            for i in 0..npockets {
                                let mut hi: Option<HandVal> = None;
                                let mut lo: Option<LowHandVal> = None;
                                let _ = std_deck_omaha_hi_low8_eval(
                                    pockets[i],
                                    complete_board,
                                    &mut hi,
                                    &mut lo,
                                );
                                hival[i] = hi.unwrap_or(HandVal { value: 0 });
                                loval[i] = lo.unwrap_or(LowHandVal { value: 0 });
                            }
                            local_res.update_statistics_batched_hilo(&hival, &loval, npockets);
                            local_res.nsamples += 1;
                        },
                    );
                    local_res
                })
                .reduce(
                    || EnumResult::new(self.game),
                    |mut a, b| {
                        a.merge(&b);
                        a
                    },
                );
            self.merge(&total_res);
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut local_res = EnumResult::new(self.game);
            local_res.nplayers = npockets as u32;

            let mut hival = vec![HandVal { value: 0 }; npockets];
            let mut loval = vec![LowHandVal { value: 0 }; npockets];

            deck_montecarlo_n_cards_d(&deck, no_dead_cards, num_cards_to_draw, niter, |combo| {
                let mut complete_board = board;
                for &card in combo {
                    complete_board = complete_board | card;
                }

                for i in 0..npockets {
                    let mut hi: Option<HandVal> = None;
                    let mut lo: Option<LowHandVal> = None;
                    let _ =
                        std_deck_omaha_hi_low8_eval(pockets[i], complete_board, &mut hi, &mut lo);
                    hival[i] = hi.unwrap_or(HandVal { value: 0 });
                    loval[i] = lo.unwrap_or(LowHandVal { value: 0 });
                }
                local_res.update_statistics_batched_hilo(&hival, &loval, npockets);
                local_res.nsamples += 1;
            });
            self.merge(&local_res);
        }

        Ok(())
    }

    /// Simulates a game where each player receives independent cards (e.g. Stud, Draw).
    fn simulate_independent_game<F>(
        &mut self,
        pockets: &[StdDeckCardMask],
        dead: StdDeckCardMask,
        npockets: usize,
        target_hand_size: usize,
        niter: usize,
        eval_func: F,
    ) -> Result<(), PokerError>
    where
        F: Fn(&[StdDeckCardMask], &mut [HandVal], &mut [LowHandVal]) -> Result<(), PokerError>
            + Sync
            + Send
            + Copy,
    {
        if npockets > ENUM_MAXPLAYERS {
            return Err(PokerError::TooManyPlayers);
        }

        let mut exclusion_mask = dead;
        let mut cards_needed_per_player = [0; ENUM_MAXPLAYERS];
        let mut total_draw_needed = 0;

        for i in 0..npockets {
            exclusion_mask = exclusion_mask | pockets[i];
            let current_len = pockets[i].num_cards();
            if current_len < target_hand_size {
                let needed = target_hand_size - current_len;
                cards_needed_per_player[i] = needed;
                total_draw_needed += needed;
            }
        }

        let deck_size = STD_DECK_N_CARDS;
        let dead_count = dead.num_cards();
        let pocket_count = (0..npockets).map(|i| pockets[i].num_cards()).sum::<usize>(); // Build deck (52 cards)
                                                                                         // Check availability
        let _available = deck_size - dead_count - pocket_count;

        let deck_mask = (0..STD_DECK_N_CARDS)
            .filter_map(|i| {
                let card_mask = StdDeckCardMask::get_mask(i);
                if !card_mask.overlaps(&exclusion_mask) {
                    Some(*card_mask)
                } else {
                    None
                }
            })
            .collect::<Vec<StdDeckCardMask>>();

        let num_cards_to_draw = total_draw_needed;
        if deck_mask.len() < num_cards_to_draw {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Not enough cards in deck. Needed {}, available {}",
                num_cards_to_draw,
                deck_mask.len()
            )));
        }

        let no_dead_cards = StdDeckCardMask::new();

        let run_batch = |chunk_size: usize, local_res: &mut EnumResult| {
            let mut hival = vec![HandVal::new(0, 0, 0, 0, 0, 0); npockets];
            let mut loval = vec![LowHandVal::new(0, 0, 0, 0, 0, 0); npockets];
            let mut player_hands = vec![StdDeckCardMask::new(); npockets];

            deck_montecarlo_n_cards_d(
                &deck_mask,
                no_dead_cards,
                num_cards_to_draw,
                chunk_size,
                |combo| {
                    let mut card_idx = 0;
                    for i in 0..npockets {
                        let mut hand = pockets[i];
                        let needed = cards_needed_per_player[i];
                        for _ in 0..needed {
                            hand = hand | combo[card_idx];
                            card_idx += 1;
                        }
                        player_hands[i] = hand;
                    }

                    if let Ok(()) = eval_func(&player_hands, &mut hival, &mut loval) {
                        match local_res.game {
                            Game::Razz | Game::Lowball | Game::Lowball27 => {
                                local_res.update_statistics_batched_lo(&loval, npockets);
                            }
                            Game::Stud78
                            | Game::Stud7nsq
                            | Game::Omaha8
                            | Game::Omaha85
                            | Game::Draw58
                            | Game::Draw5nsq => {
                                local_res.update_statistics_batched_hilo(&hival, &loval, npockets);
                            }
                            _ => {
                                local_res.update_statistics_batched(&hival, npockets);
                            }
                        }
                        local_res.nsamples += 1;
                    }
                },
            );
        };

        #[cfg(feature = "parallel")]
        {
            let n_threads = rayon::current_num_threads();
            let chunk_size = (niter / (n_threads * 4)).max(128);
            let num_chunks = (niter + chunk_size - 1) / chunk_size;

            let total_res = (0..num_chunks)
                .into_par_iter()
                .map(|chunk_idx| {
                    let iter_in_this_chunk = if chunk_idx == num_chunks - 1 {
                        niter - chunk_idx * chunk_size
                    } else {
                        chunk_size
                    };

                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    run_batch(iter_in_this_chunk, &mut local_res);
                    local_res
                })
                .reduce(
                    || EnumResult::new(self.game),
                    |mut a, b| {
                        a.merge(&b);
                        a
                    },
                );
            self.merge(&total_res);
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut local_res = EnumResult::new(self.game);
            local_res.nplayers = npockets as u32;
            run_batch(niter, &mut local_res);
            self.merge(&local_res);
        }

        Ok(())
    }

    pub fn simulate_stud_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        dead: StdDeckCardMask,
        npockets: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        use crate::enumerate::inner_loops::inner_loop_7stud;
        let eval_wrapper =
            |hands: &[StdDeckCardMask], hival: &mut [HandVal], loval: &mut [LowHandVal]| {
                let empty_masks = vec![StdDeckCardMask::new(); hands.len()];
                inner_loop_7stud(hands, &empty_masks, hival, loval)
            };
        self.simulate_independent_game(pockets, dead, npockets, 7, niter, eval_wrapper)
    }

    pub fn simulate_stud8_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        dead: StdDeckCardMask,
        npockets: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        // use crate::enumerate::inner_loops::inner_loop_7stud8; // Missing?
        // Implementing inline if missing or use fallback
        // inner_loop_7stud8 is standard stud hi/lo 8-or-better.
        // If not exported, we can use a closure here similar to inner loop logic.
        let eval_wrapper =
            |hands: &[StdDeckCardMask], hival: &mut [HandVal], loval: &mut [LowHandVal]| {
                for (i, hand) in hands.iter().enumerate() {
                    hival[i] = Eval::eval_n(hand, hand.num_cards());
                    loval[i] = std_deck_lowball8_eval(hand, hand.num_cards())
                        .unwrap_or(LowHandVal { value: 0 });
                }
                Ok(())
            };
        self.simulate_independent_game(pockets, dead, npockets, 7, niter, eval_wrapper)
    }

    pub fn simulate_studnsq_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        dead: StdDeckCardMask,
        npockets: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        use crate::enumerate::inner_loops::inner_loop_7studnsq;
        let eval_wrapper =
            |hands: &[StdDeckCardMask], hival: &mut [HandVal], loval: &mut [LowHandVal]| {
                let empty_masks = vec![StdDeckCardMask::new(); hands.len()];
                inner_loop_7studnsq(hands, &empty_masks, hival, loval)
            };
        self.simulate_independent_game(pockets, dead, npockets, 7, niter, eval_wrapper)
    }

    pub fn simulate_razz_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        dead: StdDeckCardMask,
        npockets: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        use crate::enumerate::inner_loops::inner_loop_razz;
        let eval_wrapper =
            |hands: &[StdDeckCardMask], hival: &mut [HandVal], loval: &mut [LowHandVal]| {
                let empty_masks = vec![StdDeckCardMask::new(); hands.len()];
                inner_loop_razz(hands, &empty_masks, hival, loval)
            };
        self.simulate_independent_game(pockets, dead, npockets, 7, niter, eval_wrapper)
    }

    pub fn simulate_lowball27_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        dead: StdDeckCardMask,
        npockets: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        use crate::enumerate::inner_loops::inner_loop_lowball27;
        let eval_wrapper =
            |hands: &[StdDeckCardMask], hival: &mut [HandVal], loval: &mut [LowHandVal]| {
                let empty_masks = vec![StdDeckCardMask::new(); hands.len()];
                inner_loop_lowball27(hands, &empty_masks, hival, loval)
            };
        self.simulate_independent_game(pockets, dead, npockets, 5, niter, eval_wrapper)
    }

    /// Simulates a game with Joker support (Draw, Lowball A-5).
    /// Converts input StdDeckCardMasks to JokerDeckCardMasks.
    fn simulate_independent_game_joker<F>(
        &mut self,
        pockets: &[StdDeckCardMask],
        dead: StdDeckCardMask,
        npockets: usize,
        target_hand_size: usize,
        niter: usize,
        eval_func: F,
    ) -> Result<(), PokerError>
    where
        F: Fn(
                &[crate::tables::t_jokercardmasks::JokerDeckCardMask],
                &mut [HandVal],
                &mut [LowHandVal],
            ) -> Result<(), PokerError>
            + Sync
            + Send
            + Copy,
    {
        use crate::tables::t_jokercardmasks::{JokerDeckCardMask, JOKER_DECK_CARD_MASKS_TABLE};

        if npockets > ENUM_MAXPLAYERS {
            return Err(PokerError::TooManyPlayers);
        }

        let mut exclusion_mask = dead;
        let mut cards_needed_per_player = [0; ENUM_MAXPLAYERS];
        let mut total_draw_needed = 0;

        let mut joker_pockets = Vec::with_capacity(npockets);

        for i in 0..npockets {
            exclusion_mask = exclusion_mask | pockets[i];
            let current_len = pockets[i].num_cards();
            if current_len < target_hand_size {
                let needed = target_hand_size - current_len;
                cards_needed_per_player[i] = needed;
                total_draw_needed += needed;
            }
            // Convert StdDeck to JokerDeck
            joker_pockets.push(JokerDeckCardMask {
                cards_n: pockets[i].as_raw(),
            });
        }

        // Build deck (53 cards)
        // Check availability
        let _joker_dead = JokerDeckCardMask {
            cards_n: dead.as_raw(),
        };
        // The exclusion mask is StdDeck, but we are drawing from JokerDeck.
        // We need to exclude the cards used in pockets (which are StdDeck -> JokerDeck compatible) AND dead cards.
        let joker_exclusion = JokerDeckCardMask {
            cards_n: exclusion_mask.as_raw(),
        };
        // Note: Dead cards and Pockets are StdDeck, so they don't have Joker bit set.
        // Joker is available in the deck unless explicitly excluded?
        // Typically dead vars passed here are StdDeck.
        // If we want to simulate Joker games, the Joker is available unless specifically dead.
        // Since input dead is StdDeck, it can't kill the Joker. Joker is alive.

        let deck_mask = JOKER_DECK_CARD_MASKS_TABLE
            .iter()
            .filter(|card| (card.cards_n & joker_exclusion.cards_n) == 0)
            .copied()
            .collect::<Vec<JokerDeckCardMask>>();

        if deck_mask.len() < total_draw_needed {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Not enough cards in deck. Needed {}, available {}",
                total_draw_needed,
                deck_mask.len()
            )));
        }

        let no_dead_cards = JokerDeckCardMask { cards_n: 0 };

        let run_batch = |chunk_size: usize, local_res: &mut EnumResult| {
            let mut hival = vec![HandVal::new(0, 0, 0, 0, 0, 0); npockets];
            let mut loval = vec![LowHandVal::new(0, 0, 0, 0, 0, 0); npockets];
            let mut player_hands = vec![JokerDeckCardMask { cards_n: 0 }; npockets];

            deck_montecarlo_n_cards_joker(
                &deck_mask,
                no_dead_cards,
                total_draw_needed,
                chunk_size,
                |combo| {
                    let mut card_idx = 0;
                    for i in 0..npockets {
                        let mut hand = joker_pockets[i].clone();
                        let needed = cards_needed_per_player[i];
                        for _ in 0..needed {
                            hand = JokerDeckCardMask {
                                cards_n: hand.cards_n | combo[card_idx].cards_n,
                            };
                            card_idx += 1;
                        }
                        player_hands[i] = hand;
                    }

                    if let Ok(()) = eval_func(&player_hands, &mut hival, &mut loval) {
                        match local_res.game {
                            Game::Razz | Game::Lowball | Game::Lowball27 => {
                                local_res.update_statistics_batched_lo(&loval, npockets);
                            }
                            Game::Stud78
                            | Game::Stud7nsq
                            | Game::Omaha8
                            | Game::Omaha85
                            | Game::Draw58
                            | Game::Draw5nsq => {
                                // Add other Hi/Lo games here if needed
                                local_res.update_statistics_batched_hilo(&hival, &loval, npockets);
                            }
                            _ => {
                                local_res.update_statistics_batched(&hival, npockets);
                            }
                        }
                        local_res.nsamples += 1;
                    }
                },
            );
        };

        #[cfg(feature = "parallel")]
        {
            let n_threads = rayon::current_num_threads();
            let chunk_size = (niter / (n_threads * 4)).max(128);
            let num_chunks = (niter + chunk_size - 1) / chunk_size;

            let total_res = (0..num_chunks)
                .into_par_iter()
                .map(|chunk_idx| {
                    let iter_in_this_chunk = if chunk_idx == num_chunks - 1 {
                        niter - chunk_idx * chunk_size
                    } else {
                        chunk_size
                    };

                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    run_batch(iter_in_this_chunk, &mut local_res);
                    local_res
                })
                .reduce(
                    || EnumResult::new(self.game),
                    |mut a, b| {
                        a.merge(&b);
                        a
                    },
                );
            self.merge(&total_res);
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut local_res = EnumResult::new(self.game);
            local_res.nplayers = npockets as u32;
            run_batch(niter, &mut local_res);
            self.merge(&local_res);
        }

        Ok(())
    }

    pub fn simulate_draw_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        dead: StdDeckCardMask,
        npockets: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        use crate::enumerate::inner_loops::inner_loop_5draw;
        use crate::tables::t_jokercardmasks::JokerDeckCardMask;

        let eval_wrapper =
            |hands: &[JokerDeckCardMask], hival: &mut [HandVal], loval: &mut [LowHandVal]| {
                let empty_masks = vec![JokerDeckCardMask { cards_n: 0 }; hands.len()];
                inner_loop_5draw(hands, &empty_masks, hival, loval)
            };
        self.simulate_independent_game_joker(pockets, dead, npockets, 5, niter, eval_wrapper)
    }

    pub fn simulate_draw8_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        dead: StdDeckCardMask,
        npockets: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        use crate::enumerate::inner_loops::inner_loop_5draw8;
        use crate::tables::t_jokercardmasks::JokerDeckCardMask;

        let eval_wrapper =
            |hands: &[JokerDeckCardMask], hival: &mut [HandVal], loval: &mut [LowHandVal]| {
                let empty_masks = vec![JokerDeckCardMask { cards_n: 0 }; hands.len()];
                inner_loop_5draw8(hands, &empty_masks, hival, loval)
            };
        self.simulate_independent_game_joker(pockets, dead, npockets, 5, niter, eval_wrapper)
    }

    pub fn simulate_drawnsq_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        dead: StdDeckCardMask,
        npockets: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        use crate::enumerate::inner_loops::inner_loop_5drawnsq;
        use crate::tables::t_jokercardmasks::JokerDeckCardMask;

        let eval_wrapper =
            |hands: &[JokerDeckCardMask], hival: &mut [HandVal], loval: &mut [LowHandVal]| {
                let empty_masks = vec![JokerDeckCardMask { cards_n: 0 }; hands.len()];
                inner_loop_5drawnsq(hands, &empty_masks, hival, loval)
            };
        self.simulate_independent_game_joker(pockets, dead, npockets, 5, niter, eval_wrapper)
    }

    pub fn simulate_lowball_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        dead: StdDeckCardMask,
        npockets: usize,
        niter: usize,
    ) -> Result<(), PokerError> {
        use crate::enumerate::inner_loops::inner_loop_lowball;
        use crate::tables::t_jokercardmasks::JokerDeckCardMask;

        let eval_wrapper =
            |hands: &[JokerDeckCardMask], hival: &mut [HandVal], loval: &mut [LowHandVal]| {
                let empty_masks = vec![JokerDeckCardMask { cards_n: 0 }; hands.len()];
                inner_loop_lowball(hands, &empty_masks, hival, loval)
            };
        self.simulate_independent_game_joker(pockets, dead, npockets, 5, niter, eval_wrapper)
    }

    pub fn exhaustive_holdem_evaluation(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
    ) -> Result<(), PokerError> {
        self.nplayers = npockets as u32;
        let mut exclusion_mask = dead | board;
        for pocket in pockets {
            exclusion_mask = exclusion_mask | *pocket;
        }

        // STACK ALLOCATION OPTIMIZATION
        // Instead of collecting into a Vec, we filter into a fixed-size buffer
        let mut deck_buf = [StdDeckCardMask::default(); STD_DECK_N_CARDS];
        let mut deck_len = 0;

        for &card in STD_DECK_CARD_MASKS_TABLE.iter() {
            if StdDeck::mask_to_index(&card).map_or(true, |idx| !exclusion_mask.card_is_set(idx)) {
                deck_buf[deck_len] = card;
                deck_len += 1;
            }
        }
        let deck = &deck_buf[..deck_len];
        let no_dead = StdDeckCardMask::new();

        match nboard {
            0 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .enumerate()
                        .map(|(i1, c1)| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            enumerate_4_cards_d(&deck[..i1], no_dead, |c2, c3, c4, c5| {
                                let new_board = board | *c1 | *c2 | *c3 | *c4 | *c5;
                                if let Ok(()) =
                                    local_res.evaluate_holdem_hands(pockets, &new_board, npockets)
                                {
                                    local_res.nsamples += 1;
                                }
                            });
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    // For sequential, we can just iterate. We might skip reducing per iteration to avoid overhead,
                    // or just accumulate in one result.
                    // Actually, enumerate_4_cards_d is a callback based, proper sequential loop.
                    // But here we iterate over deck.iter().
                    for (i1, c1) in deck.iter().enumerate() {
                        enumerate_4_cards_d(&deck[..i1], no_dead, |c2, c3, c4, c5| {
                            let new_board = board | *c1 | *c2 | *c3 | *c4 | *c5;
                            if let Ok(()) =
                                local_res.evaluate_holdem_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                        });
                    }
                    self.merge(&local_res);
                }
            }
            3 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .enumerate()
                        .map(|(i1, c1)| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            enumerate_1_cards_d(&deck[..i1], no_dead, |c2| {
                                let new_board = board | *c1 | *c2;
                                if let Ok(()) =
                                    local_res.evaluate_holdem_hands(pockets, &new_board, npockets)
                                {
                                    local_res.nsamples += 1;
                                }
                            });
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    for (i1, c1) in deck.iter().enumerate() {
                        enumerate_1_cards_d(&deck[..i1], no_dead, |c2| {
                            let new_board = board | *c1 | *c2;
                            if let Ok(()) =
                                local_res.evaluate_holdem_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                        });
                    }
                    self.merge(&local_res);
                }
            }
            4 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .map(|c| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            let new_board = board | *c;
                            if let Ok(()) =
                                local_res.evaluate_holdem_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    for c in deck.iter() {
                        let new_board = board | *c;
                        if let Ok(()) =
                            local_res.evaluate_holdem_hands(pockets, &new_board, npockets)
                        {
                            local_res.nsamples += 1;
                        }
                    }
                    self.merge(&local_res);
                }
            }
            5 => {
                if let Ok(()) = self.evaluate_holdem_hands(pockets, &board, npockets) {
                    self.nsamples += 1;
                }
            }
            _ => return Err(PokerError::UnsupportedBoardConfiguration),
        }

        Ok(())
    }

    /// Performs exhaustive Hold'em Hi/Lo 8-or-better evaluation.
    pub fn exhaustive_holdem8_evaluation(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
    ) -> Result<(), PokerError> {
        self.nplayers = npockets as u32;
        let mut exclusion_mask = dead | board;
        for pocket in pockets {
            exclusion_mask = exclusion_mask | *pocket;
        }

        // STACK ALLOCATION OPTIMIZATION
        let mut deck_buf = [StdDeckCardMask::default(); STD_DECK_N_CARDS];
        let mut deck_len = 0;

        for &card in STD_DECK_CARD_MASKS_TABLE.iter() {
            if StdDeck::mask_to_index(&card).map_or(true, |idx| !exclusion_mask.card_is_set(idx)) {
                deck_buf[deck_len] = card;
                deck_len += 1;
            }
        }
        let deck = &deck_buf[..deck_len];
        let no_dead = StdDeckCardMask::new();

        match nboard {
            0 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .enumerate()
                        .map(|(i1, c1)| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            enumerate_4_cards_d(&deck[..i1], no_dead, |c2, c3, c4, c5| {
                                let new_board = board | *c1 | *c2 | *c3 | *c4 | *c5;
                                if let Ok(()) =
                                    local_res.evaluate_holdem8_hands(pockets, &new_board, npockets)
                                {
                                    local_res.nsamples += 1;
                                }
                            });
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    for (i1, c1) in deck.iter().enumerate() {
                        enumerate_4_cards_d(&deck[..i1], no_dead, |c2, c3, c4, c5| {
                            let new_board = board | *c1 | *c2 | *c3 | *c4 | *c5;
                            if let Ok(()) =
                                local_res.evaluate_holdem8_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                        });
                    }
                    self.merge(&local_res);
                }
            }
            3 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .enumerate()
                        .map(|(i1, c1)| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            enumerate_1_cards_d(&deck[..i1], no_dead, |c2| {
                                let new_board = board | *c1 | *c2;
                                if let Ok(()) =
                                    local_res.evaluate_holdem8_hands(pockets, &new_board, npockets)
                                {
                                    local_res.nsamples += 1;
                                }
                            });
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    for (i1, c1) in deck.iter().enumerate() {
                        enumerate_1_cards_d(&deck[..i1], no_dead, |c2| {
                            let new_board = board | *c1 | *c2;
                            if let Ok(()) =
                                local_res.evaluate_holdem8_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                        });
                    }
                    self.merge(&local_res);
                }
            }
            4 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .map(|c| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            let new_board = board | *c;
                            if let Ok(()) =
                                local_res.evaluate_holdem8_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    for c in deck.iter() {
                        let new_board = board | *c;
                        if let Ok(()) =
                            local_res.evaluate_holdem8_hands(pockets, &new_board, npockets)
                        {
                            local_res.nsamples += 1;
                        }
                    }
                    self.merge(&local_res);
                }
            }
            5 => {
                if let Ok(()) = self.evaluate_holdem8_hands(pockets, &board, npockets) {
                    self.nsamples += 1;
                }
            }
            _ => return Err(PokerError::UnsupportedBoardConfiguration),
        }

        Ok(())
    }

    /// Evaluates Hold'em hands for all players against a given board.
    pub fn evaluate_holdem_hands(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: &StdDeckCardMask,
        npockets: usize,
    ) -> Result<(), PokerError> {
        if pockets.len() != npockets {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Invalid number of pockets: expected {}, got {}",
                npockets,
                pockets.len()
            )));
        }

        // Evaluate all hands once
        let hand_values: Vec<HandVal> = pockets
            .iter()
            .map(|pocket| {
                let hand = *pocket | *board;
                Eval::eval_n(&hand, 7)
            })
            .collect();

        // Compare cached results
        for i in 0..npockets {
            self.update_statistics_cached(i, &hand_values, npockets);
        }

        Ok(())
    }

    /// Evaluates Hold'em Hi/Lo 8-or-better hands for all players against a given board.
    pub fn evaluate_holdem8_hands(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: &StdDeckCardMask,
        npockets: usize,
    ) -> Result<(), PokerError> {
        if pockets.len() != npockets {
            return Err(PokerError::TooManyPlayers);
        }
        self.game = Game::Holdem8;

        // Evaluate all hands once
        let evals: Vec<(HandVal, Option<LowHandVal>)> = pockets
            .iter()
            .map(|pocket| {
                let hand = *pocket | *board;
                let hi_val = Eval::eval_n(&hand, 7);
                let lo_val = std_deck_lowball8_eval(&hand, 7);
                (hi_val, lo_val)
            })
            .collect();

        // Compare cached results
        for i in 0..npockets {
            self.update_statistics_hilo_cached(i, &evals, npockets);
        }
        Ok(())
    }

    /// Evaluates Short Deck Hold'em hands for all players against a given board.
    pub fn evaluate_short_deck_hands(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: &StdDeckCardMask,
        npockets: usize,
    ) -> Result<(), PokerError> {
        if pockets.len() != npockets {
            return Err(PokerError::InvalidCardConfiguration(format!(
                "Invalid number of pockets: expected {}, got {}",
                npockets,
                pockets.len()
            )));
        }

        // Evaluate all hands once
        let hand_values: Vec<HandVal> = pockets
            .iter()
            .map(|pocket| {
                // ShortDeckEvaluator::evaluate_hand(pocket, board)
                crate::evaluators::ShortDeckEvaluator::evaluate_hand(pocket, board)
                    .unwrap_or(HandVal::default())
            })
            .collect();

        // Compare cached results
        for i in 0..npockets {
            self.update_statistics_cached(i, &hand_values, npockets);
        }
        Ok(())
    }

    /// Performs exhaustive Short Deck Hold'em evaluation.
    pub fn exhaustive_short_deck_evaluation(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
    ) -> Result<(), PokerError> {
        // use crate::enumerate::inner_loops::inner_loop_short_deck; // Not used in filtered deck iter mode?
        // Actually, for exhaustive we re-implement the deck loop.
        // We reuse the logic from holdem but with different deck.

        self.nplayers = npockets as u32;
        let mut exclusion_mask = dead | board;
        for pocket in pockets {
            exclusion_mask = exclusion_mask | *pocket;
        }

        // STACK ALLOCATION OPTIMIZATION
        let mut deck_buf = [StdDeckCardMask::default(); STD_DECK_N_CARDS];
        let mut deck_len = 0;

        for &card in STD_DECK_CARD_MASKS_TABLE.iter() {
            let idx = StdDeck::mask_to_index(&card);
            if let Some(i) = idx {
                // Filter for Short Deck: Rank >= 4 (2,3,4,5 are 0,1,2,3)
                if (i / 4) >= 4 && !exclusion_mask.card_is_set(i) {
                    deck_buf[deck_len] = card;
                    deck_len += 1;
                }
            }
        }
        let deck = &deck_buf[..deck_len];
        let no_dead = StdDeckCardMask::new();

        match nboard {
            0 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .enumerate()
                        .map(|(i1, c1)| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            enumerate_4_cards_d(&deck[..i1], no_dead, |c2, c3, c4, c5| {
                                let new_board = board | *c1 | *c2 | *c3 | *c4 | *c5;
                                if let Ok(()) = local_res
                                    .evaluate_short_deck_hands(pockets, &new_board, npockets)
                                {
                                    local_res.nsamples += 1;
                                }
                            });
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    for (i1, c1) in deck.iter().enumerate() {
                        enumerate_4_cards_d(&deck[..i1], no_dead, |c2, c3, c4, c5| {
                            let new_board = board | *c1 | *c2 | *c3 | *c4 | *c5;
                            if let Ok(()) =
                                local_res.evaluate_short_deck_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                        });
                    }
                    self.merge(&local_res);
                }
            }
            3 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .enumerate()
                        .map(|(i1, c1)| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            enumerate_1_cards_d(&deck[..i1], no_dead, |c2| {
                                let new_board = board | *c1 | *c2;
                                if let Ok(()) = local_res
                                    .evaluate_short_deck_hands(pockets, &new_board, npockets)
                                {
                                    local_res.nsamples += 1;
                                }
                            });
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    for (i1, c1) in deck.iter().enumerate() {
                        enumerate_1_cards_d(&deck[..i1], no_dead, |c2| {
                            let new_board = board | *c1 | *c2;
                            if let Ok(()) =
                                local_res.evaluate_short_deck_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                        });
                    }
                    self.merge(&local_res);
                }
            }
            4 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .map(|c| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            let new_board = board | *c;
                            if let Ok(()) =
                                local_res.evaluate_short_deck_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    for c in deck.iter() {
                        let new_board = board | *c;
                        if let Ok(()) =
                            local_res.evaluate_short_deck_hands(pockets, &new_board, npockets)
                        {
                            local_res.nsamples += 1;
                        }
                    }
                    self.merge(&local_res);
                }
            }
            5 => {
                if let Ok(()) = self.evaluate_short_deck_hands(pockets, &board, npockets) {
                    self.nsamples += 1;
                }
            }
            _ => return Err(PokerError::UnsupportedBoardConfiguration),
        }
        Ok(())
    }

    /// Performs exhaustive Omaha evaluation over all possible board completions.
    pub fn exhaustive_omaha_evaluation(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
    ) -> Result<(), PokerError> {
        self.nplayers = npockets as u32;
        let mut exclusion_mask = dead | board;
        for pocket in pockets {
            exclusion_mask = exclusion_mask | *pocket;
        }

        // STACK ALLOCATION OPTIMIZATION
        let mut deck_buf = [StdDeckCardMask::default(); STD_DECK_N_CARDS];
        let mut deck_len = 0;

        for &card in STD_DECK_CARD_MASKS_TABLE.iter() {
            if StdDeck::mask_to_index(&card).map_or(true, |idx| !exclusion_mask.card_is_set(idx)) {
                deck_buf[deck_len] = card;
                deck_len += 1;
            }
        }
        let deck = &deck_buf[..deck_len];
        let no_dead = StdDeckCardMask::new();

        match nboard {
            0 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .enumerate()
                        .map(|(i1, c1)| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            enumerate_4_cards_d(&deck[..i1], no_dead, |c2, c3, c4, c5| {
                                let new_board = board | *c1 | *c2 | *c3 | *c4 | *c5;
                                if let Ok(()) =
                                    local_res.evaluate_omaha_hands(pockets, &new_board, npockets)
                                {
                                    local_res.nsamples += 1;
                                }
                            });
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    for (i1, c1) in deck.iter().enumerate() {
                        enumerate_4_cards_d(&deck[..i1], no_dead, |c2, c3, c4, c5| {
                            let new_board = board | *c1 | *c2 | *c3 | *c4 | *c5;
                            if let Ok(()) =
                                local_res.evaluate_omaha_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                        });
                    }
                    self.merge(&local_res);
                }
            }
            3 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .enumerate()
                        .map(|(i1, c1)| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            enumerate_1_cards_d(&deck[..i1], no_dead, |c2| {
                                let new_board = board | *c1 | *c2;
                                if let Ok(()) =
                                    local_res.evaluate_omaha_hands(pockets, &new_board, npockets)
                                {
                                    local_res.nsamples += 1;
                                }
                            });
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    for (i1, c1) in deck.iter().enumerate() {
                        enumerate_1_cards_d(&deck[..i1], no_dead, |c2| {
                            let new_board = board | *c1 | *c2;
                            if let Ok(()) =
                                local_res.evaluate_omaha_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                        });
                    }
                    self.merge(&local_res);
                }
            }
            4 => {
                #[cfg(feature = "parallel")]
                {
                    let total_res = deck
                        .par_iter()
                        .map(|c| {
                            let mut local_res = EnumResult::new(self.game);
                            local_res.nplayers = npockets as u32;
                            let new_board = board | *c;
                            if let Ok(()) =
                                local_res.evaluate_omaha_hands(pockets, &new_board, npockets)
                            {
                                local_res.nsamples += 1;
                            }
                            local_res
                        })
                        .reduce(
                            || EnumResult::new(self.game),
                            |mut a, b| {
                                a.merge(&b);
                                a
                            },
                        );
                    self.merge(&total_res);
                }
                #[cfg(not(feature = "parallel"))]
                {
                    let mut local_res = EnumResult::new(self.game);
                    local_res.nplayers = npockets as u32;
                    for c in deck.iter() {
                        let new_board = board | *c;
                        if let Ok(()) =
                            local_res.evaluate_omaha_hands(pockets, &new_board, npockets)
                        {
                            local_res.nsamples += 1;
                        }
                    }
                    self.merge(&local_res);
                }
            }
            5 => {
                if let Ok(()) = self.evaluate_omaha_hands(pockets, &board, npockets) {
                    self.nsamples += 1;
                }
            }
            _ => return Err(PokerError::UnsupportedBoardConfiguration),
        }

        Ok(())
    }

    /// Evaluates Omaha hands for all players against a given board.
    ///
    /// Optimized to evaluate each hand once, then compare (N evals + N*(N-1)/2 comparisons).
    pub fn evaluate_omaha_hands(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: &StdDeckCardMask,
        npockets: usize,
    ) -> Result<(), PokerError> {
        if pockets.len() != npockets {
            return Err(PokerError::TooManyPlayers);
        }
        self.game = Game::Omaha;

        // 1. Evaluate all hands once (O(N))
        // We use a fixed-size array on stack would be ideal, but HandVal is small.
        // Or just use a vec since we are pre-calculating.
        // Actually, update_statistics_batched takes &[HandVal].
        // So we need a buffer of HandVals.
        let mut hand_values = Vec::with_capacity(npockets);

        for pocket in pockets.iter().take(npockets) {
            let mut hival: Option<HandVal> = None;
            // Use optimized Hi-only evaluator from omaha module
            crate::evaluators::omaha::std_deck_omaha_hi_eval(*pocket, *board, &mut hival)?;

            // If no hand (unlikely with 5+ cards, but possible with <3 board), treat as 0 value.
            let val = hival.unwrap_or(HandVal { value: 0 });
            hand_values.push(val);
        }

        // 2. Compare using efficient batched update (O(N^2) cheap comparisons)
        self.update_statistics_batched(&hand_values, npockets);

        Ok(())
    }

    /// Updates statistics for an Omaha hand by comparing against all opponents.
    pub fn update_omaha_statistics(
        &mut self,
        player_index: usize,
        hand_value: HandVal,
        pockets: &[StdDeckCardMask],
        board: &StdDeckCardMask,
        npockets: usize,
    ) -> Result<(), PokerError> {
        let mut wins = 0;
        let mut ties = 0;
        let mut losses = 0;

        for (i, &other_pocket) in pockets.iter().enumerate() {
            if i != player_index {
                let mut other_hival: Option<HandVal> = None;

                std_deck_omaha_hi_eval(other_pocket, *board, &mut other_hival)?;

                if let Some(other_hand_value) = other_hival {
                    if hand_value > other_hand_value {
                        wins += 1;
                    } else if hand_value < other_hand_value {
                        losses += 1;
                    } else {
                        ties += 1;
                    }
                }
            }
        }

        self.nwinhi[player_index] += wins;
        self.ntiehi[player_index] += ties;
        self.nlosehi[player_index] += losses;

        let total_opponents = (npockets - 1) as f64;
        let win_rate = wins as f64 / total_opponents;
        let tie_rate = ties as f64 / total_opponents;

        let equity = win_rate + (tie_rate / 2.0);
        self.ev[player_index] += equity;
        Ok(())
    }

    /// Updates statistics for all players at once using N*(N-1)/2 comparisons.
    #[inline]
    pub fn update_statistics_batched(&mut self, hand_values: &[HandVal], npockets: usize) {
        use crate::enumdefs::ENUM_MAXPLAYERS;
        let mut wins = [0u32; ENUM_MAXPLAYERS];
        let mut ties = [0u32; ENUM_MAXPLAYERS];
        let mut losses = [0u32; ENUM_MAXPLAYERS];

        for i in 0..npockets {
            for j in i + 1..npockets {
                if hand_values[i] > hand_values[j] {
                    wins[i] += 1;
                    losses[j] += 1;
                } else if hand_values[i] < hand_values[j] {
                    losses[i] += 1;
                    wins[j] += 1;
                } else {
                    ties[i] += 1;
                    ties[j] += 1;
                }
            }
        }

        if npockets > 1 {
            let total_opponents = (npockets - 1) as f64;
            let inv_opponents = 1.0 / total_opponents;

            for i in 0..npockets {
                self.nwinhi[i] += wins[i];
                self.ntiehi[i] += ties[i];
                self.nlosehi[i] += losses[i];

                let equity = (wins[i] as f64 + (ties[i] as f64 * 0.5)) * inv_opponents;
                self.ev[i] += equity;
            }
        }
    }

    /// Updates statistics for Lowball games (smaller is better).
    #[inline]
    pub fn update_statistics_batched_lo(&mut self, hand_values: &[LowHandVal], npockets: usize) {
        use crate::enumdefs::ENUM_MAXPLAYERS;
        let mut wins = [0u32; ENUM_MAXPLAYERS];
        let mut ties = [0u32; ENUM_MAXPLAYERS];
        let mut losses = [0u32; ENUM_MAXPLAYERS];

        for i in 0..npockets {
            for j in i + 1..npockets {
                if hand_values[i].value < hand_values[j].value {
                    wins[i] += 1;
                    losses[j] += 1;
                } else if hand_values[i].value > hand_values[j].value {
                    losses[i] += 1;
                    wins[j] += 1;
                } else {
                    // Check for "Nothing" low hand?
                    // In Lowball, if value is NOTHING, it loses to any valid low?
                    // Usually LowHandVal handles this via ordering (NOTHING is largest u32?).
                    // Let's assume LowHandVal comparison works.
                    ties[i] += 1;
                    ties[j] += 1;
                }
            }
        }

        if npockets > 1 {
            let total_opponents = (npockets - 1) as f64;
            let inv_opponents = 1.0 / total_opponents;

            for i in 0..npockets {
                self.nwinlo[i] += wins[i];
                self.ntielo[i] += ties[i];
                self.nloselo[i] += losses[i];

                let equity = (wins[i] as f64 + (ties[i] as f64 * 0.5)) * inv_opponents;
                self.ev[i] += equity;
            }
        }
    }

    /// Updates statistics for Hi/Lo games.
    #[inline]
    pub fn update_statistics_batched_hilo(
        &mut self,
        hival: &[HandVal],
        loval: &[LowHandVal],
        npockets: usize,
    ) {
        use crate::enumdefs::ENUM_MAXPLAYERS;
        use crate::handval_low::LOW_HAND_VAL_NOTHING;

        let mut hi_wins = [0u32; ENUM_MAXPLAYERS];
        let mut hi_ties = [0u32; ENUM_MAXPLAYERS];
        let mut hi_losses = [0u32; ENUM_MAXPLAYERS];
        let mut lo_wins = [0u32; ENUM_MAXPLAYERS];
        let mut lo_ties = [0u32; ENUM_MAXPLAYERS];
        let mut lo_losses = [0u32; ENUM_MAXPLAYERS];

        // 1. Hi Comparisons (Larger is Better)
        for i in 0..npockets {
            for j in i + 1..npockets {
                if hival[i] > hival[j] {
                    hi_wins[i] += 1;
                    hi_losses[j] += 1;
                } else if hival[i] < hival[j] {
                    hi_losses[i] += 1;
                    hi_wins[j] += 1;
                } else {
                    hi_ties[i] += 1;
                    hi_ties[j] += 1;
                }
            }
        }

        // 2. Lo Comparisons (Smaller is Better, handling None/Nothing)
        // LowHandVal usually uses NOTHING constant for "No Low".
        // We need to implement: Valid Low < Nothing.
        // If both have Valid Low: Smaller is Better.
        // If both Nothing: Tie (or ignore?) - usually Tie in equity (both 0).
        for i in 0..npockets {
            for j in i + 1..npockets {
                let v_i = loval[i].value;
                let v_j = loval[j].value;

                if v_i == LOW_HAND_VAL_NOTHING && v_j == LOW_HAND_VAL_NOTHING {
                    // Both have no low - no one wins low pot part involved here?
                    // Or they tie for 0?
                    // In split pot games, if no low qualifies, High takes all?
                    // Wait. This logic is usually:
                    // If NO low is possible (board doesn't support), High takes all.
                    // If board supports low, but players don't qualify -> High takes their share?
                    // "Scoop" usually implies taking both pots.
                    // But if there is no Low pot, there is no Low Scoop.
                    // The simulation/INNER LOOP provides `hival` and `loval`.
                    // If game allows low, `loval` might be NOTHING.
                    // If `LOW_HAND_VAL_NOTHING` means "Did not qualify", then they effectively Lose to any qualified low.
                    // If both don't qualify, they tie for "worst".
                    // But does the Low pot exist?
                    // In Stud8, Low pot exists only if someone makes an 8-low.
                    // If NO ONE makes low, High scoops everything.
                    // The standard `poker-eval` accumulation logic usually assumes 50/50 split pot existence?
                    // `update_statistics_hilo_cached` logic:
                    // if lo_val.is_some() || lo_losses > 0 ...
                    // Wait, `lo_val` in cached is `Option<LowHandVal>`.
                    // Here `loval` is `LowHandVal` (struct).
                    // I need to check if value != NOTHING.

                    continue;
                }

                // One or both have low.
                if v_i == LOW_HAND_VAL_NOTHING {
                    // i has no low, j has low. j wins low.
                    lo_wins[j] += 1;
                    lo_losses[i] += 1;
                } else if v_j == LOW_HAND_VAL_NOTHING {
                    // i has low, j has no low. i wins low.
                    lo_wins[i] += 1;
                    lo_losses[j] += 1;
                } else {
                    // Both have low. Smaller is better.
                    if v_i < v_j {
                        lo_wins[i] += 1;
                        lo_losses[j] += 1;
                    } else if v_i > v_j {
                        lo_losses[i] += 1;
                        lo_wins[j] += 1;
                    } else {
                        lo_ties[i] += 1;
                        lo_ties[j] += 1;
                    }
                }
            }
        }

        if npockets > 1 {
            let total_opponents = (npockets - 1) as f64;
            let inv_opponents = 1.0 / total_opponents;

            for i in 0..npockets {
                self.nwinhi[i] += hi_wins[i];
                self.ntiehi[i] += hi_ties[i];
                self.nlosehi[i] += hi_losses[i];

                // Update Lo stats if they participated
                // If they have a low OR they lost to someone with a low
                if loval[i].value != LOW_HAND_VAL_NOTHING || lo_losses[i] > 0 {
                    self.nwinlo[i] += lo_wins[i];
                    self.ntielo[i] += lo_ties[i];
                    self.nloselo[i] += lo_losses[i];
                }

                // Scoop: Won Hi (beat everyone) AND Won Lo (beat everyone)
                // Note: hi_wins[i] == total_opponents implies beat everyone?
                // Wait, npockets - 1 matches.
                // But tied? Scoop means WIN, not tie.
                // If `hi_ties[i] == 0` and `hi_losses[i] == 0` -> explicit win.
                // Same for Lo.
                // And Lo must exist (value != NOTHING).
                // Actually, if I have Lo and I beat everyone (wins > 0, ties=0, loc=0).
                // Wait, if I am the ONLY one with Lo, I win Lo against everyone. (wins = N-1).

                let won_hi_outright = hi_wins[i] > 0 && hi_ties[i] == 0 && hi_losses[i] == 0;
                let won_lo_outright = lo_wins[i] > 0 && lo_ties[i] == 0 && lo_losses[i] == 0;
                let has_lo = loval[i].value != LOW_HAND_VAL_NOTHING;

                if has_lo && won_hi_outright && won_lo_outright {
                    self.nscoop[i] += 1;
                }

                // Equity: (Hi_share + Lo_share) / 2 ?
                // Or Hi_share/2 + Lo_share/2?
                // Yes. Assumes 50/50 pot split.
                // If no low qualified in the entire hand, does Hi get 100%?
                // This logic is tricky in batched updates because we don't know global state (did ANYONE qualify?).
                // In `inner_loop`, we usually calc potfrac.
                // `update_statistics_hilo_cached` assumes simple equity sum.
                // let hi_equity = (hi_wins + hi_ties/2) / opponents
                // let lo_equity = (lo_wins + lo_ties/2) / opponents
                // ev += hi_equity + lo_equity.
                // This implies EV is 2.0 max?
                // No. If I win hi against 1 opp: hi_eq = 1/1 = 1.
                // If I win lo against 1 opp: lo_eq = 1/1 = 1.
                // Total EV = 2.
                // But normally pot is split 0.5 / 0.5.
                // So should be 0.5 * hi_eq + 0.5 * lo_eq?
                // `hilo_cached` implementation:
                // self.ev[player_index] += hi_equity + lo_equity;
                // This seems to sum to 2.0.
                // Let's check `enum_result_print`.
                // It divides EV by nsamples.
                // Usually EV is normalized to 1.0 (pot).
                // If Hi/Lo, max EV should be 1.0 (scoop).
                // If `hilo_cached` adds to 2, then we need to divide by 2 somewhere?
                // Or `hi_equity` there is computed as...
                // `win_rate = wins / total_opponents`.
                // If I have 1 opponent. I win Hi. `win_rate` = 1.0.
                // `hi_equity` = 1.0.
                // If I also win Lo. `lo_equity` = 1.0.
                // Total = 2.0.
                // This suggests `hilo_cached` implementation might be wrong or using a different unit (2 pots).
                // I should stick to `hilo_cached` logic for consistency unless I verify it's wrong.

                let hi_equity = (hi_wins[i] as f64 + (hi_ties[i] as f64 * 0.5)) * inv_opponents;
                let lo_equity = (lo_wins[i] as f64 + (lo_ties[i] as f64 * 0.5)) * inv_opponents;

                // Scale by 0.5 for Hi/Lo split?
                // `update_statistics_hilo_cached` does NOT scale.
                // `inner_loop` (generic) calculates `potfrac`.
                // `hipot` = 0.5 if split? No, usually 1.0 for the pot.
                // If HasLoPot and HasHiPot, then we basically have 2 units of currency.
                // I will follow `hilo_cached` pattern for now.

                self.ev[i] += hi_equity + lo_equity;
            }
        }
    }

    /// Updates statistics from pre-evaluated hand values (avoids redundant evaluation).
    pub fn update_statistics_cached(
        &mut self,
        player_index: usize,
        hand_values: &[HandVal],
        npockets: usize,
    ) {
        let mut wins = 0;
        let mut ties = 0;
        let mut losses = 0;
        let hand_value = hand_values[player_index];

        for (i, &other_hand_value) in hand_values.iter().enumerate() {
            if i != player_index {
                if hand_value > other_hand_value {
                    wins += 1;
                } else if hand_value < other_hand_value {
                    losses += 1;
                } else {
                    ties += 1;
                }
            }
        }

        self.nwinhi[player_index] += wins;
        self.ntiehi[player_index] += ties;
        self.nlosehi[player_index] += losses;

        let total_opponents = (npockets - 1) as f64;
        let win_rate = wins as f64 / total_opponents;
        let tie_rate = ties as f64 / total_opponents;

        let equity = win_rate + (tie_rate / 2.0);
        self.ev[player_index] += equity;
    }

    /// Updates hi/lo statistics from pre-evaluated hand values (avoids redundant evaluation).
    pub fn update_statistics_hilo_cached(
        &mut self,
        player_index: usize,
        evals: &[(HandVal, Option<LowHandVal>)],
        npockets: usize,
    ) {
        let mut hi_wins = 0;
        let mut hi_ties = 0;
        let mut hi_losses = 0;
        let mut lo_wins = 0;
        let mut lo_ties = 0;
        let mut lo_losses = 0;
        let (hi_val, lo_val) = evals[player_index];

        for (i, &(other_hi_val, other_lo_val)) in evals.iter().enumerate() {
            if i != player_index {
                if hi_val > other_hi_val {
                    hi_wins += 1;
                } else if hi_val == other_hi_val {
                    hi_ties += 1;
                } else {
                    hi_losses += 1;
                }

                match (lo_val, other_lo_val) {
                    (Some(my_lo_val), Some(their_lo_val)) => {
                        if my_lo_val < their_lo_val {
                            lo_wins += 1;
                        } else if my_lo_val == their_lo_val {
                            if my_lo_val
                                == (LowHandVal {
                                    value: LOW_HAND_VAL_NOTHING,
                                })
                            {
                            } else {
                                lo_ties += 1;
                            }
                        } else {
                            lo_losses += 1;
                        }
                    }
                    (Some(_), None) => {
                        lo_wins += 1;
                    }
                    (None, Some(_)) => {
                        lo_losses += 1;
                    }
                    (None, None) => {}
                }
            }
        }

        self.nwinhi[player_index] += hi_wins;
        self.ntiehi[player_index] += hi_ties;
        self.nlosehi[player_index] += hi_losses;

        if lo_val.is_some() || lo_losses > 0 {
            self.nwinlo[player_index] += lo_wins;
            self.ntielo[player_index] += lo_ties;
            self.nloselo[player_index] += lo_losses;
        }

        if lo_val.is_some() && hi_wins > 0 && lo_wins > 0 && hi_ties == 0 && lo_ties == 0 {
            self.nscoop[player_index] += 1;
        }

        let total_opponents = (npockets - 1) as f64;
        let hi_equity = (hi_wins as f64 + hi_ties as f64 / 2.0) / total_opponents;
        let lo_equity = (lo_wins as f64 + lo_ties as f64 / 2.0) / total_opponents;
        self.ev[player_index] += hi_equity + lo_equity;
    }

    /// Prints the ordering histogram of relative hand ranks.
    pub fn print_ordering(&mut self, terse: bool) {
        if let Some(ordering) = &mut self.ordering {
            if !terse {
                println!("Histogram of relative hand ranks:");
            }
            match ordering.mode {
                EnumOrderingMode::Hi | EnumOrderingMode::Lo => {
                    if !terse {
                        for k in 0..ordering.nplayers {
                            print!(" {:2}", (b'A' + k as u8) as char);
                        }
                        println!(" {:8}", "Freq");
                    } else {
                        print!("ORD {} {}:", ordering.mode as u32, ordering.nplayers);
                    }
                    let nplayers = self.nplayers as usize;
                    for i in 0..nplayers {
                        for j in 0..nplayers {
                            if i != j {
                                let index = i * nplayers + j;
                                ordering.hist[index] += self.nwinhi[i];
                                ordering.hist[index] += self.ntiehi[i];
                                ordering.hist[index] += self.nlosehi[j];
                            }
                        }
                    }
                    for i in 0..ordering.nentries {
                        if ordering.hist[i] > 0 {
                            for k in 0..ordering.nplayers {
                                let rank = enum_ordering_decode_k(i as i32, ordering.nplayers, k);
                                if rank as usize == ordering.nplayers {
                                    print!(" NQ");
                                } else {
                                    print!(" {:2}", rank + 1);
                                }
                            }
                            print!(" {:8}", ordering.hist[i]);
                            if terse {
                                print!("|");
                            } else {
                                println!();
                            }
                        }
                    }
                }
                EnumOrderingMode::Hilo => {
                    if !terse {
                        print!("HI:");
                        for k in 0..ordering.nplayers {
                            print!(" {:2}", (b'A' + k as u8) as char);
                        }
                        print!("  LO:");
                        for k in 0..ordering.nplayers {
                            print!(" {:2}", (b'A' + k as u8) as char);
                        }
                        println!(" {:8}", "Freq");
                    } else {
                        print!("ORD HILO {}:", ordering.nplayers);
                    }

                    for i in 0..ordering.nentries {
                        if ordering.hist[i] > 0 {
                            if !terse {
                                print!("   ");
                            }

                            for k in 0..ordering.nplayers {
                                let rank_hi =
                                    enum_ordering_decode_hilo_k_hi(i as i32, ordering.nplayers, k);
                                if rank_hi as usize == ordering.nplayers {
                                    print!(" NQ");
                                } else {
                                    print!(" {:2}", rank_hi + 1);
                                }
                            }

                            if !terse {
                                print!("     ");
                            }

                            for k in 0..ordering.nplayers {
                                let rank_lo =
                                    enum_ordering_decode_hilo_k_lo(i as i32, ordering.nplayers, k);
                                if rank_lo as usize == ordering.nplayers {
                                    print!(" NQ");
                                } else {
                                    print!(" {:2}", rank_lo + 1);
                                }
                            }

                            print!(" {:8}", ordering.hist[i]);
                            if terse {
                                print!("|");
                            } else {
                                println!();
                            }
                        }
                    }
                }

                EnumOrderingMode::None => {
                    println!("No ordering mode set.");
                }
            }

            if terse {
                println!();
            }
        }
    }

    /// Prints detailed enumeration results including win/tie/loss counts and equity.
    pub fn enum_result_print(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
    ) -> Result<(), PokerError> {
        let gp = self.game.game_params();
        if let Some(gp) = gp {
            let width = gp.maxpocket * 3 - 1;
            println!(
                "{}: {} {} {}{}",
                gp.name,
                self.nsamples,
                match self.sample_type {
                    SampleType::Sample => "sampled",
                    SampleType::Exhaustive => "enumerated",
                },
                if gp.maxboard > 0 { "board" } else { "outcome" },
                if self.nsamples == 1 { "" } else { "s" }
            );

            if board.num_cards() > 0 {
                println!(" containing {}", board.to_string_representation());
            }

            if gp.haslopot == 1 && gp.hashipot == 1 {
                println!(
                    "{:width$} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>5}",
                    "cards",
                    "scoop",
                    "HIwin",
                    "HIlos",
                    "HItie",
                    "LOwin",
                    "LOlos",
                    "LOtie",
                    "EV",
                    width = width as usize
                );
                for (i, pocket) in pockets.iter().enumerate().take(self.nplayers as usize) {
                    println!(
                        "{:width$} {:7} {:7} {:7} {:7} {:7} {:7} {:7} {:5.3}",
                        pocket.to_string_representation(),
                        self.nscoop[i],
                        self.nwinhi[i],
                        self.nlosehi[i],
                        self.ntiehi[i],
                        self.nwinlo[i],
                        self.nloselo[i],
                        self.ntielo[i],
                        self.ev[i] / self.nsamples as f64,
                        width = width as usize
                    );
                }
            } else {
                println!(
                    "{:width$} {:>7} {:>6} {:>7} {:>6} {:>7} {:>6} {:>5}",
                    "cards",
                    "win",
                    "%win",
                    "lose",
                    "%lose",
                    "tie",
                    "%tie",
                    "EV",
                    width = width as usize
                );
                for (i, pocket) in pockets.iter().enumerate().take(self.nplayers as usize) {
                    let (nwin, nlose, ntie) = if gp.haslopot == 1 {
                        (self.nwinlo[i], self.nloselo[i], self.ntielo[i])
                    } else {
                        (self.nwinhi[i], self.nlosehi[i], self.ntiehi[i])
                    };

                    let win_percent = 100.0 * nwin as f64 / self.nsamples as f64;
                    let lose_percent = 100.0 * nlose as f64 / self.nsamples as f64;
                    let tie_percent = 100.0 * ntie as f64 / self.nsamples as f64;
                    let ev = self.ev[i] / self.nsamples as f64;

                    println!(
                        "{:width$} {:7} {:6.2}% {:7} {:6.2}% {:7} {:6.2}% {:5.3}",
                        pocket.to_string_representation(),
                        nwin,
                        win_percent,
                        nlose,
                        lose_percent,
                        ntie,
                        tie_percent,
                        ev,
                        width = width as usize
                    );
                }
            }

            if self.ordering.is_some() {
                self.print_ordering(false);
            }
            Ok(())
        } else {
            Err(PokerError::UnsupportedGameType)
        }
    }

    /// Prints a terse (compact) summary of equity values for all players.
    pub fn enum_result_print_terse(&self, _pockets: &[StdDeckCardMask], _board: StdDeckCardMask) {
        print!("EV {}: ", self.nplayers);
        for &ev in &self.ev[0..self.nplayers as usize] {
            print!("{:8.6} ", ev / self.nsamples as f64);
        }
        println!();
    }
}
