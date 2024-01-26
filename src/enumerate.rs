use crate::combinaison::*;
use crate::enumdefs::{EnumResult, ENUM_MAXPLAYERS};
use crate::enumdefs::{Game, GameParams};
use crate::enumord::{EnumOrdering, EnumOrderingMode};
use crate::eval_low::std_deck_lowball_eval;
use crate::eval_low8::std_deck_lowball8_eval;
use crate::eval_omaha::std_deck_omaha_hi_low8_eval;
use crate::handval::HandVal;
use crate::handval_low::{LowHandVal, LOW_HAND_VAL_NOTHING};
use crate::t_cardmasks::StdDeckCardMask;
use crate::t_jokercardmasks::JokerDeckCardMask;

use crate::eval::Eval;
use rand::seq::SliceRandom; // Assurez-vous que la crate rand est incluse dans votre Cargo.toml
use rand::thread_rng;
use std::ops::BitOr;
use std::ptr::NonNull;

// Trait pour gérer les masques de cartes
pub trait CardMask: BitOr<Output = Self> + Clone + PartialEq {
    fn mask(&self) -> u64;
    fn overlaps(&self, other: &Self) -> bool;
}

impl CardMask for StdDeckCardMask {
    fn mask(&self) -> u64 {
        self.mask
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.mask & other.mask) != 0
    }
}

impl CardMask for JokerDeckCardMask {
    fn mask(&self) -> u64 {
        self.cards_n
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.cards_n & other.cards_n) != 0
    }
}

impl BitOr for StdDeckCardMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        StdDeckCardMask {
            mask: self.mask | rhs.mask,
        }
    }
}

impl BitOr for JokerDeckCardMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        JokerDeckCardMask {
            cards_n: self.cards_n | rhs.cards_n,
        }
    }
}

fn enumerate_1_cards<T, F>(deck: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T),
{
    for card in deck {
        action(card);
    }
}

fn enumerate_2_cards<T, F>(deck: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        for i2 in 0..i1 {
            let card1 = &deck[i1];
            let card2 = &deck[i2];
            action(card1, card2);
        }
    }
}

fn enumerate_3_cards<T, F>(deck: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        for i2 in 0..i1 {
            for i3 in 0..i2 {
                let card1 = &deck[i1];
                let card2 = &deck[i2];
                let card3 = &deck[i3];
                action(card1, card2, card3);
            }
        }
    }
}

fn enumerate_4_cards<T, F>(deck: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        for i2 in 0..i1 {
            for i3 in 0..i2 {
                for i4 in 0..i3 {
                    let card1 = &deck[i1];
                    let card2 = &deck[i2];
                    let card3 = &deck[i3];
                    let card4 = &deck[i4];
                    action(card1, card2, card3, card4);
                }
            }
        }
    }
}

fn enumerate_5_cards<T, F>(deck: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        for i2 in 0..i1 {
            for i3 in 0..i2 {
                for i4 in 0..i3 {
                    for i5 in 0..i4 {
                        let card1 = &deck[i1];
                        let card2 = &deck[i2];
                        let card3 = &deck[i3];
                        let card4 = &deck[i4];
                        let card5 = &deck[i5];
                        action(card1, card2, card3, card4, card5);
                    }
                }
            }
        }
    }
}

fn enumerate_6_cards<T, F>(deck: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        for i2 in 0..i1 {
            for i3 in 0..i2 {
                for i4 in 0..i3 {
                    for i5 in 0..i4 {
                        for i6 in 0..i5 {
                            let card1 = &deck[i1];
                            let card2 = &deck[i2];
                            let card3 = &deck[i3];
                            let card4 = &deck[i4];
                            let card5 = &deck[i5];
                            let card6 = &deck[i6];
                            action(card1, card2, card3, card4, card5, card6);
                        }
                    }
                }
            }
        }
    }
}

fn enumerate_7_cards<T, F>(deck: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        for i2 in 0..i1 {
            for i3 in 0..i2 {
                for i4 in 0..i3 {
                    for i5 in 0..i4 {
                        for i6 in 0..i5 {
                            for i7 in 0..i6 {
                                let card1 = &deck[i1];
                                let card2 = &deck[i2];
                                let card3 = &deck[i3];
                                let card4 = &deck[i4];
                                let card5 = &deck[i5];
                                let card6 = &deck[i6];
                                let card7 = &deck[i7];
                                action(card1, card2, card3, card4, card5, card6, card7);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn enumerate_1_cards_d<T, F>(deck: &[T], dead_cards: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T),
{
    for card in deck {
        if dead_cards
            .iter()
            .any(|dead_card| dead_card.mask() == card.mask())
        {
            continue; // Skip dead cards
        }
        action(card);
    }
}

fn enumerate_2_cards_d<T, F>(deck: &[T], dead_cards: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards
            .iter()
            .any(|dead_card| dead_card.mask() == card1.mask())
        {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards
                .iter()
                .any(|dead_card| dead_card.mask() == card2.mask())
            {
                continue;
            }

            action(card1, card2);
        }
    }
}

fn enumerate_3_cards_d<T, F>(deck: &[T], dead_cards: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards
            .iter()
            .any(|dead_card| dead_card.mask() == card1.mask())
        {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards
                .iter()
                .any(|dead_card| dead_card.mask() == card2.mask())
            {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards
                    .iter()
                    .any(|dead_card| dead_card.mask() == card3.mask())
                {
                    continue;
                }

                action(card1, card2, card3);
            }
        }
    }
}

fn enumerate_4_cards_d<T, F>(deck: &[T], dead_cards: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards
            .iter()
            .any(|dead_card| dead_card.mask() == card1.mask())
        {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards
                .iter()
                .any(|dead_card| dead_card.mask() == card2.mask())
            {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards
                    .iter()
                    .any(|dead_card| dead_card.mask() == card3.mask())
                {
                    continue;
                }

                for i4 in 0..i3 {
                    let card4 = &deck[i4];
                    if dead_cards
                        .iter()
                        .any(|dead_card| dead_card.mask() == card4.mask())
                    {
                        continue;
                    }

                    action(card1, card2, card3, card4);
                }
            }
        }
    }
}

fn enumerate_5_cards_d<T, F>(deck: &[T], dead_cards: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards
            .iter()
            .any(|dead_card| dead_card.mask() == card1.mask())
        {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards
                .iter()
                .any(|dead_card| dead_card.mask() == card2.mask())
            {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards
                    .iter()
                    .any(|dead_card| dead_card.mask() == card3.mask())
                {
                    continue;
                }

                for i4 in 0..i3 {
                    let card4 = &deck[i4];
                    if dead_cards
                        .iter()
                        .any(|dead_card| dead_card.mask() == card4.mask())
                    {
                        continue;
                    }

                    for i5 in 0..i4 {
                        let card5 = &deck[i5];
                        if dead_cards
                            .iter()
                            .any(|dead_card| dead_card.mask() == card5.mask())
                        {
                            continue;
                        }

                        action(card1, card2, card3, card4, card5);
                    }
                }
            }
        }
    }
}

fn enumerate_6_cards_d<T, F>(deck: &[T], dead_cards: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards
            .iter()
            .any(|dead_card| dead_card.mask() == card1.mask())
        {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards
                .iter()
                .any(|dead_card| dead_card.mask() == card2.mask())
            {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards
                    .iter()
                    .any(|dead_card| dead_card.mask() == card3.mask())
                {
                    continue;
                }

                for i4 in 0..i3 {
                    let card4 = &deck[i4];
                    if dead_cards
                        .iter()
                        .any(|dead_card| dead_card.mask() == card4.mask())
                    {
                        continue;
                    }

                    for i5 in 0..i4 {
                        let card5 = &deck[i5];
                        if dead_cards
                            .iter()
                            .any(|dead_card| dead_card.mask() == card5.mask())
                        {
                            continue;
                        }

                        for i6 in 0..i5 {
                            let card6 = &deck[i6];
                            if dead_cards
                                .iter()
                                .any(|dead_card| dead_card.mask() == card6.mask())
                            {
                                continue;
                            }

                            action(card1, card2, card3, card4, card5, card6);
                        }
                    }
                }
            }
        }
    }
}

fn enumerate_7_cards_d<T, F>(deck: &[T], dead_cards: &[T], mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards
            .iter()
            .any(|dead_card| dead_card.mask() == card1.mask())
        {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards
                .iter()
                .any(|dead_card| dead_card.mask() == card2.mask())
            {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards
                    .iter()
                    .any(|dead_card| dead_card.mask() == card3.mask())
                {
                    continue;
                }

                for i4 in 0..i3 {
                    let card4 = &deck[i4];
                    if dead_cards
                        .iter()
                        .any(|dead_card| dead_card.mask() == card4.mask())
                    {
                        continue;
                    }

                    for i5 in 0..i4 {
                        let card5 = &deck[i5];
                        if dead_cards
                            .iter()
                            .any(|dead_card| dead_card.mask() == card5.mask())
                        {
                            continue;
                        }

                        for i6 in 0..i5 {
                            let card6 = &deck[i6];
                            if dead_cards
                                .iter()
                                .any(|dead_card| dead_card.mask() == card6.mask())
                            {
                                continue;
                            }

                            for i7 in 0..i6 {
                                let card7 = &deck[i7];
                                if dead_cards
                                    .iter()
                                    .any(|dead_card| dead_card.mask() == card7.mask())
                                {
                                    continue;
                                }

                                action(card1, card2, card3, card4, card5, card6, card7);
                            }
                        }
                    }
                }
            }
        }
    }
}
fn enumerate_n_cards_d<T, F>(deck: &[T], dead_cards: &[T], n_cards: usize, mut action: F)
where
    T: CardMask,
    F: FnMut(Vec<&T>),
{
    let mut indices = (0..n_cards).collect::<Vec<_>>();

    while !indices.is_empty() {
        if indices.last().unwrap() < &deck.len() {
            if indices.iter().all(|&i| {
                !dead_cards
                    .iter()
                    .any(|dead_card| dead_card.mask() == deck[i].mask())
            }) {
                let current_combination = indices.iter().map(|&i| &deck[i]).collect::<Vec<_>>();
                action(current_combination);
            }

            *indices.last_mut().unwrap() += 1;
        } else {
            indices.pop();
            if let Some(last) = indices.last_mut() {
                *last += 1;
                while indices.len() < n_cards {
                    indices.push(indices.last().unwrap() + 1);
                }
            }
        }
    }
}

fn enumerate_n_cards<T, F>(deck: &[T], n_cards: usize, mut action: F)
where
    T: CardMask,
    F: FnMut(Vec<&T>),
{
    let mut indices = (0..n_cards).collect::<Vec<_>>();

    while !indices.is_empty() {
        if indices.last().unwrap() < &deck.len() {
            let current_combination = indices.iter().map(|&i| &deck[i]).collect::<Vec<_>>();
            action(current_combination);

            *indices.last_mut().unwrap() += 1;
        } else {
            indices.pop();
            if let Some(last) = indices.last_mut() {
                *last += 1;
                while indices.len() < n_cards {
                    indices.push(indices.last().unwrap() + 1);
                }
            }
        }
    }
}

fn enumerate_combinations_d<T, F>(
    decks: Vec<&[T]>,
    set_sizes: Vec<usize>,
    dead_cards: Vec<T>,
    mut action: F,
) where
    T: CardMask + PartialEq,
    F: FnMut(Vec<Vec<&T>>),
{
    let num_sets = decks.len();
    let mut combos = Vec::with_capacity(num_sets);

    // Initialiser les combinaisons pour chaque set
    for (deck, &size) in decks.iter().zip(set_sizes.iter()) {
        if let Some(combo) = Combination::new(deck.len(), size) {
            combos.push(combo);
        } else {
            return; // Pas assez de cartes pour générer les combinaisons
        }
    }

    let mut indices = vec![0; num_sets];
    let mut is_done = false;

    while !is_done {
        let mut current_sets = Vec::with_capacity(num_sets);
        let mut is_valid = true;

        // Construire la combinaison actuelle
        for (index, combo) in indices.iter().zip(combos.iter()) {
            if let Some(set) = combo.get_combination(*index) {
                let set_cards: Vec<_> = set.iter().map(|&i| &decks[combo.nelem][i]).collect();
                // Vérifier si la combinaison contient des cartes mortes
                if set_cards.iter().any(|&card| dead_cards.contains(card)) {
                    is_valid = false;
                    break;
                }
                current_sets.push(set_cards);
            } else {
                is_done = true;
                break;
            }
        }

        if is_valid && !is_done {
            action(current_sets);
        }

        // Incrémenter les indices pour la prochaine combinaison
        for i in (0..num_sets).rev() {
            indices[i] += 1;
            if indices[i] < combos[i].num_combinations() {
                break;
            }
            if i == 0 {
                is_done = true;
            }
            indices[i] = 0;
        }
    }
}

fn enumerate_permutations_d<T, F>(
    decks: &[Vec<T>],
    set_sizes: &[usize],
    dead_cards: &[T],
    default_card: T,
    mut action: F,
) where
    T: CardMask + Clone + PartialEq + std::ops::BitOr<Output = T>,
    F: FnMut(Vec<Vec<T>>),
{
    let live_cards: Vec<T> = decks
        .iter()
        .flatten()
        .filter(|&card| !dead_cards.contains(card))
        .cloned()
        .collect();

    let n_cards = set_sizes.iter().sum();
    if live_cards.len() < n_cards {
        eprintln!("ENUMERATE_PERMUTATIONS: not enough cards");
        return;
    }

    let mut indices = vec![0; n_cards + 1];
    let mut or_masks = vec![default_card.clone(); n_cards + 1];

    // Initialisation des indices et des masques OR
    for i in 1..=n_cards {
        indices[i] = i - 1;
        or_masks[i] = or_masks[i - 1].clone() | live_cards[indices[i]].clone();
    }

    loop {
        let mut set_vars = Vec::with_capacity(decks.len());
        let mut t = 0;

        for &size in set_sizes {
            let mut set = Vec::with_capacity(size);
            for j in t..t + size {
                set.push(live_cards[indices[j]].clone());
            }
            set_vars.push(set);
            t += size;
        }

        action(set_vars);

        // Trouver le prochain indice pour la permutation
        let mut index = n_cards;
        loop {
            indices[index] += 1;
            while indices[index] >= live_cards.len() {
                if index == 0 {
                    return;
                }
                index -= 1;
                indices[index] += 1;
            }

            if !or_masks[index - 1].overlaps(&live_cards[indices[index]]) {
                break;
            }
        }

        if index == 0 {
            return;
        }

        // Mise à jour des masques OR pour la nouvelle permutation
        or_masks[index] = or_masks[index - 1].clone() | live_cards[indices[index]].clone();
        for i in index + 1..=n_cards {
            indices[i] = 0;
            while or_masks[i - 1].overlaps(&live_cards[indices[i]]) {
                indices[i] += 1;
            }
            or_masks[i] = or_masks[i - 1].clone() | live_cards[indices[i]].clone();
        }
    }
}

fn deck_montecarlo_n_cards_d<T, F>(
    deck: &[T],
    dead_cards: &[T],
    num_cards: usize,
    num_iter: usize,
    mut action: F,
) where
    T: CardMask + Clone,
    F: FnMut(Vec<&T>),
{
    let mut rng = rand::thread_rng();

    for _ in 0..num_iter {
        let mut used = dead_cards.to_vec();
        let mut cards_var = Vec::with_capacity(num_cards);

        while cards_var.len() < num_cards {
            let card = deck.choose(&mut rng).unwrap();

            if !used.contains(card) {
                used.push(card.clone());
                cards_var.push(card);
            }
        }

        action(cards_var.clone());
    }
}

fn montecarlo_permutations_d<T, F>(
    decks: &[Vec<T>],
    set_sizes: &[usize],
    dead_cards: &[T],
    num_iter: usize,
    mut action: F,
) where
    T: CardMask + Clone,
    F: FnMut(Vec<Vec<&T>>),
{
    let mut rng = thread_rng();
    for _ in 0..num_iter {
        let mut used_cards = dead_cards.to_vec();
        let mut set_vars = Vec::with_capacity(set_sizes.len());

        for (&size, deck) in set_sizes.iter().zip(decks.iter()) {
            let mut set = Vec::with_capacity(size);
            while set.len() < size {
                let card = deck.choose(&mut rng).expect("Deck is empty");
                if !used_cards.contains(card) {
                    used_cards.push(card.clone());
                    set.push(card);
                }
            }
            set_vars.push(set);
        }

        action(set_vars);
    }
}

pub fn game_params() -> Vec<GameParams> {
    vec![
        GameParams {
            game: Game::Holdem,
            minpocket: 2,
            maxpocket: 2,
            maxboard: 5,
            haslopot: 0,
            hashipot: 1,
            name: "Holdem Hi".to_string(),
        },
        GameParams {
            game: Game::Holdem8,
            minpocket: 2,
            maxpocket: 2,
            maxboard: 5,
            haslopot: 1,
            hashipot: 1,
            name: "Holdem Hi/Low 8-or-better".to_string(),
        },
        GameParams {
            game: Game::Omaha,
            minpocket: 4,
            maxpocket: 4,
            maxboard: 5,
            haslopot: 0,
            hashipot: 1,
            name: "Omaha Hi".to_string(),
        },
        GameParams {
            game: Game::Omaha5,
            minpocket: 5,
            maxpocket: 5,
            maxboard: 5,
            haslopot: 0,
            hashipot: 1,
            name: "Omaha Hi 5cards".to_string(),
        },
        GameParams {
            game: Game::Omaha6,
            minpocket: 6,
            maxpocket: 6,
            maxboard: 5,
            haslopot: 0,
            hashipot: 1,
            name: "Omaha Hi 6cards".to_string(),
        },
        GameParams {
            game: Game::Omaha8,
            minpocket: 4,
            maxpocket: 4,
            maxboard: 5,
            haslopot: 1,
            hashipot: 1,
            name: "Omaha Hi/Low 8-or-better".to_string(),
        },
        GameParams {
            game: Game::Omaha85,
            minpocket: 5,
            maxpocket: 5,
            maxboard: 5,
            haslopot: 1,
            hashipot: 1,
            name: "Omaha 5cards Hi/Low 8-or-better".to_string(),
        },
        GameParams {
            game: Game::Stud7,
            minpocket: 3,
            maxpocket: 7,
            maxboard: 0,
            haslopot: 0,
            hashipot: 1,
            name: "7-card Stud Hi".to_string(),
        },
        GameParams {
            game: Game::Stud78,
            minpocket: 3,
            maxpocket: 7,
            maxboard: 0,
            haslopot: 1,
            hashipot: 1,
            name: "7-card Stud Hi/Low 8-or-better".to_string(),
        },
        GameParams {
            game: Game::Stud7nsq,
            minpocket: 3,
            maxpocket: 7,
            maxboard: 0,
            haslopot: 1,
            hashipot: 1,
            name: "7-card Stud Hi/Low no qualifier".to_string(),
        },
        GameParams {
            game: Game::Razz,
            minpocket: 3,
            maxpocket: 7,
            maxboard: 0,
            haslopot: 1,
            hashipot: 0,
            name: "Razz (7-card Stud A-5 Low)".to_string(),
        },
        GameParams {
            game: Game::Draw5,
            minpocket: 0,
            maxpocket: 5,
            maxboard: 0,
            haslopot: 0,
            hashipot: 1,
            name: "5-card Draw Hi with joker".to_string(),
        },
        GameParams {
            game: Game::Draw58,
            minpocket: 0,
            maxpocket: 5,
            maxboard: 0,
            haslopot: 1,
            hashipot: 1,
            name: "5-card Draw Hi/Low 8-or-better with joker".to_string(),
        },
        GameParams {
            game: Game::Draw5nsq,
            minpocket: 0,
            maxpocket: 5,
            maxboard: 0,
            haslopot: 1,
            hashipot: 1,
            name: "5-card Draw Hi/Low no qualifier with joker".to_string(),
        },
        GameParams {
            game: Game::Lowball,
            minpocket: 0,
            maxpocket: 5,
            maxboard: 0,
            haslopot: 1,
            hashipot: 0,
            name: "5-card Draw A-5 Lowball with joker".to_string(),
        },
        GameParams {
            game: Game::Lowball27,
            minpocket: 0,
            maxpocket: 5,
            maxboard: 0,
            haslopot: 1,
            hashipot: 0,
            name: "5-card Draw 2-7 Lowball".to_string(),
        },
    ]
}

fn inner_loop<F, G, H>(
    npockets: usize,
    mut evalwrap: F,
    mut ordering_increment: G,
    mut ordering_increment_hilo: H,
    result: &mut EnumResult,
) where
    F: FnMut(usize) -> (Result<HandVal, i32>, Result<LowHandVal, i32>),
    G: FnMut(&mut EnumResult, &[usize], &[usize]),
    H: FnMut(&mut EnumResult, &[usize], &[usize]),
{
    let HANDVAL_NOTHING: u32 = HandVal::new(0, 0, 0, 0, 0, 0).value;

    let mut hival = vec![HANDVAL_NOTHING; ENUM_MAXPLAYERS];
    let mut loval = vec![LOW_HAND_VAL_NOTHING; ENUM_MAXPLAYERS];
    let mut besthi = HANDVAL_NOTHING;
    let mut bestlo = LOW_HAND_VAL_NOTHING;
    let mut hishare = 0;
    let mut loshare = 0;

    // Determine winning hands for high and low
    for i in 0..npockets {
        let (hi_res, lo_res) = evalwrap(i);

        let hi = hi_res.map(|h| h.value).unwrap_or(HANDVAL_NOTHING);
        let lo = lo_res.map(|l| l.value).unwrap_or(LOW_HAND_VAL_NOTHING);

        hival[i] = hi;
        loval[i] = lo;

        if hi != HANDVAL_NOTHING {
            if hi > besthi {
                besthi = hi;
                hishare = 1;
            } else if hi == besthi {
                hishare += 1;
            }
        }

        if lo != LOW_HAND_VAL_NOTHING {
            if lo < bestlo {
                bestlo = lo;
                loshare = 1;
            } else if lo == bestlo {
                loshare += 1;
            }
        }
    }

    let hipot = if besthi != HANDVAL_NOTHING {
        1.0 / hishare as f64
    } else {
        0.0
    };
    let lopot = if bestlo != LOW_HAND_VAL_NOTHING {
        1.0 / loshare as f64
    } else {
        0.0
    };

    // Award pot fractions to winning hands
    for i in 0..npockets {
        let mut potfrac = 0.0;

        if hival[i] == besthi {
            potfrac += hipot;
            if hishare == 1 {
                result.nwinhi[i] += 1;
            } else {
                result.ntiehi[i] += 1;
            }
        } else if hival[i] != HANDVAL_NOTHING {
            result.nlosehi[i] += 1;
        }

        if loval[i] == bestlo {
            potfrac += lopot;
            if loshare == 1 {
                result.nwinlo[i] += 1;
            } else {
                result.ntielo[i] += 1;
            }
        } else if loval[i] != LOW_HAND_VAL_NOTHING {
            result.nloselo[i] += 1;
        }

        result.ev[i] += potfrac;
    }

    // Update ordering if applicable
    unsafe {
        if let Some(mut ordering_ptr) = result.ordering {
            let ordering = ordering_ptr.as_mut(); // Get a mutable reference to EnumOrdering

            let hiranks: Vec<_> = hival.iter().map(|&val| val as usize).collect();
            let loranks: Vec<_> = loval.iter().map(|&val| val as usize).collect();

            match ordering.mode {
                EnumOrderingMode::Hi => ordering_increment(result, &hiranks, &loranks),
                EnumOrderingMode::Lo => ordering_increment(result, &loranks, &hiranks),
                EnumOrderingMode::Hilo => ordering_increment_hilo(result, &hiranks, &loranks),
                _ => (),
            }
        }
    }

    result.nsamples += 1;
}

// Variante: Texas Hold'em
pub fn inner_loop_holdem(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        // Clone the values before performing the bitwise OR operation
        let final_board = board.clone() | shared_cards.clone();
        let hand = pocket.clone() | final_board;

        // Assuming Eval::eval_n() is a function that evaluates the hand
        hival[i] = Eval::eval_n(&hand, 7);
        loval[i] = LowHandVal { value: 0 };
    }
}

// Variante: Texas Hold'em Hi/Lo 8 or better
pub fn inner_loop_holdem8(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        // Combinaison des cartes du tableau et des cartes partagées
        let final_board = board.clone() | shared_cards.clone();
        let hand = pocket.clone() | final_board;

        // Évaluation des mains hautes et basses
        hival[i] = Eval::eval_n(&hand, 7); // Remplacer par la fonction appropriée
        loval[i] = std_deck_lowball8_eval(&hand, 7); // Remplacer par la fonction appropriée
    }
}

// Variante: Omaha
pub fn inner_loop_omaha(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        let final_board = board.clone() | shared_cards.clone();
        let mut high_option: Option<HandVal> = None;
        let mut low_option: Option<LowHandVal> = None;

        // Appel de la fonction d'évaluation avec les bons arguments
        match std_deck_omaha_hi_low8_eval(
            pocket.clone(),
            final_board,
            &mut high_option,
            &mut low_option,
        ) {
            Ok(()) => {
                // Assigner les valeurs évaluées à hival et loval
                if let Some(high_hand) = high_option {
                    hival[i] = high_hand;
                }
                // ou utilisez une autre fonction si nécessaire
                loval[i] = LowHandVal { value: 0 }; // Utiliser une constante appropriée pour "rien"
            }
            Err(e) => {
                eprintln!("Erreur lors de l'évaluation : {}", e);
                continue; // Gestion d'erreur
            }
        }
    }
}
// Variante: Omaha 5 Cards
pub fn inner_loop_omaha5(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        let final_board = board.clone() | shared_cards.clone();
        let mut high_option: Option<HandVal> = None;
        let mut low_option: Option<LowHandVal> = None;

        // Appel de la fonction d'évaluation avec les bons arguments
        match std_deck_omaha_hi_low8_eval(
            pocket.clone(),
            final_board,
            &mut high_option,
            &mut low_option,
        ) {
            Ok(()) => {
                // Assigner les valeurs évaluées à hival et loval
                if let Some(high_hand) = high_option {
                    hival[i] = high_hand;
                }
                // ou utilisez une autre fonction si nécessaire
                loval[i] = LowHandVal { value: 0 }; // Utiliser une constante appropriée pour "rien"
            }
            Err(e) => {
                eprintln!("Erreur lors de l'évaluation : {}", e);
                continue; // Gestion d'erreur
            }
        }
    }
}
// Variante: Omaha 6 Cards
pub fn inner_loop_omaha6(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        let final_board = board.clone() | shared_cards.clone();
        let mut high_option: Option<HandVal> = None;
        let mut low_option: Option<LowHandVal> = None;

        // Appel de la fonction d'évaluation avec les bons arguments
        match std_deck_omaha_hi_low8_eval(
            pocket.clone(),
            final_board,
            &mut high_option,
            &mut low_option,
        ) {
            Ok(()) => {
                // Assigner les valeurs évaluées à hival et loval
                if let Some(high_hand) = high_option {
                    hival[i] = high_hand;
                }
                // ou utilisez une autre fonction si nécessaire
                loval[i] = LowHandVal { value: 0 }; // Utiliser une constante appropriée pour "rien"
            }
            Err(e) => {
                eprintln!("Erreur lors de l'évaluation : {}", e);
                continue; // Gestion d'erreur
            }
        }
    }
}
// Variante: Omaha 4 Cards Hi/Lo
pub fn inner_loop_omaha8(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        let final_board = board.clone() | shared_cards.clone();
        let mut high_option: Option<HandVal> = None;
        let mut low_option: Option<LowHandVal> = None;

        // Appel de la fonction d'évaluation pour Omaha Hi/Lo
        match std_deck_omaha_hi_low8_eval(
            pocket.clone(),
            final_board,
            &mut high_option,
            &mut low_option,
        ) {
            Ok(()) => {
                // Assigner les valeurs évaluées à hival et loval
                if let Some(high_hand) = high_option {
                    hival[i] = high_hand;
                }
                if let Some(low_hand) = low_option {
                    loval[i] = low_hand;
                }
            }
            Err(e) => {
                eprintln!("Erreur lors de l'évaluation : {}", e);
                continue; // Gestion d'erreur
            }
        }
    }
}
// Variante: Omaha 5 Cards Hi/Lo
pub fn inner_loop_omaha85(
    pockets: &[StdDeckCardMask],
    board: &StdDeckCardMask,
    shared_cards: &StdDeckCardMask,
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        let final_board = board.clone() | shared_cards.clone();
        let mut high_option: Option<HandVal> = None;
        let mut low_option: Option<LowHandVal> = None;

        // Appel de la fonction d'évaluation pour Omaha 5 Hi/Lo
        match std_deck_omaha_hi_low8_eval(
            pocket.clone(),
            final_board,
            &mut high_option,
            &mut low_option,
        ) {
            Ok(()) => {
                // Assigner les valeurs évaluées à hival et loval
                if let Some(high_hand) = high_option {
                    hival[i] = high_hand;
                }
                if let Some(low_hand) = low_option {
                    loval[i] = low_hand;
                }
            }
            Err(e) => {
                eprintln!("Erreur lors de l'évaluation : {}", e);
                continue; // Gestion d'erreur
            }
        }
    }
}
// Variante: 7-Card Stud
pub fn inner_loop_7stud(
    pockets: &[StdDeckCardMask],
    unshared_cards: &[StdDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        // Assurez-vous qu'il y a un nombre correspondant de cartes non partagées pour chaque poche
        if i >= unshared_cards.len() {
            eprintln!(
                "Nombre insuffisant de cartes non partagées pour l'index {}",
                i
            );
            continue;
        }

        // Combinaison des cartes de poche et des cartes non partagées
        let hand = pocket.clone() | unshared_cards[i].clone();

        // Évaluation de la main haute
        hival[i] = Eval::eval_n(&hand, 7); // Remplacer par la fonction appropriée

        // La main basse n'est pas évaluée dans le 7-Card Stud standard
        loval[i] = LowHandVal { value: 0 }; // Utilisez une constante appropriée pour "rien"
    }
}
// Variante: 7-Card Stud Hi/Lo 8 or better
pub fn inner_loop_7stud8(
    pockets: &[StdDeckCardMask],
    unshared_cards: &[StdDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        // Assurez-vous qu'il y a un nombre correspondant de cartes non partagées pour chaque poche
        if i >= unshared_cards.len() {
            eprintln!(
                "Nombre insuffisant de cartes non partagées pour l'index {}",
                i
            );
            continue;
        }

        // Combinaison des cartes de poche et des cartes non partagées
        let hand = pocket.clone() | unshared_cards[i].clone();

        // Évaluation de la main haute
        hival[i] = Eval::eval_n(&hand, 7); // Remplacer par la fonction appropriée

        // Évaluation de la main basse (lowball 8 ou mieux)
        loval[i] = std_deck_lowball8_eval(&hand, 7); // Remplacer par la fonction appropriée
    }
}
// Variante: 7-Card Stud Hi/Lo no stinking qualifier
pub fn inner_loop_7studnsq(
    pockets: &[StdDeckCardMask],
    unshared_cards: &[StdDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        // Combinaison des cartes privatives du joueur avec les cartes non partagées
        let hand = pocket.clone() | unshared_cards[i].clone();

        // Évaluation de la main haute
        hival[i] = Eval::eval_n(&hand, 7);

        // Évaluation de la main basse (A-5 lowball)
        loval[i] = std_deck_lowball_eval(&hand, 7);
    }
}
// Variante: Razz
pub fn inner_loop_razz(
    pockets: &[StdDeckCardMask],
    unshared_cards: &[StdDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        // Combinaison des cartes privatives du joueur avec les cartes non partagées
        let hand = pocket.clone() | unshared_cards[i].clone();

        // Dans Razz, il n'y a pas de main haute, donc on la définit comme "rien"
        hival[i] = HandVal { value: 0 }; // Assurez-vous que HANDVAL_NOTHING est défini

        // Évaluation de la main basse selon les règles du lowball 2-7
        loval[i] = std_deck_lowball_eval(&hand, 7);
    }
}
