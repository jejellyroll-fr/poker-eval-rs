use super::joker_low::joker_lowball_eval;
use crate::handval_low::{LowHandVal, LOW_HAND_VAL_WORST_EIGHT};
use crate::tables::t_jokercardmasks::JokerDeckCardMask;

pub fn joker_lowball8_eval(cards: &JokerDeckCardMask, n_cards: usize) -> LowHandVal {
    let loval = joker_lowball_eval(cards, n_cards); // Use the existing joker lowball evaluation function

    // Check if the hand value qualifies as an "8-low" or better
    if loval.value <= LOW_HAND_VAL_WORST_EIGHT {
        loval
    } else {
        LowHandVal {
            value: crate::handval_low::LOW_HAND_VAL_NOTHING,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::{JokerDeck, JOKER_DECK_JOKER};
    use crate::handval_low::LOW_HAND_VAL_NOTHING;

    fn make_mask(cards_str: &str) -> JokerDeckCardMask {
        // Copied from eval_joker_low.rs tests (shared logic helper)
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
            let (m, _) = JokerDeck::string_to_mask(&clean_str).unwrap_or_else(|_| {
                let (_std_m, _) =
                    crate::deck::StdDeck::string_to_mask(&clean_str).unwrap_or_else(|e| {
                        panic!(
                            "Failed to parse standard card string '{}': {:?}",
                            clean_str, e
                        )
                    });
                // Hack: since we fixed deck_std, we assume JokerDeck uses compatible layout for first 52 cards
                // But we can't easily convert StdDeckCardMask to JokerDeckCardMask without manual bit copy
                // or relying on identical layout.
                // Let's rely on JokerDeck::string_to_mask working now.
                panic!("JokerDeck::string_to_mask failed");
            });
            mask = m;
        }

        if has_joker {
            mask.set(JOKER_DECK_JOKER);
        }
        mask
    }

    #[test]
    fn test_joker_low8_wheel() {
        // 2-3-4-5-Joker -> A-2-3-4-5 (Wheel) -> Qualifies
        let mask = make_mask("2s3d4c5hXx");
        let val = joker_lowball8_eval(&mask, 5);
        assert_ne!(val.value, LOW_HAND_VAL_NOTHING);
    }

    #[test]
    fn test_joker_low8_does_not_qualify() {
        // 2-3-4-9-Joker -> A-2-3-4-9 (9-low) -> Does NOT qualify for 8-or-better
        let mask = make_mask("2s3d4c9hXx");
        let val = joker_lowball8_eval(&mask, 5);
        println!("Value: {}", val.value);
        assert_eq!(val.value, LOW_HAND_VAL_NOTHING);
    }

    #[test]
    fn test_joker_low8_qualifies_exact() {
        // 2-3-4-8-Joker -> A-2-3-4-8 (8-low) -> Qualifies
        let mask = make_mask("2s3d4c8hXx");
        let val = joker_lowball8_eval(&mask, 5);
        assert_ne!(val.value, LOW_HAND_VAL_NOTHING);
    }
}
