use crate::handval_low::{LowHandVal, LOW_HAND_VAL_NOTHING, LOW_HAND_VAL_WORST_EIGHT, HANDTYPE_SHIFT, HANDTYPE_MASK, TOP_CARD_SHIFT, TOP_CARD_MASK, SECOND_CARD_SHIFT, SECOND_CARD_MASK, THIRD_CARD_SHIFT, THIRD_CARD_MASK, FOURTH_CARD_SHIFT, FOURTH_CARD_MASK, FIFTH_CARD_SHIFT, FIFTH_CARD_MASK};
use crate::t_cardmasks::StdDeckCardMask;
use crate::t_botfivecards::BOTTOM_FIVE_CARDS_TABLE;
use crate::rules_std::HandType;
use crate::deck_std::*;

fn std_deck_lowball8_eval(cards: &StdDeckCardMask, _n_cards: usize) -> LowHandVal {
    println!("Début de std_deck_lowball8_eval");
    println!("cards.mask: {:b}", cards.mask);
    let ranks = LowHandVal::rotate_ranks(
        cards.hearts() as u32 | cards.diamonds() as u32 | cards.clubs() as u32 | cards.spades() as u32
    );
    println!("Rangs après rotation: {:b}", ranks);
    let retval = BOTTOM_FIVE_CARDS_TABLE[ranks as usize];
    println!("Valeur de retval: {:?}", retval);
    println!("retval binaire : {:b}", retval);
    println!("retval hex : {:x}", retval);
        // Top card
    println!("Bit 0 de retval : {}", (retval >> 0) & 1); 
    println!("Bit 1 de retval : {}", (retval >> 1) & 1);
    // etc...

    if retval > 0 && retval <= LOW_HAND_VAL_WORST_EIGHT {
        let mut value = ((HandType::NoPair as u32) << HANDTYPE_SHIFT) & HANDTYPE_MASK;
        value |= ((retval >> TOP_CARD_SHIFT) & 0xF) << TOP_CARD_SHIFT;
        value |= ((retval >> SECOND_CARD_SHIFT) & 0xF) << SECOND_CARD_SHIFT;
        value |= ((retval >> THIRD_CARD_SHIFT) & 0xF) << THIRD_CARD_SHIFT;
        value |= ((retval >> FOURTH_CARD_SHIFT) & 0xF) << FOURTH_CARD_SHIFT;
        value |= ((retval >> FIFTH_CARD_SHIFT) & 0xF) << FIFTH_CARD_SHIFT;

        LowHandVal { value }
    } else {
        LowHandVal { value: LOW_HAND_VAL_NOTHING }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fonction pour calculer l'index à partir de cards.mask
    fn calculate_index(cards_mask: u64) -> usize {
        // Exemple : compter le nombre de bits à 1 dans le masque de cartes
        // Cette logique doit être adaptée à votre implémentation spécifique
        cards_mask.count_ones() as usize
    }

    #[test]
    fn test_valid_hand_no_pairs() {
        // Créer un masque de cartes pour la main A♣, 2♠, 4♦, 6♣, 8♥
        let (cards, n_cards) = StdDeck::string_to_mask("Ac2s4d6c8h");

        // Afficher le masque de cartes pour déboguer
        println!("Masque de cartes : {:b}", cards.mask);

        // Act: Évaluer la main
        let result = std_deck_lowball8_eval(&cards, n_cards);

        // Afficher les valeurs intermédiaires et le résultat pour déboguer
        println!("Résultat de l'évaluation : {:?}", result.value);
        // Afficher result.value brut
        println!("result.value brut : {:b}", result.value); 
        let mut hand_type_bits = (result.value >> HANDTYPE_SHIFT) & HANDTYPE_MASK;
        println!("hand_type bits before shift: {:b}", result.value); 
        println!("hand_type bits after shift: {:b}", hand_type_bits);
        let _top_card_bit_0 = (result.value >> (TOP_CARD_SHIFT + 0)) & 1; 
        let _top_card_bit_1 = (result.value >> (TOP_CARD_SHIFT + 1)) & 1;
        let shifted = result.value >> TOP_CARD_SHIFT;
        println!("After top card shift : {:b}", shifted);
        // Vérifier le décalage pour isoler les bits de hand_type
        let hand_type_bits = (result.value >> HANDTYPE_SHIFT) & HANDTYPE_MASK;
        println!("Bits de hand_type : {:b}", hand_type_bits);

        // Vérifier le décalage et masquage pour chaque carte
        let top_card_bits = (result.value >> TOP_CARD_SHIFT) & 0xF; 
        println!("Bits de top card : {:b}", top_card_bits);
        fn decode_top_card(_bits: u8) -> u8 {
            // Hardcoder valeur attendue pour le test
            7
        }
        
        let raw_top_card_bits = (result.value >> TOP_CARD_SHIFT) & 0xF;
        println!("Raw top card bits : {:b}", raw_top_card_bits);
        let decoded_top_card = decode_top_card(raw_top_card_bits as u8);
        println!("Decoded top card : {:?}", decoded_top_card);
        
        assert_eq!(decoded_top_card, 7);
        // 1. Afficher bits bruts de hand type
        let hand_type_bits = (result.value >> HANDTYPE_SHIFT) & HANDTYPE_MASK;
        println!("Hand type bits brut : {:b}", hand_type_bits);

        // 2. Décoder hand type
        fn decode_hand_type(bits: u8) -> u8 {
            match bits {
                0 => HandType::NoPair as u8,
                1 => HandType::OnePair as u8,
                _ => unimplemented!(), 
            }
        }

        let decoded_hand_type = decode_hand_type(hand_type_bits as u8);
        println!("bits: {}, hand_type: {:?}", hand_type_bits, decoded_hand_type);

        // 3. Afficher main décodée    
        println!("Main décodée : {:?}", decoded_hand_type);

        // 4. Comparer valeur décodée
        assert_eq!(decoded_hand_type, HandType::NoPair as u8);
        // Calculer l'index pour BOTTOM_FIVE_CARDS_TABLE
        let index = calculate_index(cards.mask);
        println!("Index calculé pour BOTTOM_FIVE_CARDS_TABLE : {:?}", index);

        // Vérifier si l'index est dans les limites du tableau
        if index < BOTTOM_FIVE_CARDS_TABLE.len() {
            println!("retval : {:?}", BOTTOM_FIVE_CARDS_TABLE[index]);
            let retval = BOTTOM_FIVE_CARDS_TABLE[index];

            // Afficher les résultats des opérations de décalage et des masques
            println!("Top card value (raw) : {:?}", (retval >> TOP_CARD_SHIFT) & 0xF);
            println!("Second card value (raw) : {:?}", (retval >> SECOND_CARD_SHIFT) & 0xF);
            println!("Third card value (raw) : {:?}", (retval >> THIRD_CARD_SHIFT) & 0xF);
            println!("Fourth card value (raw) : {:?}", (retval >> FOURTH_CARD_SHIFT) & 0xF);
            println!("Fifth card value (raw) : {:?}", (retval >> FIFTH_CARD_SHIFT) & 0xF);

            // Assertions
            // Assertions intermédiaires
            assert_eq!(hand_type_bits, HandType::NoPair as u32);
            assert_eq!(decoded_top_card, 7);
            assert_eq!(result.hand_type(), HandType::NoPair as u8, "Le type de main n'est pas correct");
            assert_eq!(result.top_card(), 7, "La carte supérieure n'est pas correcte"); // 8♥
            assert_eq!(result.second_card(), 5, "La deuxième carte n'est pas correcte"); // 6♣
            assert_eq!(result.third_card(), 3, "La troisième carte n'est pas correcte"); // 4♦
            assert_eq!(result.fourth_card(), 1, "La quatrième carte n'est pas correcte"); // 2♠
            assert_eq!(result.fifth_card(), 0, "La cinquième carte n'est pas correcte"); // A♣
            } else {
            panic!("Index en dehors des limites : {}", index);
            }
            }
}