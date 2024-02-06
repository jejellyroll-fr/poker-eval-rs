use crate::handval_low::{
    LowHandVal, FIFTH_CARD_SHIFT, FOURTH_CARD_SHIFT, HANDTYPE_MASK, HANDTYPE_SHIFT,
    LOW_HAND_VAL_NOTHING, LOW_HAND_VAL_WORST_EIGHT, SECOND_CARD_SHIFT, THIRD_CARD_SHIFT,
    TOP_CARD_SHIFT,
};
use crate::rules_std::HandType;
use crate::t_botfivecards::BOTTOM_FIVE_CARDS_TABLE;
use crate::t_cardmasks::StdDeckCardMask;

pub fn std_deck_lowball8_eval(cards: &StdDeckCardMask, _n_cards: usize) -> Option<LowHandVal> {
    let ss = LowHandVal::rotate_ranks(cards.spades().into());
    let sc = LowHandVal::rotate_ranks(cards.clubs().into());
    let sd = LowHandVal::rotate_ranks(cards.diamonds().into());
    let sh = LowHandVal::rotate_ranks(cards.hearts().into());

    let ranks = sc | ss | sd | sh;

    let retval = BOTTOM_FIVE_CARDS_TABLE[ranks as usize];

    if retval > 0 && retval <= LOW_HAND_VAL_WORST_EIGHT {
        let mut value = ((HandType::NoPair as u32) << HANDTYPE_SHIFT) & HANDTYPE_MASK;
        value |= ((retval >> TOP_CARD_SHIFT) & 0xF) << TOP_CARD_SHIFT;
        value |= ((retval >> SECOND_CARD_SHIFT) & 0xF) << SECOND_CARD_SHIFT;
        value |= ((retval >> THIRD_CARD_SHIFT) & 0xF) << THIRD_CARD_SHIFT;
        value |= ((retval >> FOURTH_CARD_SHIFT) & 0xF) << FOURTH_CARD_SHIFT;
        value |= ((retval >> FIFTH_CARD_SHIFT) & 0xF) << FIFTH_CARD_SHIFT;

        Some(LowHandVal { value })
    } else {
        Some(LowHandVal {
            value: LOW_HAND_VAL_NOTHING,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck_std::StdDeck;

    #[test]
    fn test_valid_lowball_hand() {
        let result_mask = StdDeck::string_to_mask("Ac2s4d6c8h");
        let (cards, n_cards) = match result_mask {
            Ok((mask, num_cards)) => (mask, num_cards),
            Err(e) => panic!(
                "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
                e
            ),
        };
        let result = std_deck_lowball8_eval(&cards, n_cards);
        assert_eq!(result.hand_type(), HandType::NoPair as u8);
        assert_eq!(
            result.top_card(),
            7,
            "La carte supérieure n'est pas correcte"
        );
        assert_eq!(
            result.second_card(),
            5,
            "La deuxième carte n'est pas correcte"
        );
        assert_eq!(
            result.third_card(),
            3,
            "La troisième carte n'est pas correcte"
        );
        assert_eq!(
            result.fourth_card(),
            1,
            "La quatrième carte n'est pas correcte"
        );
        assert_eq!(
            result.fifth_card(),
            0,
            "La cinquième carte n'est pas correcte"
        );
    }

    #[test]
    fn test_valid_lowball_hand2() {
        let result_mask = StdDeck::string_to_mask("Ac2s3d4c5h");
        let (cards, n_cards) = match result_mask {
            Ok((mask, num_cards)) => (mask, num_cards),
            Err(e) => panic!(
                "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
                e
            ),
        };
        let result = std_deck_lowball8_eval(&cards, n_cards);
        //println!("result: {:?}", result);
        assert_eq!(result.hand_type(), HandType::NoPair as u8);
        assert_eq!(
            result.top_card(),
            4,
            "La carte supérieure n'est pas correcte"
        );
        assert_eq!(
            result.second_card(),
            3,
            "La deuxième carte n'est pas correcte"
        );
        assert_eq!(
            result.third_card(),
            2,
            "La troisième carte n'est pas correcte"
        );
        assert_eq!(
            result.fourth_card(),
            1,
            "La quatrième carte n'est pas correcte"
        );
        assert_eq!(
            result.fifth_card(),
            0,
            "La cinquième carte n'est pas correcte"
        );
    }

    #[test]
    fn test_invalid_lowball_hand_with_pair() {
        let result_mask = StdDeck::string_to_mask("4c4d6c8hJs");
        let (cards, n_cards) = match result_mask {
            Ok((mask, num_cards)) => (mask, num_cards),
            Err(e) => panic!(
                "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
                e
            ),
        };
        let result = std_deck_lowball8_eval(&cards, n_cards);
        assert_eq!(
            result,
            LowHandVal {
                value: LOW_HAND_VAL_NOTHING
            }
        );
    }

    #[test]
    fn test_invalid_lowball_hand_with_high_jack() {
        let result_mask = StdDeck::string_to_mask("4c3d6c8hJs");
        let (cards, n_cards) = match result_mask {
            Ok((mask, num_cards)) => (mask, num_cards),
            Err(e) => panic!(
                "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
                e
            ),
        };
        let result = std_deck_lowball8_eval(&cards, n_cards);
        assert_eq!(
            result,
            LowHandVal {
                value: LOW_HAND_VAL_NOTHING
            }
        );
    }
}
