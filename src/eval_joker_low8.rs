use crate::eval_joker_low::joker_lowball_eval;
use crate::handval_low::{LowHandVal, LOW_HAND_VAL_WORST_EIGHT};
use crate::t_jokercardmasks::JokerDeckCardMask;

pub fn joker_lowball8_eval(cards: &JokerDeckCardMask, n_cards: usize) -> LowHandVal {
    let loval = joker_lowball_eval(cards, n_cards); // Utilisez votre fonction existante d'évaluation lowball avec joker

    // Vérifiez si la valeur de la main est meilleure ou égale à une "8-basse"
    if loval.value <= LOW_HAND_VAL_WORST_EIGHT {
        loval
    } else {
        LowHandVal { value: 0 }
    }
}
