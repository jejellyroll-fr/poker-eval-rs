use crate::t_cardmasks::StdDeckCardMask;
use crate::t_jokercardmasks::JokerDeckCardMask;
use crate::combinaison::*;
use std::ops::BitOr;



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
        if dead_cards.iter().any(|dead_card| dead_card.mask() == card.mask()) {
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
        if dead_cards.iter().any(|dead_card| dead_card.mask() == card1.mask()) {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards.iter().any(|dead_card| dead_card.mask() == card2.mask()) {
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
        if dead_cards.iter().any(|dead_card| dead_card.mask() == card1.mask()) {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards.iter().any(|dead_card| dead_card.mask() == card2.mask()) {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards.iter().any(|dead_card| dead_card.mask() == card3.mask()) {
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
        if dead_cards.iter().any(|dead_card| dead_card.mask() == card1.mask()) {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards.iter().any(|dead_card| dead_card.mask() == card2.mask()) {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards.iter().any(|dead_card| dead_card.mask() == card3.mask()) {
                    continue;
                }

                for i4 in 0..i3 {
                    let card4 = &deck[i4];
                    if dead_cards.iter().any(|dead_card| dead_card.mask() == card4.mask()) {
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
        if dead_cards.iter().any(|dead_card| dead_card.mask() == card1.mask()) {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards.iter().any(|dead_card| dead_card.mask() == card2.mask()) {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards.iter().any(|dead_card| dead_card.mask() == card3.mask()) {
                    continue;
                }

                for i4 in 0..i3 {
                    let card4 = &deck[i4];
                    if dead_cards.iter().any(|dead_card| dead_card.mask() == card4.mask()) {
                        continue;
                    }

                    for i5 in 0..i4 {
                        let card5 = &deck[i5];
                        if dead_cards.iter().any(|dead_card| dead_card.mask() == card5.mask()) {
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
        if dead_cards.iter().any(|dead_card| dead_card.mask() == card1.mask()) {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards.iter().any(|dead_card| dead_card.mask() == card2.mask()) {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards.iter().any(|dead_card| dead_card.mask() == card3.mask()) {
                    continue;
                }

                for i4 in 0..i3 {
                    let card4 = &deck[i4];
                    if dead_cards.iter().any(|dead_card| dead_card.mask() == card4.mask()) {
                        continue;
                    }

                    for i5 in 0..i4 {
                        let card5 = &deck[i5];
                        if dead_cards.iter().any(|dead_card| dead_card.mask() == card5.mask()) {
                            continue;
                        }

                        for i6 in 0..i5 {
                            let card6 = &deck[i6];
                            if dead_cards.iter().any(|dead_card| dead_card.mask() == card6.mask()) {
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
        if dead_cards.iter().any(|dead_card| dead_card.mask() == card1.mask()) {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards.iter().any(|dead_card| dead_card.mask() == card2.mask()) {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards.iter().any(|dead_card| dead_card.mask() == card3.mask()) {
                    continue;
                }

                for i4 in 0..i3 {
                    let card4 = &deck[i4];
                    if dead_cards.iter().any(|dead_card| dead_card.mask() == card4.mask()) {
                        continue;
                    }

                    for i5 in 0..i4 {
                        let card5 = &deck[i5];
                        if dead_cards.iter().any(|dead_card| dead_card.mask() == card5.mask()) {
                            continue;
                        }

                        for i6 in 0..i5 {
                            let card6 = &deck[i6];
                            if dead_cards.iter().any(|dead_card| dead_card.mask() == card6.mask()) {
                                continue;
                            }

                            for i7 in 0..i6 {
                                let card7 = &deck[i7];
                                if dead_cards.iter().any(|dead_card| dead_card.mask() == card7.mask()) {
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
                !dead_cards.iter().any(|dead_card| dead_card.mask() == deck[i].mask())
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
    let mut live_cards: Vec<T> = decks.iter().flatten()
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
