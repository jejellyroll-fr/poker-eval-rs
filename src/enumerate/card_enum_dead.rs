//! Functions for enumerating card combinations while excluding dead cards.

use super::CardMask;

/// Enumerates single cards from a deck, skipping dead cards.
pub(crate) fn enumerate_1_cards_d<T, F>(deck: &[T], dead_cards: T, mut action: F)
where
    T: CardMask,
    F: FnMut(&T),
{
    for card in deck {
        if dead_cards.overlaps(card) {
            continue; // Skip dead cards
        }
        action(card);
    }
}

/// Enumerates all combinations of 2 cards from a deck, skipping dead cards.
#[allow(dead_code)]
pub(crate) fn enumerate_2_cards_d<T, F>(deck: &[T], dead_cards: T, mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards.overlaps(card1) {
            continue;
        }

        for card2 in deck.iter().take(i1) {
            if dead_cards.overlaps(card2) {
                continue;
            }

            action(card1, card2);
        }
    }
}

/// Enumerates all combinations of 3 cards from a deck, skipping dead cards.
#[allow(dead_code)]
pub(crate) fn enumerate_3_cards_d<T, F>(deck: &[T], dead_cards: T, mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards.overlaps(card1) {
            continue;
        }

        for (i2, card2) in deck.iter().enumerate().take(i1) {
            if dead_cards.overlaps(card2) {
                continue;
            }

            for card3 in deck.iter().take(i2) {
                if dead_cards.overlaps(card3) {
                    continue;
                }

                action(card1, card2, card3);
            }
        }
    }
}

/// Enumerates all combinations of 4 cards from a deck, skipping dead cards.
pub(crate) fn enumerate_4_cards_d<T, F>(deck: &[T], dead_cards: T, mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards.overlaps(card1) {
            continue;
        }

        for (i2, card2) in deck.iter().enumerate().take(i1) {
            if dead_cards.overlaps(card2) {
                continue;
            }

            for (i3, card3) in deck.iter().enumerate().take(i2) {
                if dead_cards.overlaps(card3) {
                    continue;
                }

                for card4 in deck.iter().take(i3) {
                    if dead_cards.overlaps(card4) {
                        continue;
                    }

                    action(card1, card2, card3, card4);
                }
            }
        }
    }
}

/// Enumerates all combinations of 5 cards from a deck, skipping dead cards.
#[allow(dead_code)]
pub(crate) fn enumerate_5_cards_d<T, F>(deck: &[T], dead_cards: T, mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards.overlaps(card1) {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards.overlaps(card2) {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards.overlaps(card3) {
                    continue;
                }

                for i4 in 0..i3 {
                    let card4 = &deck[i4];
                    if dead_cards.overlaps(card4) {
                        continue;
                    }

                    for card5 in deck.iter().take(i4) {
                        if dead_cards.overlaps(card5) {
                            continue;
                        }

                        action(card1, card2, card3, card4, card5);
                    }
                }
            }
        }
    }
}

/// Enumerates all combinations of 6 cards from a deck, skipping dead cards.
#[allow(dead_code)]
pub(crate) fn enumerate_6_cards_d<T, F>(deck: &[T], dead_cards: T, mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards.overlaps(card1) {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards.overlaps(card2) {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards.overlaps(card3) {
                    continue;
                }

                for i4 in 0..i3 {
                    let card4 = &deck[i4];
                    if dead_cards.overlaps(card4) {
                        continue;
                    }

                    for i5 in 0..i4 {
                        let card5 = &deck[i5];
                        if dead_cards.overlaps(card5) {
                            continue;
                        }

                        for card6 in deck.iter().take(i5) {
                            if dead_cards.overlaps(card6) {
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

/// Enumerates all combinations of 7 cards from a deck, skipping dead cards.
#[allow(dead_code)]
pub(crate) fn enumerate_7_cards_d<T, F>(deck: &[T], dead_cards: T, mut action: F)
where
    T: CardMask,
    F: FnMut(&T, &T, &T, &T, &T, &T, &T),
{
    let n_cards = deck.len();
    for i1 in 0..n_cards {
        let card1 = &deck[i1];
        if dead_cards.overlaps(card1) {
            continue;
        }

        for i2 in 0..i1 {
            let card2 = &deck[i2];
            if dead_cards.overlaps(card2) {
                continue;
            }

            for i3 in 0..i2 {
                let card3 = &deck[i3];
                if dead_cards.overlaps(card3) {
                    continue;
                }

                for i4 in 0..i3 {
                    let card4 = &deck[i4];
                    if dead_cards.overlaps(card4) {
                        continue;
                    }

                    for i5 in 0..i4 {
                        let card5 = &deck[i5];
                        if dead_cards.overlaps(card5) {
                            continue;
                        }

                        for i6 in 0..i5 {
                            let card6 = &deck[i6];
                            if dead_cards.overlaps(card6) {
                                continue;
                            }

                            for card7 in deck.iter().take(i6) {
                                if dead_cards.overlaps(card7) {
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
/// Enumerates all combinations of N cards from a deck, skipping dead cards.
#[allow(dead_code)]
pub(crate) fn enumerate_n_cards_d<T, F>(deck: &[T], dead_cards: T, n: usize, mut action: F)
where
    T: CardMask,
    F: FnMut(&[T]),
{
    // Filter out dead cards from the deck first to simplify combination logic
    // OPTIMIZATION: Use bitmask check instead of linear search
    let valid_deck: Vec<T> = deck
        .iter()
        .filter(|card| !dead_cards.overlaps(card))
        .cloned()
        .collect();

    if valid_deck.len() < n {
        return;
    }

    let mut hand_buffer: Vec<T> = Vec::with_capacity(n);
    // Reuse buffer for hand construction
    crate::combinations::for_each_combination(valid_deck.len(), n, |indices| {
        hand_buffer.clear();
        for &i in indices {
            hand_buffer.push(valid_deck[i]);
        }
        action(&hand_buffer);
    });
}
