//! Monte Carlo sampling functions for random card draws.
//!
//! Uses Fisher-Yates partial shuffle for efficient random card selection
//! instead of rejection sampling.

use super::CardMask;
use crate::tables::t_cardmasks::StdDeckCardMask;
use rand::seq::SliceRandom;

/// Draws `num_cards` random cards from a deck `num_iter` times, excluding dead cards.
///
/// Uses Fisher-Yates partial shuffle: builds a live deck (excluding dead cards),
/// then shuffles only the first `num_cards` positions per iteration.
/// This is O(num_cards) per iteration instead of O(num_cards * deck_size) with rejection sampling.
pub(crate) fn deck_montecarlo_n_cards_d<F>(
    deck: &[StdDeckCardMask],
    dead_cards: StdDeckCardMask,
    num_cards: usize,
    num_iter: usize,
    mut action: F,
) where
    F: FnMut(&[StdDeckCardMask]),
{
    use rand::rngs::SmallRng;
    use rand::SeedableRng;
    let mut rng = SmallRng::from_entropy();

    // Build live deck excluding dead cards (one-time allocation)
    let mut live_deck: Vec<StdDeckCardMask> = deck
        .iter()
        .filter(|card| !card.is_empty() && !dead_cards.overlaps(card))
        .copied()
        .collect();

    let live_len = live_deck.len();
    if num_cards > live_len {
        return;
    }

    for _ in 0..num_iter {
        // Fisher-Yates partial shuffle: only shuffle first num_cards positions
        live_deck.partial_shuffle(&mut rng, num_cards);
        action(&live_deck[..num_cards]);
    }
}

/// Draws `num_cards` random cards using Quasi-Monte Carlo (Sobol sequence).
pub(crate) fn deck_qmc_n_cards_d<F>(
    deck: &[StdDeckCardMask],
    dead_cards: StdDeckCardMask,
    num_cards: usize,
    num_iter: usize,
    mut action: F,
) where
    F: FnMut(&[StdDeckCardMask]),
{
    use super::qmc::SobolGenerator;
    let mut sobol = SobolGenerator::new(num_cards);

    let live_deck_base: Vec<StdDeckCardMask> = deck
        .iter()
        .filter(|card| !card.is_empty() && !dead_cards.overlaps(card))
        .copied()
        .collect();

    let live_len = live_deck_base.len();
    if num_cards > live_len {
        return;
    }

    let mut drawn_cards = vec![StdDeckCardMask::new(); num_cards];

    for _ in 0..num_iter {
        let point = sobol.next_point();
        let mut current_deck = live_deck_base.clone();

        for i in 0..num_cards {
            let remaining_len = current_deck.len();
            let idx = (point[i] * remaining_len as f64) as usize;
            let actual_idx = if idx >= remaining_len {
                remaining_len - 1
            } else {
                idx
            };

            drawn_cards[i] = current_deck.remove(actual_idx);
        }

        action(&drawn_cards);
    }
}

/// Identical to deck_montecarlo_n_cards_d but for JokerDeckCardMask.
pub(crate) fn deck_montecarlo_n_cards_joker<F>(
    deck: &[crate::tables::t_jokercardmasks::JokerDeckCardMask],
    dead_cards: crate::tables::t_jokercardmasks::JokerDeckCardMask,
    num_cards: usize,
    num_iter: usize,
    mut action: F,
) where
    F: FnMut(&[crate::tables::t_jokercardmasks::JokerDeckCardMask]),
{
    use crate::tables::t_jokercardmasks::JokerDeckCardMask;
    use rand::rngs::SmallRng;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;

    let mut rng = SmallRng::from_entropy();

    // Build live deck
    // We can't use .overlaps() if it's not defined on JokerDeckCardMask.
    // Let's check definition. It usually has .is_disjoint or simple & check.
    // JokerDeckCardMask::overlaps? Let's check what checking dead cards logic requires.
    // StdDeckCardMask uses overlaps.
    // JokerDeckCardMask is a struct wrapping u64.
    // We can just check (card.cards_n & dead_cards.cards_n) != 0.

    let mut live_deck: Vec<JokerDeckCardMask> = deck
        .iter()
        .filter(|card| (card.cards_n & dead_cards.cards_n) == 0 && card.cards_n != 0)
        .copied()
        .collect();

    let live_len = live_deck.len();
    if num_cards > live_len {
        return;
    }

    for _ in 0..num_iter {
        live_deck.partial_shuffle(&mut rng, num_cards);
        action(&live_deck[..num_cards]);
    }
}

/// Draws random permutations of cards from multiple decks, excluding dead cards.
///
/// Each deck has a corresponding set size; cards are drawn without replacement across all sets.
#[allow(dead_code)]
pub(crate) fn montecarlo_permutations_d<T, F>(
    decks: &[Vec<T>],
    set_sizes: &[usize],
    dead_cards: &[T],
    num_iter: usize,
    mut action: F,
) -> Result<(), String>
where
    T: CardMask,
    F: FnMut(Vec<Vec<&T>>),
{
    use rand::rngs::SmallRng;
    use rand::SeedableRng;
    let mut rng = SmallRng::from_entropy();
    let max_cards: usize = set_sizes.iter().sum();
    let mut used_cards = Vec::with_capacity(dead_cards.len() + max_cards);
    let mut set_vars = Vec::with_capacity(set_sizes.len());

    for _ in 0..num_iter {
        used_cards.clear();
        used_cards.extend_from_slice(dead_cards);

        set_vars.clear();

        for (&size, deck) in set_sizes.iter().zip(decks.iter()) {
            let mut set = Vec::with_capacity(size);
            while set.len() < size {
                let card = deck
                    .choose(&mut rng)
                    .ok_or_else(|| "Deck is empty".to_string())?;
                if !used_cards.contains(card) {
                    used_cards.push(*card);
                    set.push(card);
                }
            }
            set_vars.push(set);
        }

        action(set_vars.clone());
    }
    Ok(())
}
