use crate::deck_joker::*;
use crate::eval_low::std_deck_lowball_eval;
use crate::handval_low::LowHandVal;
use crate::t_jokercardmasks::JokerDeckCardMask;

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

    // Convertissez `rank` en `u16` uniquement lors de l'appel des méthodes
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
