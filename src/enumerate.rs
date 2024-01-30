use crate::combinaison::*;
use crate::deck_std::STD_DECK_N_CARDS;
use crate::enumdefs::SampleType;
use crate::enumdefs::{EnumResult, ENUM_MAXPLAYERS};
use crate::enumdefs::{Game, GameParams};
use crate::enumord::EnumOrdering;
use crate::enumord::EnumOrderingMode;
use crate::enumord::{
    enum_ordering_decode_hilo_k_hi, enum_ordering_decode_hilo_k_lo, enum_ordering_decode_k,
    enum_ordering_nentries, enum_ordering_nentries_hilo,
};
use crate::enumord::{ENUM_ORDERING_MAXPLAYERS, ENUM_ORDERING_MAXPLAYERS_HILO};
use crate::eval_joker::EvalJoker;
use crate::eval_joker_low::joker_lowball_eval;
use crate::eval_joker_low8::joker_lowball8_eval;
use crate::eval_low::std_deck_lowball_eval;
use crate::eval_low27::std_deck_lowball27_eval;
use crate::eval_low8::std_deck_lowball8_eval;
use crate::eval_omaha::std_deck_omaha_hi_low8_eval;
use crate::handval::HandVal;
use crate::handval_low::{LowHandVal, LOW_HAND_VAL_NOTHING};
use crate::t_cardmasks::StdDeckCardMask;
use crate::t_jokercardmasks::JokerDeckCardMask;

use crate::eval::Eval;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::fmt;
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
// Fonction pour énumérer chaque carte dans un deck et appliquer une action donnée sur chaque carte
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
// Fonction pour énumérer chaque carte dans un deck, en excluant les dead cards, et appliquer une action donnée sur chaque carte restante.
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
// Fonction pour énumérer toutes les combinaisons possibles de `n_cards` cartes dans un deck, en excluant les dead card,
// et appliquer une action donnée sur chaque combinaison valide
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

// Fonction pour énumérer toutes les combinaisons possibles de `n_cards` cartes dans un deck
// et appliquer une action donnée sur chaque combinaison
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
// Fonction pour énumérer toutes les combinaisons possibles à partir de plusieurs sets de cartes, en excluant les cartes spécifiées comme dead.
// Chaque set dans le `decks` de cartes a sa propre taille de combinaison spécifiée dans `set_sizes`. `dead_cards` contient les cartes à exclure
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
                // Vérifier si la combinaison contient des dead cards
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

// Fonction pour énumérer toutes les permutations possibles à partir de plusieurs sets de cartes, en excluant les cartes spécifiées comme dead.
// `decks` contient les sets de cartes, `set_sizes` les tailles de chaque set, `dead_cards` les cartes à exclure, et `default_card` une carte par défaut utilisée pour initialiser les masques OR
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
// Fonction pour simuler des tirages de cartes aléatoires à partir d'un deck, en excluant certaines cartes, et appliquer une action donnée sur chaque tirage
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
// Fonction pour simuler des tirages aléatoires de plusieurs sets de cartes à partir de plusieurs decks, en excluant certaines cartes
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

// Fonction qui renvoie la liste des paramètres des différentes variantes
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
// Boucle pour évaluer des mains de poker, mettre à jour les statistiques de résultats et gérer le classement des mains.
// `npockets` est le nombre de poches (mains de départ) à évaluer, `evalwrap` est une fonction d'évaluation de main,
// `ordering_increment` et `ordering_increment_hilo` sont des fonctions pour mettre à jour le classement des mains,
// et `result` est la structure pour stocker les résultats de l'énumération.
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
    let handval_nothing: u32 = HandVal::new(0, 0, 0, 0, 0, 0).value;

    let mut hival = vec![handval_nothing; ENUM_MAXPLAYERS];
    let mut loval = vec![LOW_HAND_VAL_NOTHING; ENUM_MAXPLAYERS];
    let mut besthi = handval_nothing;
    let mut bestlo = LOW_HAND_VAL_NOTHING;
    let mut hishare = 0;
    let mut loshare = 0;

    for i in 0..npockets {
        let (hi_res, lo_res) = evalwrap(i);

        let hi = hi_res.map(|h| h.value).unwrap_or(handval_nothing);
        let lo = lo_res.map(|l| l.value).unwrap_or(LOW_HAND_VAL_NOTHING);

        hival[i] = hi;
        loval[i] = lo;

        if hi != handval_nothing {
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

    let hipot = if besthi != handval_nothing {
        1.0 / hishare as f64
    } else {
        0.0
    };
    let lopot = if bestlo != LOW_HAND_VAL_NOTHING {
        1.0 / loshare as f64
    } else {
        0.0
    };

    for i in 0..npockets {
        let mut potfrac = 0.0;

        if hival[i] == besthi {
            potfrac += hipot;
            if hishare == 1 {
                result.nwinhi[i] += 1;
            } else {
                result.ntiehi[i] += 1;
            }
        } else if hival[i] != handval_nothing {
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

    unsafe {
        if let Some(mut ordering_ptr) = result.ordering {
            let ordering = ordering_ptr.as_mut();

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
        let final_board = board.clone() | shared_cards.clone();
        let hand = pocket.clone() | final_board;

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
        hival[i] = Eval::eval_n(&hand, 7);
        loval[i] = std_deck_lowball8_eval(&hand, 7);
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

                loval[i] = LowHandVal { value: 0 };
            }
            Err(e) => {
                eprintln!("Erreur lors de l'évaluation : {}", e);
                continue;
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

                loval[i] = LowHandVal { value: 0 };
            }
            Err(e) => {
                eprintln!("Erreur lors de l'évaluation : {}", e);
                continue;
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

                loval[i] = LowHandVal { value: 0 };
            }
            Err(e) => {
                eprintln!("Erreur lors de l'évaluation : {}", e);
                continue;
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
                continue;
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
                continue;
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
        hival[i] = Eval::eval_n(&hand, 7);

        // La main basse n'est pas évaluée dans le 7-Card Stud standard
        loval[i] = LowHandVal { value: 0 };
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
        hival[i] = Eval::eval_n(&hand, 7);

        // Évaluation de la main basse (lowball 8 or better)
        loval[i] = std_deck_lowball8_eval(&hand, 7);
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
        hival[i] = HandVal { value: 0 };

        // Évaluation de la main basse (A-5 lowball)
        loval[i] = std_deck_lowball_eval(&hand, 7);
    }
}

pub fn inner_loop_5draw(
    pockets: &[JokerDeckCardMask],
    unshared_cards: &[JokerDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        // Vérifiez si l'index est dans les limites des cartes non partagées
        if i >= unshared_cards.len() {
            eprintln!(
                "Nombre insuffisant de cartes non partagées pour l'index {}",
                i
            );
            continue;
        }

        // Fusionnez les cartes en main avec les cartes non partagées
        let hand = pocket.clone() | unshared_cards[i].clone();

        // Évaluez la main pour obtenir la valeur haute
        hival[i] = EvalJoker::eval_n(hand, 5);

        // Attribuez une valeur fixe pour la valeur basse, car il semble que la macro originale ne fasse pas d'évaluation basse
        loval[i] = LowHandVal { value: 0 };
    }
}

pub fn inner_loop_5draw8(
    pockets: &[JokerDeckCardMask],
    unshared_cards: &[JokerDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        // Vérifiez si l'index est dans les limites des cartes non partagées
        if i >= unshared_cards.len() {
            eprintln!(
                "Nombre insuffisant de cartes non partagées pour l'index {}",
                i
            );
            continue;
        }

        // Fusionnez les cartes en main avec les cartes non partagées
        let hand = *pocket | unshared_cards[i]; // Assume que l'opération '|' est surchargée pour `JokerDeckCardMask`

        // Évaluez la main pour obtenir la valeur haute en utilisant les règles spécifiques aux jokers
        hival[i] = EvalJoker::eval_n(hand, 5);

        // Évaluez la main pour obtenir la valeur basse en utilisant les règles de lowball 8
        loval[i] = joker_lowball8_eval(&hand, 5);
    }
}

pub fn inner_loop_5drawnsq(
    pockets: &[JokerDeckCardMask],
    unshared_cards: &[JokerDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        // Vérifiez si l'index est dans les limites des cartes non partagées
        if i >= unshared_cards.len() {
            eprintln!(
                "Nombre insuffisant de cartes non partagées pour l'index {}",
                i
            );
            continue;
        }

        // Fusionnez les cartes en main avec les cartes non partagées
        let hand = *pocket | unshared_cards[i]; // Assume que l'opération '|' est implémentée pour `JokerDeckCardMask`

        // Évaluez la main pour la valeur haute en utilisant les règles spécifiques aux jokers
        hival[i] = EvalJoker::eval_n(hand, 5);

        // Évaluez la main pour la valeur basse sans qualification spécifique
        loval[i] = joker_lowball_eval(&hand, 5); // Fonction hypothétique d'évaluation basse
    }
}

pub fn inner_loop_lowball(
    pockets: &[JokerDeckCardMask],
    unshared_cards: &[JokerDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        // Vérifiez si l'index est dans les limites des cartes non partagées
        if i >= unshared_cards.len() {
            eprintln!(
                "Nombre insuffisant de cartes non partagées pour l'index {}",
                i
            );
            continue;
        }

        // Fusionnez les cartes en main avec les cartes non partagées
        let hand = *pocket | unshared_cards[i]; // Assume que l'opération '|' est implémentée pour `JokerDeckCardMask`

        // La valeur haute n'est pas pertinente dans ce contexte, attribuez HandVal_NOTHING
        hival[i] = HandVal { value: 0 };

        // Évaluez la main pour la valeur basse en utilisant la fonction d'évaluation lowball
        loval[i] = joker_lowball_eval(&hand, 5); // Supposons que cette fonction retourne une `LowHandVal`
    }
}

pub fn inner_loop_lowball27(
    pockets: &[StdDeckCardMask],
    unshared_cards: &[StdDeckCardMask],
    hival: &mut [HandVal],
    loval: &mut [LowHandVal],
) {
    for (i, pocket) in pockets.iter().enumerate() {
        if i >= unshared_cards.len() {
            eprintln!(
                "Nombre insuffisant de cartes non partagées pour l'index {}",
                i
            );
            continue;
        }

        // Fusionnez les cartes en main avec les cartes non partagées
        let hand = pocket.clone() | unshared_cards[i].clone();

        // La valeur haute n'est pas pertinente dans ce contexte, attribuez HandVal_NOTHING
        hival[i] = HandVal { value: 0 };

        // Évaluez la main pour la valeur basse en utilisant la fonction d'évaluation lowball 2-7
        let hand_val_result = std_deck_lowball27_eval(&hand, 5);

        // Convertissez HandVal en LowHandVal directement ici
        loval[i] = LowHandVal {
            value: hand_val_result.value,
        };
    }
}

// Fonction d'évaluation exhaustive
// Définissez une erreur personnalisée pour gérer divers scénarios d'erreur dans la fonction
#[derive(Debug)]
pub enum EnumError {
    TooManyPlayers,
    UnsupportedGameType,
    UnsupportedBoardConfiguration,
    OtherError(String), // Pour gérer d'autres types d'erreurs
}

impl fmt::Display for EnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EnumError::TooManyPlayers => write!(f, "Too many players"),
            EnumError::UnsupportedGameType => write!(f, "Unsupported game type"),
            EnumError::UnsupportedBoardConfiguration => {
                write!(f, "Unsupported board configuration")
            }
            EnumError::OtherError(ref cause) => write!(f, "Error: {}", cause),
        }
    }
}

impl Error for EnumError {}

// Fonction d'évaluation par échantillonnage adaptée pour Rust
pub fn enum_sample(
    game: Game,
    pockets: &[StdDeckCardMask],
    board: StdDeckCardMask,
    dead: StdDeckCardMask,
    npockets: usize,
    nboard: usize,
    niter: usize,
    orderflag: bool,
    result: &mut EnumResult,
) -> Result<(), EnumError> {
    if npockets > ENUM_MAXPLAYERS {
        return Err(EnumError::TooManyPlayers);
    }
    // Réinitialisez les résultats
    result.clear();

    // Le mode d'ordonnancement est déterminé par le type de jeu
    let mode = match game {
        Game::Holdem | Game::Omaha | Game::Omaha5 | Game::Omaha6 | Game::Stud7 | Game::Draw5 => {
            EnumOrderingMode::Hi
        }
        Game::Razz | Game::Lowball | Game::Lowball27 => EnumOrderingMode::Lo,
        Game::Holdem8
        | Game::Omaha8
        | Game::Omaha85
        | Game::Stud78
        | Game::Stud7nsq
        | Game::Draw58
        | Game::Draw5nsq => EnumOrderingMode::Hilo,
        _ => return Err(EnumError::UnsupportedGameType),
    };

    // Allocation des ressources pour le résultat en fonction du mode d'ordonnancement
    if orderflag {
        result.allocate_resources(npockets, mode)?;
    }

    match game {
        Game::Holdem => {
            result.simulate_holdem_game(pockets, board, dead, npockets, nboard, niter)?;
        }
        Game::Omaha => {
            //simulate_omaha_game(pockets, board, dead, npockets, nboard, niter, result)?;
        }
        // Ajoutez d'autres branches pour d'autres types de jeux ici...
        _ => return Err(EnumError::UnsupportedGameType),
    }
    Ok(())
}

impl EnumResult {
    pub fn clear(&mut self) {
        // Réinitialiser les champs simples à leur valeur par défaut
        self.game = Game::Holdem;
        self.sample_type = SampleType::Exhaustive;
        self.nsamples = 0;
        self.nplayers = 0;

        // Réinitialiser les tableaux à 0
        self.nwinhi = [0; ENUM_MAXPLAYERS];
        self.ntiehi = [0; ENUM_MAXPLAYERS];
        self.nlosehi = [0; ENUM_MAXPLAYERS];
        self.nwinlo = [0; ENUM_MAXPLAYERS];
        self.ntielo = [0; ENUM_MAXPLAYERS];
        self.nloselo = [0; ENUM_MAXPLAYERS];
        self.nscoop = [0; ENUM_MAXPLAYERS];

        // Réinitialiser les tableaux multi-dimensionnels à 0
        self.nsharehi = [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS];
        self.nsharelo = [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS];
        self.nshare = [[[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS];

        // Réinitialiser les valeurs d'équité à 0.0
        self.ev = [0.0; ENUM_MAXPLAYERS];

        // Réinitialiser le pointeur d'ordonnancement à None
        self.ordering = None;
    }

    pub fn allocate_resources(
        &mut self,
        nplayers: usize,
        mode: EnumOrderingMode,
    ) -> Result<(), EnumError> {
        if nplayers > ENUM_ORDERING_MAXPLAYERS && mode != EnumOrderingMode::Hilo {
            return Err(EnumError::OtherError(
                "Nombre de joueurs trop élevé pour le mode non-Hilo".to_string(),
            ));
        } else if nplayers > ENUM_ORDERING_MAXPLAYERS_HILO && mode == EnumOrderingMode::Hilo {
            return Err(EnumError::OtherError(
                "Nombre de joueurs trop élevé pour le mode Hilo".to_string(),
            ));
        }

        let nentries = match mode {
            EnumOrderingMode::Hilo => enum_ordering_nentries_hilo(nplayers),
            _ => enum_ordering_nentries(nplayers),
        };

        if nentries <= 0 {
            return Err(EnumError::OtherError(
                "Nombre d'entrées invalide".to_string(),
            ));
        }

        // Créez une instance de EnumOrdering
        let ordering = EnumOrdering {
            mode,
            nplayers,
            nentries: nentries as usize,
            hist: vec![0; nentries as usize],
        };
        let ordering_non_null = NonNull::new(Box::leak(Box::new(ordering)))
            .expect("Failed to convert EnumOrdering to NonNull<EnumOrdering>");

        // Affectez la valeur NonNull à self.ordering
        self.ordering = Some(ordering_non_null);

        Ok(())
    }

    pub fn simulate_holdem_game(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: StdDeckCardMask,
        dead: StdDeckCardMask,
        npockets: usize,
        nboard: usize,
        niter: usize,
    ) -> Result<(), EnumError> {
        let mut rng = thread_rng();

        // Création d'un deck complet moins les cartes déjà sur le board et les cartes mortes
        let mut deck = (0..STD_DECK_N_CARDS)
            .filter_map(|i| {
                let card_mask = StdDeckCardMask::get_mask(i);
                if (board.mask & card_mask.mask) == 0 && (dead.mask & card_mask.mask) == 0 {
                    Some(card_mask)
                } else {
                    None
                }
            })
            .cloned() // Clone chaque référence StdDeckCardMask pour créer une valeur StdDeckCardMask
            .collect::<Vec<StdDeckCardMask>>();

        let num_cards = 5 - nboard;
        if num_cards > 0 {
            // Simulation des tirages Monte Carlo
            for _ in 0..niter {
                deck.shuffle(&mut rng);

                let mut monte_carlo_board = board.clone();
                for card in deck.iter().take(num_cards) {
                    monte_carlo_board = monte_carlo_board | card.clone();
                }

                // Évaluation des mains avec le tableau de simulation
                self.evaluate_hands(pockets, &monte_carlo_board, npockets)?;
            }
        } else {
            // Pas besoin de simulation, évaluer directement avec le tableau existant
            self.evaluate_hands(pockets, &board, npockets)?;
        }

        // Mise à jour de nsamples pour refléter le nombre d'itérations effectuées
        self.nsamples += niter as u32;

        Ok(())
    }

    pub fn evaluate_hands(
        &mut self,
        pockets: &[StdDeckCardMask],
        board: &StdDeckCardMask,
        npockets: usize,
    ) -> Result<(), EnumError> {
        for (i, pocket) in pockets.iter().enumerate().take(npockets) {
            let hand = pocket.clone() | board.clone(); // Combinez les cartes en main avec le tableau
            let hand_value = Eval::eval_n(&hand, 7); // Évaluez la main
                                                     // Passez tous les arguments nécessaires à update_statistics
            self.update_statistics(i, hand_value, pockets, board, npockets);
        }

        Ok(())
    }

    // Supposons que vous avez une fonction pour mettre à jour les statistiques de jeu
    pub fn update_statistics(
        &mut self,
        player_index: usize,
        hand_value: HandVal,
        pockets: &[StdDeckCardMask],
        board: &StdDeckCardMask,
        npockets: usize,
    ) {
        // Initialiser les compteurs pour les victoires, égalités et défaites
        let mut wins = 0;
        let mut ties = 0;
        let mut losses = 0;

        // Comparer la main du joueur actuel avec celles des autres joueurs
        for (i, other_pocket) in pockets.iter().enumerate().take(npockets) {
            if i != player_index {
                // S'assurer de ne pas comparer le joueur à lui-même
                let other_hand = *other_pocket | *board;
                let other_hand_value = Eval::eval_n(&other_hand, 7);

                // Mettre à jour les compteurs en fonction de la comparaison des valeurs des mains
                if hand_value > other_hand_value {
                    wins += 1;
                } else if hand_value < other_hand_value {
                    losses += 1;
                } else {
                    ties += 1;
                }
            }
        }

        // Mettre à jour les statistiques globales pour le joueur
        self.nwinhi[player_index] += wins;
        self.ntiehi[player_index] += ties;
        self.nlosehi[player_index] += losses;

        // Calculer et mettre à jour l'équité (EV) pour le joueur
        let total_opponents = (npockets - 1) as f64; // Nombre total d'opposants
        let win_rate = wins as f64 / total_opponents;
        let tie_rate = ties as f64 / total_opponents;

        // L'équité est la somme de la probabilité de gagner et la moitié de la probabilité d'égalité (puisque les égalités sont partagées)
        let equity = win_rate + (tie_rate / 2.0);
        self.ev[player_index] += equity;
    }

    pub fn print_ordering(&self, terse: bool) {
        if let Some(ordering_ptr) = self.ordering {
            let ordering = unsafe { ordering_ptr.as_ref() }; // Utilisation sécurisée de unsafe pour déréférencer le pointeur NonNull

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

    // Méthode pour afficher les résultats d'une manière détaillée
    pub fn enum_result_print(&self, _pockets: &[StdDeckCardMask], _board: StdDeckCardMask) {
        let gp = self.game.game_params(); // Assumant que Game implémente une méthode game_params qui retourne Option<GameParams>
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

            // Affichage des résultats pour chaque joueur
            for i in 0..self.nplayers as usize {
                println!(
                    "{:width$} {:7} {:6.2}   {:7} {:6.2}   {:7} {:6.2}     {:5.3}",
                    "cards", // Remplacer par la représentation des cartes
                    self.nwinhi[i],
                    100.0 * self.nwinhi[i] as f64 / self.nsamples as f64,
                    self.nlosehi[i],
                    100.0 * self.nlosehi[i] as f64 / self.nsamples as f64,
                    self.ntiehi[i],
                    100.0 * self.ntiehi[i] as f64 / self.nsamples as f64,
                    self.ev[i] / self.nsamples as f64,
                    width = width as usize
                );
            }
        } else {
            println!("enumResultPrint: invalid game type");
        }
    }

    // Méthode pour afficher les résultats de manière concise
    pub fn enum_result_print_terse(&self, _pockets: &[StdDeckCardMask], _board: StdDeckCardMask) {
        print!("EV {}: ", self.nplayers);
        for &ev in &self.ev[0..self.nplayers as usize] {
            print!("{:8.6} ", ev / self.nsamples as f64);
        }
        println!();
    }
}

// so you might need to define a conversion method or use a match statement
impl EnumOrderingMode {
    fn as_u32(&self) -> u32 {
        match self {
            EnumOrderingMode::Hi => 1,
            EnumOrderingMode::Lo => 2,
            EnumOrderingMode::Hilo => 3,
            EnumOrderingMode::None => 0, // Ajoutez cette ligne pour gérer le cas `None`
        }
    }
}
impl Game {
    pub fn game_params(self) -> Option<GameParams> {
        match self {
            Game::Holdem => Some(GameParams {
                game: Game::Holdem,
                minpocket: 2,
                maxpocket: 2,
                maxboard: 5,
                haslopot: 0,
                hashipot: 1,
                name: "Holdem Hi".to_string(),
            }),
            Game::Holdem8 => Some(GameParams {
                game: Game::Holdem8,
                minpocket: 2,
                maxpocket: 2,
                maxboard: 5,
                haslopot: 1, // Holdem Hi/Lo a un pot bas
                hashipot: 1, // et un pot haut
                name: "Holdem Hi/Low 8-or-better".to_string(),
            }),
            Game::Omaha => Some(GameParams {
                game: Game::Omaha,
                minpocket: 4, // Omaha a 4 cartes de poche
                maxpocket: 4,
                maxboard: 5, // et un tableau de 5 cartes
                haslopot: 0, // Omaha Hi n'a pas de pot bas
                hashipot: 1, // mais a un pot haut
                name: "Omaha Hi".to_string(),
            }),
            // Ajoutez d'autres variantes de jeu ici, si nécessaire
            // omaha 5 cards hi
            Game::Omaha5 => Some(GameParams {
                game: Game::Omaha5,
                minpocket: 5,
                maxpocket: 5,
                maxboard: 5,
                haslopot: 0,
                hashipot: 1,
                name: "Omaha 5cards Hi".to_string(),
            }),
            // omaha 6 cards hi
            Game::Omaha6 => Some(GameParams {
                game: Game::Omaha6,
                minpocket: 6,
                maxpocket: 6,
                maxboard: 5,
                haslopot: 0,
                hashipot: 1,
                name: "Omaha 6cards Hi".to_string(),
            }),
            // Exemple pour Omaha Hi/Lo
            Game::Omaha8 => Some(GameParams {
                game: Game::Omaha8,
                minpocket: 4,
                maxpocket: 4,
                maxboard: 5,
                haslopot: 1, // Omaha Hi/Lo a un pot bas
                hashipot: 1, // et un pot haut
                name: "Omaha Hi/Low 8-or-better".to_string(),
            }),
            // omaha 5 cards hilow
            Game::Omaha85 => Some(GameParams {
                game: Game::Omaha85,
                minpocket: 5,
                maxpocket: 5,
                maxboard: 5,
                haslopot: 1,
                hashipot: 1,
                name: "Omaha 5cards Hi/Low".to_string(),
            }),
            // stud 7 cards hi
            Game::Stud7 => Some(GameParams {
                game: Game::Stud7,
                minpocket: 3,
                maxpocket: 7,
                maxboard: 0,
                haslopot: 0,
                hashipot: 1,
                name: "Stud 7cards Hi".to_string(),
            }),
            // stud 7 cards hilow
            Game::Stud78 => Some(GameParams {
                game: Game::Stud78,
                minpocket: 3,
                maxpocket: 7,
                maxboard: 0,
                haslopot: 1,
                hashipot: 1,
                name: "Stud 7cards Hi/Low".to_string(),
            }),
            // stud 7 cards hi/lo no qualifier
            Game::Stud7nsq => Some(GameParams {
                game: Game::Stud7nsq,
                minpocket: 3,
                maxpocket: 7,
                maxboard: 0,
                haslopot: 1,
                hashipot: 1,
                name: "Stud 7cards Hi/Low no qualifier".to_string(),
            }),
            // Razz
            Game::Razz => Some(GameParams {
                game: Game::Razz,
                minpocket: 3,
                maxpocket: 7,
                maxboard: 0,
                haslopot: 1,
                hashipot: 0,
                name: "Razz (7-card Stud A-5 Low)".to_string(),
            }),
            // Draw 5 cards
            Game::Draw5 => Some(GameParams {
                game: Game::Draw5,
                minpocket: 0,
                maxpocket: 5,
                maxboard: 0,
                haslopot: 0,
                hashipot: 1,
                name: "5-card Draw Hi with joker".to_string(),
            }),
            // Draw 5 cards hi/lo
            Game::Draw58 => Some(GameParams {
                game: Game::Draw58,
                minpocket: 0,
                maxpocket: 5,
                maxboard: 0,
                haslopot: 1,
                hashipot: 1,
                name: "5-card Draw Hi/Low 8-or-better with joker".to_string(),
            }),
            // Draw 5 cards no qualifier
            Game::Draw5nsq => Some(GameParams {
                game: Game::Draw5nsq,
                minpocket: 0,
                maxpocket: 5,
                maxboard: 0,
                haslopot: 1,
                hashipot: 1,
                name: "5-card Draw Hi/Low no qualifier with joker".to_string(),
            }),
            // Lowball
            Game::Lowball => Some(GameParams {
                game: Game::Lowball,
                minpocket: 0,
                maxpocket: 5,
                maxboard: 0,
                haslopot: 1,
                hashipot: 0,
                name: "5-card Draw A-5 Lowball with joker".to_string(),
            }),
            // Lowball 27
            Game::Lowball27 => Some(GameParams {
                game: Game::Lowball27,
                minpocket: 0,
                maxpocket: 5,
                maxboard: 0,
                haslopot: 1,
                hashipot: 0,
                name: "5-card Draw 2-7 Lowball".to_string(),
            }),
            // Gérer les cas non spécifiés ou non supportés
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Importez toutes les fonctions et structures nécessaires de votre bibliothèque.
    use crate::StdDeck;
    #[test]
    fn test_holdem_evaluation_ac7c_vs_5s4s() {
        // Initialiser les mains et le tableau
        let pocket_str1 = "Ac7c";
        let pocket_str2 = "5s4s";
        let hand1 = StdDeck::string_to_mask(pocket_str1).unwrap().0;
        let hand2 = StdDeck::string_to_mask(pocket_str2).unwrap().0;
        let board = StdDeckCardMask::new(); // Commencez avec un tableau vide
        let dead = StdDeckCardMask::new(); // Aucune carte morte pour commencer

        // Définissez les valeurs manquantes de `game`, `enum_type`, et `npockets`
        let game = Game::Holdem; // Exemple, utilisez la valeur appropriée pour votre cas
        let enum_type = SampleType::Exhaustive; // Exemple, utilisez la valeur appropriée pour votre cas
        let npockets = 2; // Puisque vous avez deux mains

        // Initialiser les résultats
        let mut result = EnumResult {
            game,
            sample_type: enum_type,
            nsamples: 0,
            nplayers: npockets as u32,
            nwinhi: [0; ENUM_MAXPLAYERS],
            ntiehi: [0; ENUM_MAXPLAYERS],
            nlosehi: [0; ENUM_MAXPLAYERS],
            nwinlo: [0; ENUM_MAXPLAYERS],
            ntielo: [0; ENUM_MAXPLAYERS],
            nloselo: [0; ENUM_MAXPLAYERS],
            nscoop: [0; ENUM_MAXPLAYERS],
            nsharehi: [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
            nsharelo: [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
            nshare: [[[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
            ev: [0.0; ENUM_MAXPLAYERS],
            ordering: None,
        };

        // Simuler les 10000 itérations
        const N_ITER: usize = 100;
        let nboard = 0; // Nombre de cartes déjà présentes sur le tableau (0 dans ce cas)
        for _ in 0..N_ITER {
            // Corrigez l'appel à `simulate_holdem_game` avec tous les arguments nécessaires
            result.simulate_holdem_game(&[hand1, hand2], board, dead, npockets, nboard, N_ITER);
        }

        // Vérifier les résultats
        const EXPECTED_WIN_HAND1: usize = 6113;
        const EXPECTED_WIN_HAND2: usize = 3800;
        const EXPECTED_TIE: usize = 56;

        //assert_eq!(result.nwinhi[0], EXPECTED_WIN_HAND1 as u32, "Le nombre de victoires de la main 1 ne correspond pas");
        println!(
            "Le nombre de victoires de la main 1 est : {}",
            result.nwinhi[0]
        );
        //assert_eq!(result.nwinhi[1], EXPECTED_WIN_HAND2 as u32, "Le nombre de victoires de la main 2 ne correspond pas");
        println!(
            "Le nombre de victoires de la main 2 est : {}",
            result.nwinhi[1]
        );
        //assert_eq!(result.ntiehi[0], EXPECTED_TIE as u32, "Le nombre d'égalités ne correspond pas");
        println!("Le nombre d'égalités est : {}", result.ntiehi[0]);
        //assert_eq!(result.ntiehi[1], EXPECTED_TIE as u32, "Le nombre d'égalités ne correspond pas");
        println!("Le nombre d'égalités est : {}", result.ntiehi[1]);
    }
}
