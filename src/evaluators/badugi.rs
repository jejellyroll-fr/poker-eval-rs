//! Badugi evaluator implementation.
//!
//! Badugi is a 4-card lowball variant where the goal is to have the lowest cards
//! with unique ranks and unique suits.

use crate::handval_low::LowHandVal;
use crate::tables::t_cardmasks::StdDeckCardMask;

/// Hand types for Badugi.
/// Since we use LowHandVal (smaller is better), 4-card (Badugi) has the smallest value.
pub enum BadugiHandType {
    Badugi = 0,    // 4 unique cards
    ThreeCard = 1, // 3 unique cards
    TwoCard = 2,   // 2 unique cards
    OneCard = 3,   // 1 unique card
}

/// Evaluates a 4-card hand for Badugi.
/// Returns a LowHandVal representing the best valid subset.
pub fn badugi_eval(mask: &StdDeckCardMask) -> LowHandVal {
    let mut best_subset_size = 0;
    let mut best_val = LowHandVal { value: 0xFFFFFFFF }; // Worst possible

    let cards = (0..52)
        .filter(|&i| mask.card_is_set(i))
        .collect::<Vec<usize>>();

    let n = cards.len();
    // For Badugi, we usually have exactly 4 cards in hand, but the evaluator should be robust.
    // Try all 2^n subsets
    for i in 1..(1 << n) {
        let mut subset = Vec::new();
        for (j, &card) in cards.iter().enumerate().take(n) {
            if (i & (1 << j)) != 0 {
                subset.push(card);
            }
        }

        if is_valid_badugi_subset(&subset) {
            let size = subset.len();
            let val = calculate_badugi_val(size, &subset);

            if size > best_subset_size {
                best_subset_size = size;
                best_val = val;
            } else if size == best_subset_size && val.value < best_val.value {
                best_val = val;
            }
        }
    }

    if best_subset_size == 0 {
        // Should not happen if mask is non-empty
        return LowHandVal { value: 0xFFFFFFFF };
    }

    best_val
}

fn is_valid_badugi_subset(subset: &[usize]) -> bool {
    let mut ranks = 0u16;
    let mut suits = 0u8;
    for &card in subset {
        let r = card % 13;
        let s = card / 13;
        if (ranks & (1 << r)) != 0 || (suits & (1 << s)) != 0 {
            return false;
        }
        ranks |= 1 << r;
        suits |= 1 << s;
    }
    true
}

fn calculate_badugi_val(size: usize, subset: &[usize]) -> LowHandVal {
    // Badugi uses A-5 lowball style ranking (A is low).
    // Standard LowHandVal uses 1-13 for ranks (A=1, 2=2...).
    let mut ranks = subset
        .iter()
        .map(|&c| (c % 13) as u8 + 1)
        .collect::<Vec<u8>>();
    ranks.sort_by(|a, b| b.cmp(a)); // Descending order for top-down comparison

    let hand_type = match size {
        4 => BadugiHandType::Badugi as u8,
        3 => BadugiHandType::ThreeCard as u8,
        2 => BadugiHandType::TwoCard as u8,
        _ => BadugiHandType::OneCard as u8,
    };

    // LowHandVal::new(hand_type, top, second, third, fourth, fifth)
    // For Badugi (4 cards), we'll put 0 in the fifth card slot.
    let top = ranks.first().cloned().unwrap_or(0);
    let second = ranks.get(1).cloned().unwrap_or(0);
    let third = ranks.get(2).cloned().unwrap_or(0);
    let fourth = ranks.get(3).cloned().unwrap_or(0);

    LowHandVal::new(hand_type, top, second, third, fourth, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::StdDeck;

    #[test]
    fn test_badugi_eval_basic() {
        let (m1, _) = StdDeck::string_to_mask("As 2d 3c 4h").unwrap(); // 4-3-2-A Badugi
        let (m2, _) = StdDeck::string_to_mask("As 2d 3c 5h").unwrap(); // 5-3-2-A Badugi

        let v1 = badugi_eval(&m1);
        let v2 = badugi_eval(&m2);

        assert!(
            v1.value < v2.value,
            "4-high Badugi should beat 5-high Badugi"
        );
    }

    #[test]
    fn test_badugi_eval_three_card() {
        let (m1, _) = StdDeck::string_to_mask("As 2s 3c 4h").unwrap(); // 4-3-A of diff suits vs A-2 of same suit. Best is 4-3-A (3-card)
        let (m2, _) = StdDeck::string_to_mask("Ks Qd Jc Th").unwrap(); // K-Q-J-T Badugi

        let v1 = badugi_eval(&m1); // 3-card hand
        let v2 = badugi_eval(&m2); // 4-card hand

        assert!(
            v2.value < v1.value,
            "Any 4-card hand should beat any 3-card hand"
        );
    }
}
