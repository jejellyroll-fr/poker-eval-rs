use crate::deck_std::STD_DECK_N_CARDS;
use crate::eval::Eval;
use crate::eval_low8::std_deck_lowball8_eval;
use crate::handval::HandVal;
use crate::handval_low::{LowHandVal, LOW_HAND_VAL_NOTHING};
use crate::t_cardmasks::StdDeckCardMask;

pub const OMAHA_MINHOLE: usize = 4;
pub const OMAHA_MAXHOLE: usize = 6;
pub const OMAHA_MINBOARD: usize = 3;
pub const OMAHA_MAXBOARD: usize = 5;

// Assuming we have a Rust equivalent of StdDeck_CardMask, HandVal, and LowHandVal
// and related functionality.
// ...

pub fn std_deck_omaha_hi_low8_eval(
    hole: StdDeckCardMask,
    board: StdDeckCardMask,
    hival: &mut Option<HandVal>,
    loval: &mut Option<LowHandVal>,
) -> Result<(), &'static str> {
    let mut allcards = StdDeckCardMask::new();
    allcards.or(&hole);
    allcards.or(&board);

    // Utilisez std_deck_lowball8_eval pour évaluer la main basse
    let allval = std_deck_lowball8_eval(&allcards, STD_DECK_N_CARDS); // Ici, le nombre de cartes est STD_DECK_N_CARDS, ajustez si nécessaire
    let eligible_for_low = allval
        != LowHandVal {
            value: LOW_HAND_VAL_NOTHING,
        };

    let mut hole_cards: Vec<StdDeckCardMask> = Vec::new();
    let mut board_cards: Vec<StdDeckCardMask> = Vec::new();

    for i in 0..STD_DECK_N_CARDS {
        if hole.card_is_set(i) {
            if hole_cards.len() >= OMAHA_MAXHOLE {
                return Err("Too many hole cards");
            }
            let mut card_mask = StdDeckCardMask::new();
            card_mask.set(i);
            hole_cards.push(card_mask);
        }
        if board.card_is_set(i) {
            if hole.card_is_set(i) {
                return Err("Same card in hole and board");
            }
            if board_cards.len() >= OMAHA_MAXBOARD {
                return Err("Too many board cards");
            }
            let mut card_mask = StdDeckCardMask::new();
            card_mask.set(i);
            board_cards.push(card_mask);
        }
    }

    if hole_cards.len() < OMAHA_MINHOLE || hole_cards.len() > OMAHA_MAXHOLE {
        return Err("Wrong number of hole cards");
    }
    if board_cards.len() < OMAHA_MINBOARD || board_cards.len() > OMAHA_MAXBOARD {
        return Err("Wrong number of board cards");
    }

    // La logique pour évaluer les mains hautes et basses
    let mut best_hi = HandVal { value: 0 }; // Assurez-vous que c'est la valeur appropriée pour "rien"
    let mut best_lo = LowHandVal {
        value: LOW_HAND_VAL_NOTHING,
    }; // Assurez-vous que c'est la valeur appropriée pour "rien"

    for h1 in 0..hole_cards.len() {
        for h2 in h1 + 1..hole_cards.len() {
            for b1 in 0..board_cards.len() {
                for b2 in (b1 + 1)..board_cards.len() {
                    for b3 in (b2 + 1)..board_cards.len() {
                        let mut n5 = StdDeckCardMask::new();
                        n5.or(&hole_cards[h1]);
                        n5.or(&hole_cards[h2]);
                        n5.or(&board_cards[b1]);
                        n5.or(&board_cards[b2]);
                        n5.or(&board_cards[b3]);

                        // Évaluation de la main haute
                        let cur_hi = Eval::eval_n(&n5, 5);
                        if cur_hi.value > best_hi.value || best_hi.value == 0 {
                            best_hi = cur_hi;
                        }

                        // Évaluation de la main basse si éligible
                        if eligible_for_low {
                            let cur_lo = std_deck_lowball8_eval(&n5, 5);
                            if cur_lo.value < best_lo.value || best_lo.value == LOW_HAND_VAL_NOTHING
                            {
                                best_lo = cur_lo;
                            }
                        }
                    }
                }
            }
        }
    }

    // Ici, vous pouvez mettre à jour hival et loval avec les meilleures mains hautes et basses trouvées
    *hival = Some(best_hi);
    *loval = Some(best_lo);

    Ok(())
}

pub fn std_deck_omaha_hi_eval(
    hole: StdDeckCardMask,
    board: StdDeckCardMask,
    hival: &mut Option<HandVal>,
) -> Result<(), &'static str> {
    std_deck_omaha_hi_low8_eval(hole, board, hival, &mut None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck_std::StdDeck;

    #[test]
    fn test_omaha_high_hand_evaluation() {
        let hole_str = "As2dKhQh";
        let board_str = "JhTh4h3c5d";

        let (hole, _) =
            StdDeck::string_to_mask(hole_str).expect("Failed to convert hole string to mask");
        let (board, _) =
            StdDeck::string_to_mask(board_str).expect("Failed to convert board string to mask");

        let mut hival = None;
        std_deck_omaha_hi_eval(hole, board, &mut hival).expect("High hand evaluation failed");

        let expected_high_value = HandVal { value: 84650370 }; // Valeur représentant KhQhJhTh4h

        assert_eq!(
            hival,
            Some(expected_high_value),
            "High hand value did not match expected value"
        );
    }

    #[test]
    fn test_omaha_low_hand_evaluation() {
        let hole_str = "As2dKhQh";
        let board_str = "JhTh4h3c5d";

        let (hole, _) =
            StdDeck::string_to_mask(hole_str).expect("Failed to convert hole string to mask");
        let (board, _) =
            StdDeck::string_to_mask(board_str).expect("Failed to convert board string to mask");

        let mut loval = None;
        std_deck_omaha_hi_low8_eval(hole, board, &mut None, &mut loval)
            .expect("Low hand evaluation failed");

        let expected_low_value = LowHandVal { value: 274960 }; // Valeur représentant As2d4h3c5d

        assert_eq!(
            loval,
            Some(expected_low_value),
            "Low hand value did not match expected value"
        );
    }
}
