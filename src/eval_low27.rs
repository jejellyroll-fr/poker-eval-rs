use crate::t_cardmasks::StdDeckCardMask;
use crate::t_nbits::NBITS_TABLE;
use crate::t_straight::STRAIGHT_TABLE;
use crate::rules_std::HandType;
use crate::eval_low::extract_top_five_cards_lowball;
use crate::t_topcard::TOP_CARD_TABLE;
use crate::eval::count_bits;
use crate::deck_std::*;
use crate::t_topfivecards::TOP_FIVE_CARDS_TABLE;
use crate::handval::{HandVal, CARD_MASK, HANDTYPE_SHIFT, CARD_WIDTH, FIFTH_CARD_MASK, SECOND_CARD_SHIFT, SECOND_CARD_MASK, THIRD_CARD_SHIFT, THIRD_CARD_MASK, FOURTH_CARD_SHIFT, FOURTH_CARD_MASK, TOP_CARD_SHIFT, TOP_CARD_MASK};

pub fn std_deck_lowball27_eval(cards: &StdDeckCardMask, _n_cards: usize) -> HandVal {
    let ss = cards.spades().into(); // Rangs pour Spades
    let sc = cards.clubs().into(); // Rangs pour Clubs
    let sd = cards.diamonds().into(); // Rangs pour Diamonds
    let sh = cards.hearts().into(); // Rangs pour Hearts

    let ranks = sc | ss | sd | sh;
    let n_ranks = NBITS_TABLE[ranks as usize];
    let n_dups = _n_cards - n_ranks as usize;
    let retval: Option<HandVal> = None;

    //println!("Rangs combinés: {:b}, Nombre de rangs: {}", ranks, n_ranks);
    //println!("Nombre de duplicatas 1: {}", n_dups);
    //println!("Spades: {:b}, Clubs: {:b}, Diamonds: {:b}, Hearts: {:b}", ss, sc, sd, sh);

    if n_ranks >= 5 {
        for suit in [ss, sc, sd, sh].iter() {
            println!("Vérification de la couleur : {:b}", suit);
            if NBITS_TABLE[*suit as usize] >= 5 {
                let (top, second, third, fourth, fifth) = extract_top_five_cards_lowball(*suit);
                if STRAIGHT_TABLE[*suit as usize] != 0 {
                    println!("Straight Flush détecté dans la couleur: {:b}", suit);
                    let retval = HandVal::new(
                        HandType::StFlush as u8,
                        top,
                        second,
                        third,
                        fourth,
                        fifth,
                    );
                    return retval;
                } else {
                    println!("Flush détecté dans la couleur: {:b}", suit);
                    let retval = HandVal::new(
                        HandType::Flush as u8,
                        top,
                        second,
                        third,
                        fourth,
                        fifth,
                    );
                    return retval;
                }
            }
        }
        let st = STRAIGHT_TABLE[ranks as usize];
        if st != 0 {
            println!("Vérification de la suite : {:b}", st);
            // Suite trouvée
            let top_card = st;
            println!("top_card: {}", top_card);
            let second_card = if top_card > 0 { top_card - 1 } else { 0 };
            println!("second_card: {}", second_card);
            let third_card = if top_card > 1 { top_card - 2 } else { 0 };
            println!("third_card: {}", third_card);
            let fourth_card = if top_card > 2 { top_card - 3 } else { 0 };
            println!("fourth_card: {}", fourth_card);
            let fifth_card = if top_card > 3 { top_card - 4 } else { 0 };
            println!("fifth_card: {}", fifth_card);

            let retval = HandVal::new(
                HandType::Straight as u8,
                top_card,
                second_card,
                third_card,
                fourth_card,
                fifth_card,
            );
            println!("Retourne retVal pour Straight: {:?}", retval);
            return retval;
            }
            // Vérifier si hand_val est défini et si n_dups < 3
            if let Some(val) = retval {
                if n_dups < 3 {
                    println!("Retourne HandVal pour Flush: {:?}", val);
                    return val;
                }
            }
    }


    match n_dups {
        0 => {
            // C'est une main sans paire
            let retval = HandVal {
                value: ((HandType::NoPair as u32) ) + TOP_FIVE_CARDS_TABLE[ranks as usize],
            };
            //println!("Retourne HandVal pour NoPair: {:?}", retval);
            return retval;
        },
        1 => {
            // C'est une main avec une paire
            //println!("c'est une paire");
            let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
            //println!("two_mask: {}", two_mask);
            let pair_card = TOP_CARD_TABLE[two_mask as usize];
            //println!("pair_card: {}", pair_card);
            let kickers_mask = ranks ^ two_mask;
            //println!("kickers_mask: {}", kickers_mask);
            //let (kicker1, kicker2, kicker3, _, _) = extract_top_five_cards_lowball(kickers_mask);
            // Obtenir les kickers en utilisant la table topFiveCardsTable
            let kickers = TOP_FIVE_CARDS_TABLE[kickers_mask as usize] >> CARD_WIDTH
                            & !FIFTH_CARD_MASK;

            let kicker1_mask = CARD_MASK << SECOND_CARD_SHIFT;
            let kicker2_mask = CARD_MASK << THIRD_CARD_SHIFT;
            let kicker3_mask = CARD_MASK << FOURTH_CARD_SHIFT;
                            
            let kicker1 = (kickers & kicker1_mask) >> SECOND_CARD_SHIFT;
            let kicker2 = (kickers & kicker2_mask) >> THIRD_CARD_SHIFT;
            let kicker3 = (kickers & kicker3_mask) >> FOURTH_CARD_SHIFT;
                            
            //println!("Kickers: Kicker1: {}, Kicker2: {}, Kicker3: {}", kicker1, kicker2, kicker3);
            let retval = HandVal::new(
                HandType::OnePair as u8,
                pair_card,
                kicker1 as u8,
                kicker2 as u8,
                kicker3 as u8,
                0,
            );
            //println!("Retourne HandVal pour OnePair: {:?}", retval);
            return retval;
        },
        2 => {
            //println!("c'est deux paires ou brelan");
            let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
            //println!("two_mask: {}", two_mask);
            let t = ranks ^ two_mask;
            //println!("t: {}", t);
        
            if two_mask == 0 {
                // C'est un brelan
                //println!("C'est un brelan");
                let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                //println!("three_mask: {}", three_mask);
                let trips_card = TOP_CARD_TABLE[three_mask as usize];
                //println!("trips_card: {}", trips_card);
                let kickers_mask = ranks ^ three_mask;
                //println!("kickers_mask: {}", kickers_mask);
                let (kicker1, kicker2, _, _, _) = extract_top_five_cards_lowball(kickers_mask);
                //println!("kicker1: {}", kicker1);
                //println!("kicker2: {}", kicker2);
                let retval = HandVal::new(
                    HandType::Trips as u8,
                    trips_card,
                    kicker1,
                    kicker2,
                    0,
                    0,
                );
                //println!("Retourne HandVal pour Trips: {:?}", retval);
                return retval;
            } else {
                // C'est deux paires
                //println!("C'est deux paires");
                let pair1 = TOP_CARD_TABLE[two_mask as usize];
                //println!("pair1: {}", pair1);
                let pair2 = TOP_CARD_TABLE[(two_mask ^ (1 << pair1)) as usize];
                //println!("pair2: {}", pair2);
                let kicker = TOP_CARD_TABLE[(ranks ^ (1 << pair1) ^ (1 << pair2)) as usize];
                //println!("kicker: {}", kicker);
                let retval = HandVal::new(
                    HandType::TwoPair as u8,
                    pair1.max(pair2),
                    pair1.min(pair2),
                    kicker,
                    0,
                    0,
                );
                //println!("Retourne HandVal pour TwoPair: {:?}", retval);
                return retval;
            }
        },
        
        
        _ => {
            // Possible quads, fullhouse, straight or flush, or two pair
            let four_mask = sh & sd & sc & ss;
            if four_mask != 0 {
                // C'est un carré (Quads)
                let quads_card = TOP_CARD_TABLE[four_mask as usize];
                let kicker = TOP_CARD_TABLE[(ranks ^ (1 << quads_card)) as usize];
                let retval = HandVal::new(
                    HandType::Quads as u8,
                    quads_card,
                    kicker,
                    0,
                    0,
                    0,
                );
                //println!("Retourne HandVal pour Quads: {:?}", retval);
                return retval;
            } else {
                // Vérifier full house ou deux paires
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                if three_mask != 0 {
                    // C'est un full house
                    let fullhouse_card = TOP_CARD_TABLE[three_mask as usize];
                    let pair_card = TOP_CARD_TABLE[(two_mask ^ (1 << fullhouse_card)) as usize];
                    let retval = HandVal::new(
                        HandType::FullHouse as u8,
                        fullhouse_card,
                        pair_card,
                        0,
                        0,
                        0,
                    );
                    //println!("Retourne HandVal pour FullHouse: {:?}", retval);
                    return retval;
                } else {
                    // Deux paires
                    let pair1 = TOP_CARD_TABLE[two_mask as usize];
                    let pair2 = TOP_CARD_TABLE[(two_mask ^ (1 << pair1)) as usize];
                    let kicker = TOP_CARD_TABLE[(ranks ^ (1 << pair1) ^ (1 << pair2)) as usize];
                    let retval = HandVal::new(
                        HandType::TwoPair as u8,
                        pair1.max(pair2),
                        pair1.min(pair2),
                        kicker,
                        0,
                        0,
                    );
                    //println!("Retourne HandVal pour TwoPair: {:?}", retval);
                    return retval;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck_std::{StdDeck, STD_DECK_RANK_2, STD_DECK_RANK_3, STD_DECK_RANK_4, STD_DECK_RANK_5, STD_DECK_RANK_6, STD_DECK_RANK_7, STD_DECK_RANK_8, STD_DECK_RANK_TEN};
    use crate::deck_std::{STD_DECK_SUIT_SPADES, STD_DECK_SUIT_DIAMONDS, STD_DECK_SUIT_CLUBS, STD_DECK_SUIT_HEARTS};

    fn create_mask_from_string(cards_str: &str) -> StdDeckCardMask {
        let (mask, _) = StdDeck::string_to_mask(cards_str).expect("Failed to create mask");
        mask
    }
    fn calculate_rank_mask(cards: &StdDeckCardMask) -> u32 {
        let ranks = [
            cards.spades(),
            cards.clubs(),
            cards.diamonds(),
            cards.hearts(),
        ];
    
        ranks.iter().fold(0, |acc, &suit_ranks| acc | suit_ranks as u32)
    }


    #[test]
    fn test_no_pair() {
        let mask = create_mask_from_string("2s3d4c7h8s");
        let rank_mask = calculate_rank_mask(&mask);
        let result = std_deck_lowball27_eval(&mask, 5);
        let expected_value = TOP_FIVE_CARDS_TABLE[rank_mask as usize];
        assert_eq!(result.value, expected_value, "Failed to evaluate no pair hand");
    }


    #[test]
    fn test_no_pair_withace() {
        let mask = create_mask_from_string("As3d4c7h8s");
        let rank_mask = calculate_rank_mask(&mask);
        let result = std_deck_lowball27_eval(&mask, 5);
        let expected_value = TOP_FIVE_CARDS_TABLE[rank_mask as usize];
        assert_eq!(result.value, expected_value, "Failed to evaluate no pair hand");
    }

    #[test]
    fn test_one_pair() {
        let mask = create_mask_from_string("2s2d4c7h8s");
        let result = std_deck_lowball27_eval(&mask, 5);
        
        // Extract hand type from the result
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(hand_type, HandType::OnePair as u32, "Failed to detect one pair");
    }
    
    
    #[test]
    fn test_two_pair() {
        let mask = create_mask_from_string("2s2d3c3h8s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(hand_type, HandType::TwoPair as u32, "Failed to detect two pair");
    }
    
    #[test]
    fn test_three_of_a_kind() {
        let mask = create_mask_from_string("2s2d2c7h8s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        println!("hand_type: {}", hand_type);
        assert_eq!(hand_type, HandType::Trips as u32, "Failed to detect three of a kind");
    }
    
    #[test]
    fn test_straight() {
        let mask = create_mask_from_string("2s3d4c5h6s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(hand_type, HandType::Straight as u32, "Failed to detect straight");
    }
    
    #[test]
    fn test_flush() {
        let mask = create_mask_from_string("2s4s6s8sTs");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(hand_type, HandType::Flush as u32, "Failed to detect flush");
    }
    
    #[test]
    fn test_full_house() {
        let mask = create_mask_from_string("2s2d2c3h3s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(hand_type, HandType::FullHouse as u32, "Failed to detect full house");
    }
    
    #[test]
    fn test_four_of_a_kind() {
        let mask = create_mask_from_string("2s2d2c2h8s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(hand_type, HandType::Quads as u32, "Failed to detect four of a kind");
    }
    
    #[test]
    fn test_straight_flush() {
        let mask = create_mask_from_string("2s3s4s5s6s");
        let result = std_deck_lowball27_eval(&mask, 5);
        let hand_type = result.value >> HANDTYPE_SHIFT;
        assert_eq!(hand_type, HandType::StFlush as u32, "Failed to detect straight flush");
    }
}    



