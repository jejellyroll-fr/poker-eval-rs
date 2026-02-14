use super::lowball::std_deck_lowball_eval;
use crate::deck::*;
use crate::handval_low::LowHandVal;
use crate::tables::t_jokercardmasks::JokerDeckCardMask;

// Evaluation function for lowball including a joker.
// `cards` represents the card mask potentially including a joker, `n_cards` is the number of cards in hand
pub fn joker_lowball_eval(cards: &JokerDeckCardMask, n_cards: usize) -> LowHandVal {
    let ss = cards.spades();
    let sh = cards.hearts();
    let sd = cards.diamonds();
    let sc = cards.clubs();

    let ranks = sc | ss | sd | sh;
    let mut rank: u64 = 0;

    let mut s_cards = cards.to_std();

    if !cards.card_is_set(JOKER_DECK_JOKER) {
        return std_deck_lowball_eval(&s_cards, n_cards);
    }

    if (ranks & (1 << JOKER_DECK_RANK_ACE as u64)) == 0 {
        rank = 1 << JOKER_DECK_RANK_ACE as u64;
    } else {
        for r in JOKER_DECK_RANK_2..=JOKER_DECK_RANK_KING {
            let bit = 1 << r as u64;
            if (ranks & bit) == 0 {
                rank = bit;
                break;
            }
        }
    }

    let rank_u16 = rank as u16;

    if (sc & rank) == 0 {
        s_cards.set_clubs(sc as u16 | rank_u16);
    } else if (sd & rank) == 0 {
        s_cards.set_diamonds(sd as u16 | rank_u16);
    } else if (sh & rank) == 0 {
        s_cards.set_hearts(sh as u16 | rank_u16);
    } else if (ss & rank) == 0 {
        s_cards.set_spades(ss as u16 | rank_u16);
    }

    std_deck_lowball_eval(&s_cards, n_cards)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::{JokerDeck, JOKER_DECK_JOKER};
    use crate::handval_low::LOW_HAND_VAL_NOTHING;

    fn make_mask(cards_str: &str) -> JokerDeckCardMask {
        // Try parsing, if it fails (due to Xx bug), construct manually for standard cards
        // and add Joker if Xx is present.
        let mut mask = JokerDeckCardMask::new();
        let mut clean_str = String::new();
        let mut has_joker = false;

        let mut chars = cards_str.chars();
        while let Some(c1) = chars.next() {
            if let Some(c2) = chars.next() {
                let s = format!("{}{}", c1, c2);
                if s == "Xx" {
                    has_joker = true;
                } else {
                    clean_str.push_str(&s);
                }
            }
        }

        if !clean_str.is_empty() {
            // We can use StdDeck to parse standard cards or JokerDeck if it works for non-jokers
            // JokerDeck::string_to_mask works for standard cards
            let (m, _) = JokerDeck::string_to_mask(&clean_str).unwrap_or_else(|_| {
                // Fallback: use crate::deck_std::StdDeck
                let (_std_m, _) =
                    crate::deck::StdDeck::string_to_mask(&clean_str).unwrap_or_else(|e| {
                        panic!(
                            "Failed to parse standard card string '{}': {:?}",
                            clean_str, e
                        )
                    });
                // Convert std mask to joker mask?
                // JokerDeckCardMask has .to_std() but not from_std easily maybe?
                // Actually JokerDeckCardMask has the same layout for first 52 cards?
                // Let's rely on JokerDeck::string_to_mask working for standard cards.
                panic!("JokerDeck::string_to_mask failed for {}", clean_str);
            });
            mask = m;
        }

        if has_joker {
            mask.set(JOKER_DECK_JOKER);
        }
        mask
    }

    #[test]
    fn test_joker_as_ace() {
        // 2-3-4-5-Joker -> A-2-3-4-5 (Wheel)
        let mask = make_mask("2s3d4c5hXx");
        let val = joker_lowball_eval(&mask, 5);
        assert_ne!(val.value, LOW_HAND_VAL_NOTHING);

        // Check against actual Wheel
        let wheel_mask = make_mask("As2s3d4c5h");
        let wheel_val = joker_lowball_eval(&wheel_mask, 5);

        assert_eq!(val.value, wheel_val.value, "Joker should form a Wheel");
    }

    #[test]
    fn test_joker_fills_gap() {
        // A-2-4-5-Joker -> A-2-3-4-5
        let mask = make_mask("As2s4d5cXx");
        let val = joker_lowball_eval(&mask, 5);

        let wheel_mask = make_mask("As2s3d4d5c");
        let wheel_val = joker_lowball_eval(&wheel_mask, 5);
        assert_eq!(
            val.value, wheel_val.value,
            "Joker should fill the gap to form Wheel"
        );
    }

    #[test]
    fn test_joker_with_pair() {
        // A-A-2-3-Joker -> A-2-3-4-5 (Joker becomes 4 or 5?)
        // Cards: A, A, 2, 3. Joker.
        // Joker should NOT fix the pair. Joker fills a RANK.
        // Discarding duplicates is not what eval logic usually does for "best hand" from 5 cards?
        // Wait, lowball evaluation with 5 cards... if you have a pair, you have a pair.
        // A-A-2-3-Joker. Joker will pick best rank not present.
        // Present: A, 2, 3. Best missing: 4.
        // Hand becomes A, A, 2, 3, 4. This is a Pair of Aces.
        // It does NOT un-pair the Aces.

        let mask = make_mask("AsAc2d3hXx");
        let val = joker_lowball_eval(&mask, 5);

        // Should be worse than a non-pair hand
        let no_pair = make_mask("KsQcJd9h8s"); // King high
        let no_pair_val = joker_lowball_eval(&no_pair, 5);

        assert!(
            val > no_pair_val,
            "Pair should be worse (higher value) than NoPair"
        );
    }

    #[test]
    fn test_joker_deep_gap() {
        // 2-3-4-8-Joker -> 2-3-4-5-8 (Joker becomes 5, assumption: A is present? No.)
        // Present: 2, 3, 4, 8.
        // Missing: A, 5, 6, 7...
        // Best card is A.
        // So 2-3-4-8-Joker -> A-2-3-4-8. (8-low)

        let mask = make_mask("2s3s4s8sXx");
        let val = joker_lowball_eval(&mask, 5);

        let target = make_mask("As2s3s4s8s");
        let target_val = joker_lowball_eval(&target, 5);

        assert_eq!(val.value, target_val.value, "Joker should become Ace");
    }
}
