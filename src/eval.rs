use crate::handval::{HandVal} ;
use crate::t_cardmasks::StdDeckCardMask;
use crate::deck_std::*;
use crate::rules_std::HandType;
use crate::t_nbits::NBITS_TABLE;
use crate::t_straight::STRAIGHT_TABLE;
use crate::t_topcard::TOP_CARD_TABLE;


pub struct Eval;

fn count_bits(mut n: u32) -> u32 {
    let mut count = 0;
    while n != 0 {
        count += (n & 1) as u32;
        n >>= 1;
    }
    count
}


impl Eval {
    fn extract_top_five_cards(suit_mask: u16) -> (u8, u8, u8, u8, u8) {
        let mut cards = Vec::new();
        let mask = suit_mask;
    
        // Parcourir les bits du masque pour trouver les cartes
        for i in (0..13).rev() {
            if mask & (1 << i) != 0 {
                cards.push(i as u8);
                if cards.len() == 5 {
                    break;
                }
            }
        }
    
        // Assurez-vous que la liste contient exactement 5 éléments
        while cards.len() < 5 {
            cards.push(0);
        }
    
        // Renvoie les valeurs des cinq cartes les plus hautes
        (cards[0], cards[1], cards[2], cards[3], cards[4])
    }
    

    pub fn eval_n(cards: &StdDeckCardMask, n_cards: usize) -> HandVal {
        let ss = cards.spades();
        //println!("Spades: {:b}", ss);
        let sc = cards.clubs();
        //println!("Clubs: {:b}", sc);
        let sd = cards.diamonds();
        //println!("Diamonds: {:b}", sd);
        let sh = cards.hearts();
        //println!("Hearts: {:b}", sh);
        let ranks = ss | sc | sd | sh;
        //println!("Combined ranks: {:b}", ranks);
        let n_ranks = NBITS_TABLE[ranks as usize];
        // Utiliser la fonction count_bits
        //let n_ranks = count_bits(ranks);
        let n_dups = n_cards - n_ranks as usize;
        let hand_val: Option<HandVal> = None;
        //println!("main: Rangs combinés: {:b}, Nombre de rangs: {}", ranks, n_ranks);
        //println!("main: Nombre de duplicatas 1: {}", n_dups);
        //println!("main: Spades: {:b}, Clubs: {:b}, Diamonds: {:b}, Hearts: {:b}", ss, sc, sd, sh);
        if n_ranks >= 5 {
            // Vérifier les flushes et les straight flushes
            for suit in [ss, sc, sd, sh].iter() {
                if NBITS_TABLE[*suit as usize] >= 5 {
                    // Extraire les cinq cartes les plus hautes
                    let (top, second, third, fourth, fifth) = Self::extract_top_five_cards(*suit);

                    // Vérifier si les cartes forment une suite
                    if STRAIGHT_TABLE[(*suit) as usize] != 0 {
                        let hand_val = HandVal::new(HandType::StFlush as u8, top, second, third, fourth, fifth);
                        //println!("Retourne HandVal pour Straight Flush: {:?}", hand_val);
                        return hand_val;
                    } else {
                        let hand_val = HandVal::new(HandType::Flush as u8, top, second, third, fourth, fifth);
                        //println!("Retourne HandVal pour Flush: {:?}", hand_val);
                        return hand_val;
                    }

                }

            }
            let st = STRAIGHT_TABLE[ranks as usize];
            if st != 0 {
                // Suite trouvée
                let top_card = st;
                let second_card = if top_card > 0 { top_card - 1 } else { 0 };
                let third_card = if top_card > 1 { top_card - 2 } else { 0 };
                let fourth_card = if top_card > 2 { top_card - 3 } else { 0 };
                let fifth_card = if top_card > 3 { top_card - 4 } else { 0 };
                
                let hand_val = HandVal::new(HandType::Straight as u8, top_card, second_card, third_card, fourth_card, fifth_card);
                //println!("Retourne HandVal pour Straight: {:?}", hand_val);
                return hand_val;
            }
            // Vérifier si hand_val est défini et si n_dups < 3
            if let Some(val) = hand_val {
                if n_dups < 3 {
                    return val;
                }
            }
        }

            

    

        //println!("main: Nombre de duplicatas 1: {}", n_dups);
        match n_dups {
            0 => {
                // C'est une main sans paire
                let (top, second, third, fourth, fifth) = Self::extract_top_five_cards(ranks);
                
                let hand_val = HandVal::new(
                    HandType::NoPair as u8,
                    top, second, third, fourth, fifth
                );
                //println!("Retourne HandVal pour NoPair: {:?}", hand_val);
                return hand_val;
            },
            1 => {
                // C'est une main avec une paire
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                let pair_card = TOP_CARD_TABLE[two_mask as usize];
                let kickers_mask = ranks ^ two_mask; // Supprime les cartes de la paire
                let (kicker1, kicker2, kicker3, _, _) = Self::extract_top_five_cards(kickers_mask);
            
                let hand_val = HandVal::new(
                    HandType::OnePair as u8,
                    pair_card,
                    kicker1, kicker2, kicker3, 0
                );
                //println!("Retourne HandVal pour OnePair: {:?}", hand_val);
                return hand_val;
            },
            
            2 => {
                // Soit deux paires, soit un brelan
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                if two_mask != 0 {
                    // Deux paires
                    let pair1_mask = two_mask & (-(two_mask as i16)) as u16; // Masque pour la première paire
                    let pair2_mask = two_mask & (!(pair1_mask) & two_mask); // Masque pour la deuxième paire
                    let kickers_mask = ranks ^ two_mask; // Masque pour les kickers
                    let pair1_top_card = TOP_CARD_TABLE[pair1_mask as usize];
                    let pair2_top_card = TOP_CARD_TABLE[pair2_mask as usize];
                    let kicker = TOP_CARD_TABLE[kickers_mask as usize];
            
                    let hand_val = HandVal::new(
                        HandType::TwoPair as u8,
                        pair1_top_card.max(pair2_top_card), // Plus haute paire
                        pair1_top_card.min(pair2_top_card), // Plus basse paire
                        kicker, 0, 0
                    );
                    //println!("Retourne HandVal pour TwoPair: {:?}", hand_val);
                    return hand_val;
                } else {
                    // Un brelan
                    let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                    let brelan_card = TOP_CARD_TABLE[three_mask as usize];
                    let kickers_mask = ranks ^ three_mask; // Supprime les cartes du brelan
                    let (kicker1, kicker2, _, _, _) = Self::extract_top_five_cards(kickers_mask);

                    let hand_val = HandVal::new(
                        HandType::Trips as u8,
                        brelan_card,
                        kicker1,
                        kicker2,
                        0, 0
                    );
                    //println!("Retourne HandVal pour Trips: {:?}", hand_val);
                    return hand_val;
                }
            },
            _ => {
                // Carré (Quads)
                let four_mask = sh & sd & sc & ss;
                if four_mask != 0 {
                    let tc = TOP_CARD_TABLE[four_mask as usize];
                    let hand_val = HandVal::new(
                        HandType::Quads as u8,
                        tc,
                        TOP_CARD_TABLE[(ranks ^ (1 << tc)) as usize],
                        0, 0, 0
                    );
                    //println!("Retourne HandVal pour Quads : {:?}", hand_val);
                    return hand_val;
                }
            
                // Full House 
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                if usize::from(NBITS_TABLE[two_mask as usize]) != n_dups {
                    let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                    let tc = TOP_CARD_TABLE[three_mask as usize];
                    let t = (two_mask | three_mask) ^ (1 << tc);
                    let hand_val = HandVal::new(
                        HandType::FullHouse as u8,
                        tc,
                        TOP_CARD_TABLE[t as usize],
                        0, 0, 0
                    );
                    //println!("Retourne HandVal pourFullHouse: {:?}", hand_val);
                    return hand_val;
                }
            
                // Deux Paires
                let top = TOP_CARD_TABLE[two_mask as usize];
                let second = TOP_CARD_TABLE[(two_mask ^ (1 << top)) as usize];
                let hand_val = HandVal::new(
                    HandType::TwoPair as u8,
                    top,
                    second,
                    TOP_CARD_TABLE[(ranks ^ (1 << top) ^ (1 << second)) as usize],
                    0, 0
                );
                //println!("Retourne HandVal pour TwoPair: {:?}", hand_val);
                return hand_val;
            }
        }
    }

}
