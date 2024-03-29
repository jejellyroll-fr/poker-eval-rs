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

// pub fn std_deck_omaha_hi_low8_eval2(
//     hole: StdDeckCardMask,
//     board: StdDeckCardMask,
//     hival: &mut Option<HandVal>,
//     loval: &mut Option<LowHandVal>,
// ) -> Result<(), &'static str> {
//     let mut allcards = StdDeckCardMask::new();
//     allcards.or(&hole);
//     allcards.or(&board);

//     // Utilisez std_deck_lowball8_eval pour évaluer la main basse
//     let allval = std_deck_lowball8_eval(&allcards, STD_DECK_N_CARDS); // Ici, le nombre de cartes est STD_DECK_N_CARDS, ajustez si nécessaire
//     let eligible_for_low = allval
//         != Some(LowHandVal {
//             value: LOW_HAND_VAL_NOTHING,
//         });

//     let mut hole_cards: Vec<StdDeckCardMask> = Vec::new();
//     let mut board_cards: Vec<StdDeckCardMask> = Vec::new();

//     for i in 0..STD_DECK_N_CARDS {
//         if hole.card_is_set(i) {
//             if hole_cards.len() >= OMAHA_MAXHOLE {
//                 return Err("Too many hole cards");
//             }
//             let mut card_mask = StdDeckCardMask::new();
//             card_mask.set(i);
//             hole_cards.push(card_mask);
//         }
//         if board.card_is_set(i) {
//             if hole.card_is_set(i) {
//                 return Err("Same card in hole and board");
//             }
//             if board_cards.len() >= OMAHA_MAXBOARD {
//                 return Err("Too many board cards");
//             }
//             let mut card_mask = StdDeckCardMask::new();
//             card_mask.set(i);
//             board_cards.push(card_mask);
//         }
//     }

//     if hole_cards.len() < OMAHA_MINHOLE || hole_cards.len() > OMAHA_MAXHOLE {
//         return Err("Wrong number of hole cards");
//     }
//     if board_cards.len() < OMAHA_MINBOARD || board_cards.len() > OMAHA_MAXBOARD {
//         return Err("Wrong number of board cards");
//     }

//     // La logique pour évaluer les mains hautes et basses
//     let best_hi = HandVal { value: 0 }; // Assurez-vous que c'est la valeur appropriée pour "rien"
//                                         // Initialiser best_lo comme None pour indiquer aucune main basse qualifiée trouvée jusqu'à présent
//     let mut best_lo: Option<LowHandVal> = None;

//     // Évaluation de la main basse si éligible
//     if eligible_for_low {
//         for h1 in 0..hole_cards.len() {
//             for h2 in h1 + 1..hole_cards.len() {
//                 for b1 in 0..board_cards.len() {
//                     for b2 in (b1 + 1)..board_cards.len() {
//                         for b3 in (b2 + 1)..board_cards.len() {
//                             let mut n5 = StdDeckCardMask::new();
//                             n5.or(&hole_cards[h1]);
//                             n5.or(&hole_cards[h2]);
//                             n5.or(&board_cards[b1]);
//                             n5.or(&board_cards[b2]);
//                             n5.or(&board_cards[b3]);

//                             let cur_lo = std_deck_lowball8_eval(&n5, 5);

//                             if let Some(cur_lo_val) = cur_lo {
//                                 if best_lo.map_or(true, |best_lo_val| cur_lo_val < best_lo_val) {
//                                     best_lo = Some(cur_lo_val);
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     *hival = Some(best_hi);
//     *loval = best_lo; // Pas besoin de déballer, car loval est déjà de type Option<LowHandVal>

//     Ok(())
// }


pub fn std_deck_omaha_hi_low8_eval(
    hole: StdDeckCardMask,
    board: StdDeckCardMask,
    hival: &mut Option<HandVal>,
    loval: &mut Option<LowHandVal>,
) -> Result<(), &'static str> {
    let mut allcards = StdDeckCardMask::new();
    allcards.or(&hole);
    allcards.or(&board);

    // Conserver l'évaluation de la main basse
    let allval = std_deck_lowball8_eval(&allcards, STD_DECK_N_CARDS);
    *loval = if allval != Some(LowHandVal { value: LOW_HAND_VAL_NOTHING }) { allval } else { None };

    let mut hole_cards: Vec<StdDeckCardMask> = Vec::new();
    let mut board_cards: Vec<StdDeckCardMask> = Vec::new();

    // Extraire les cartes de poche et du tableau individuellement
    for i in 0..STD_DECK_N_CARDS {
        if hole.card_is_set(i) {
            let mut card_mask = StdDeckCardMask::new();
            card_mask.set(i);
            hole_cards.push(card_mask);
        }
        if board.card_is_set(i) {
            let mut card_mask = StdDeckCardMask::new();
            card_mask.set(i);
            board_cards.push(card_mask);
        }
    }

    // Initialiser la meilleure main haute
    let mut best_hi: Option<HandVal> = None;

    // Itérer sur toutes les combinaisons possibles de 2 cartes de poche et 3 cartes du tableau pour la main haute
    for i in 0..hole_cards.len() {
        for j in i + 1..hole_cards.len() {
            for k in 0..board_cards.len() {
                for l in k + 1..board_cards.len() {
                    for m in l + 1..board_cards.len() {
                        let mut potential_hand = StdDeckCardMask::new();
                        potential_hand.or(&hole_cards[i]);
                        potential_hand.or(&hole_cards[j]);
                        potential_hand.or(&board_cards[k]);
                        potential_hand.or(&board_cards[l]);
                        potential_hand.or(&board_cards[m]);

                        // Évaluer la main potentielle pour la partie haute
                        let potential_hi_val = Eval::eval_n(&potential_hand, 5);
                        if let Some(current_best_hi) = best_hi {
                            if potential_hi_val > current_best_hi {
                                best_hi = Some(potential_hi_val);
                            }
                        } else {
                            best_hi = Some(potential_hi_val);
                        }
                    }
                }
            }
        }
    }

    *hival = best_hi;

    // Continuer l'évaluation de la main basse comme avant
    if *loval != Some(LowHandVal { value: LOW_HAND_VAL_NOTHING }) {
        // Itérer sur toutes les combinaisons possibles de 2 cartes de poche et 3 cartes du tableau pour la main basse
        for h1 in 0..hole_cards.len() {
            for h2 in h1 + 1..hole_cards.len() {
                for b1 in 0..board_cards.len() {
                    for b2 in b1 + 1..board_cards.len() {
                        for b3 in b2 + 1..board_cards.len() {
                            let mut potential_hand = StdDeckCardMask::new();
                            potential_hand.or(&hole_cards[h1]);
                            potential_hand.or(&hole_cards[h2]);
                            potential_hand.or(&board_cards[b1]);
                            potential_hand.or(&board_cards[b2]);
                            potential_hand.or(&board_cards[b3]);

                            // Évaluer la main potentielle pour la partie basse
                            let cur_lo = std_deck_lowball8_eval(&potential_hand, 5);
                            if let Some(cur_lo_val) = cur_lo {
                                if let Some(best_lo_val) = *loval {
                                    if cur_lo_val < best_lo_val {
                                        *loval = Some(cur_lo_val);
                                    }
                                } else {
                                    *loval = Some(cur_lo_val);
                                }
                            }
                        }
                    }
                }
            }
        }
    }


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
