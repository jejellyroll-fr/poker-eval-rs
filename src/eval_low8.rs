use crate::handval_low::{LowHandVal, LOW_HAND_VAL_NOTHING, LOW_HAND_VAL_WORST_EIGHT, HANDTYPE_SHIFT, HANDTYPE_MASK, TOP_CARD_SHIFT,  SECOND_CARD_SHIFT,  THIRD_CARD_SHIFT, FOURTH_CARD_SHIFT, FIFTH_CARD_SHIFT};
use crate::t_cardmasks::StdDeckCardMask;
use crate::t_botfivecards::BOTTOM_FIVE_CARDS_TABLE;
use crate::rules_std::HandType;


pub fn std_deck_lowball8_eval(cards: &StdDeckCardMask, _n_cards: usize) -> LowHandVal {
    //println!("Début de std_deck_lowball8_eval");
    //println!("cards.mask: {:b}", cards.mask);
    //println!("Longueur du masque avant rotation: {}", cards.mask.count_ones());
    let ss = LowHandVal::rotate_ranks(cards.spades().into());
    let sc = LowHandVal::rotate_ranks(cards.clubs().into());
    let sd = LowHandVal::rotate_ranks(cards.diamonds().into());
    let sh = LowHandVal::rotate_ranks(cards.hearts().into());

    let ranks = sc | ss | sd | sh;
    //println!("Rangs après rotation: {:b}", ranks);
    //println!("Longueur du masque après rotation: {}", ranks.count_ones());

    let retval = BOTTOM_FIVE_CARDS_TABLE[ranks as usize];
    //println!("Valeur de retval: {:?}", retval);
    //println!("retval binaire : {:b}", retval);
    //println!("retval hex : {:x}", retval);

    // Affichage des bits pour chaque carte
    //println!("Bit 0 de retval : {}", (retval >> 0) & 1);
    //println!("Bit 1 de retval : {}", (retval >> 1) & 1);
    //println!("Bit 2 de retval : {}", (retval >> 2) & 1);
    //println!("Bit 3 de retval : {}", (retval >> 3) & 1);
    //println!("Bit 4 de retval : {}", (retval >> 4) & 1);

    if retval > 0 && retval <= LOW_HAND_VAL_WORST_EIGHT {
        let mut value = ((HandType::NoPair as u32) << HANDTYPE_SHIFT) & HANDTYPE_MASK;
        value |= ((retval >> TOP_CARD_SHIFT) & 0xF) << TOP_CARD_SHIFT;
        value |= ((retval >> SECOND_CARD_SHIFT) & 0xF) << SECOND_CARD_SHIFT;
        value |= ((retval >> THIRD_CARD_SHIFT) & 0xF) << THIRD_CARD_SHIFT;
        value |= ((retval >> FOURTH_CARD_SHIFT) & 0xF) << FOURTH_CARD_SHIFT;
        value |= ((retval >> FIFTH_CARD_SHIFT) & 0xF) << FIFTH_CARD_SHIFT;

        // Afficher les valeurs des cartes extraites
        //println!("Valeur de la carte supérieure : {}", (value >> TOP_CARD_SHIFT) & 0xF);
        //println!("Valeur de la deuxième carte : {}", (value >> SECOND_CARD_SHIFT) & 0xF);
        //println!("Valeur de la troisième carte : {}", (value >> THIRD_CARD_SHIFT) & 0xF);
        //println!("Valeur de la quatrième carte : {}", (value >> FOURTH_CARD_SHIFT) & 0xF);
        //println!("Valeur de la cinquième carte : {}", (value >> FIFTH_CARD_SHIFT) & 0xF);

        LowHandVal { value }
    } else {
        LowHandVal { value: LOW_HAND_VAL_NOTHING }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_lowball_hand() {
        let result_mask = StdDeck::string_to_mask("Ac2s4d6c8h");
        let (cards, n_cards) = match result_mask {
            Ok((mask, num_cards)) => (mask, num_cards),
            Err(e) => panic!("Erreur lors de la conversion de la chaîne en masque de cartes : {}", e),
        };
        let result = std_deck_lowball8_eval(&cards, n_cards);
        assert_eq!(result.hand_type(), HandType::NoPair as u8);
        assert_eq!(result.top_card(), 7, "La carte supérieure n'est pas correcte");
        assert_eq!(result.second_card(), 5, "La deuxième carte n'est pas correcte");
        assert_eq!(result.third_card(), 3, "La troisième carte n'est pas correcte");
        assert_eq!(result.fourth_card(), 1, "La quatrième carte n'est pas correcte");
        assert_eq!(result.fifth_card(), 0, "La cinquième carte n'est pas correcte");
    }

    #[test]
    fn test_valid_lowball_hand2() {
        let result_mask = StdDeck::string_to_mask("Ac2s3d4c5h");
        let (cards, n_cards) = match result_mask {
            Ok((mask, num_cards)) => (mask, num_cards),
            Err(e) => panic!("Erreur lors de la conversion de la chaîne en masque de cartes : {}", e),
        };
        let result = std_deck_lowball8_eval(&cards, n_cards);
        //println!("result: {:?}", result);
        assert_eq!(result.hand_type(), HandType::NoPair as u8);
        assert_eq!(result.top_card(), 4, "La carte supérieure n'est pas correcte");
        assert_eq!(result.second_card(), 3, "La deuxième carte n'est pas correcte");
        assert_eq!(result.third_card(), 2, "La troisième carte n'est pas correcte");
        assert_eq!(result.fourth_card(), 1, "La quatrième carte n'est pas correcte");
        assert_eq!(result.fifth_card(), 0, "La cinquième carte n'est pas correcte");
    }

    #[test]
    fn test_invalid_lowball_hand_with_pair() {
        let result_mask = StdDeck::string_to_mask("4c4d6c8hJs");
        let (cards, n_cards) = match result_mask {
            Ok((mask, num_cards)) => (mask, num_cards),
            Err(e) => panic!("Erreur lors de la conversion de la chaîne en masque de cartes : {}", e),
        };
        let result = std_deck_lowball8_eval(&cards, n_cards);
        assert_eq!(result, LowHandVal { value: LOW_HAND_VAL_NOTHING });
    }
}
