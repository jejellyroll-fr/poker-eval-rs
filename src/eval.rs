use crate::handval::{HandVal, CARD_WIDTH, FIFTH_CARD_MASK} ;
use crate::t_cardmasks::StdDeckCardMask;
use crate::deck_std::*;
use crate::rules_std::HandType;
use crate::t_nbits::NBITS_TABLE;
use crate::t_straight::STRAIGHT_TABLE;
use crate::t_topfivecards::TOP_FIVE_CARDS_TABLE;
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
    pub fn eval_n(cards: &StdDeckCardMask, n_cards: usize) -> HandVal {
        let ss = cards.spades();
        println!("Spades: {:b}", ss);
        let sc = cards.clubs();
        println!("Clubs: {:b}", sc);
        let sd = cards.diamonds();
        println!("Diamonds: {:b}", sd);
        let sh = cards.hearts();
        println!("Hearts: {:b}", sh);
        let ranks = ss | sc | sd | sh;
        println!("Combined ranks: {:b}", ranks);
        let n_ranks = NBITS_TABLE[ranks as usize];
        // Utiliser la fonction count_bits
        //let n_ranks = count_bits(ranks);
        let n_dups = n_cards - n_ranks as usize;

        println!("main: Rangs combinés: {:b}, Nombre de rangs: {}", ranks, n_ranks);
        println!("main: Nombre de duplicatas: {}", n_dups);
        println!("main: Spades: {:b}, Clubs: {:b}, Diamonds: {:b}, Hearts: {:b}", ss, sc, sd, sh);
        if n_ranks >= 5 {
            // Vérifier les flushes et les straight flushes
            for suit in [ss, sc, sd, sh].iter() {
                println!("Évaluation pour le suit: {:b}, flush: {}", *suit, NBITS_TABLE[*suit as usize] >= 5);
                if NBITS_TABLE[*suit as usize] >= 5 {
                    if let Some(top_card) = STRAIGHT_TABLE.get(*suit as usize) {
                        let hand_val = HandVal::new(HandType::Straight as u8, *top_card, 0, 0, 0, 0);
                        println!("Retourne HandVal pour Straight: {:?}", hand_val);
                        return hand_val;
                    } else {
                        let hand_val = HandVal::new(
                            HandType::Flush as u8,
                            TOP_FIVE_CARDS_TABLE[*suit as usize].try_into().unwrap(),
                            0, 0, 0, 0
                        );    
                        println!("Retourne HandVal pour Flush: {:?}", hand_val);
                        return hand_val;
                    }

                }
            }
    
            if let Some(top_card) = STRAIGHT_TABLE.get(ranks as usize) {
                let hand_val = HandVal::new(HandType::Straight as u8, *top_card, 0, 0, 0, 0);
                println!("Retourne HandVal pour Straight avec top_card {}", *top_card);
                return hand_val;
                
            }
        }
    
        if n_dups < 3 {
            // Si on a déjà une main formée et pas assez de duplicatas pour un full house ou un carré
            let hand_val = HandVal::new(0, 0, 0, 0, 0, 0); // ou toute autre valeur par défaut
            println!("Retourne HandVal pour aucune main formée: {:?}", hand_val);
            return hand_val;
        }
    
        // Ici, implémenter la logique pour les autres types de mains
        // comme les paires, les brelans, les fulls, les carrés, etc.
    
        // ...
    
        // Retour par défaut si aucune main n'est formée
        let hand_val = HandVal::new(0, 0, 0, 0, 0, 0);
        println!("Retourne HandVal par défaut: {:?}", hand_val);
        hand_val;


    match n_dups {
        0 => {
            // C'est une main sans paire
            let hand_val = HandVal::new(
                HandType::NoPair as u8,
                TOP_FIVE_CARDS_TABLE[ranks as usize].try_into().unwrap(),
                0, 0, 0, 0
            );
            println!("Retourne HandVal pour NoPair: {:?}", hand_val);
            return hand_val;
            
        },
        1 => {
            // C'est une main avec une paire
            let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
            let t = ranks ^ two_mask;
            let kickers = (TOP_FIVE_CARDS_TABLE[t as usize] >> CARD_WIDTH) & !FIFTH_CARD_MASK;
            let hand_val = HandVal::new(
                HandType::OnePair as u8,
                TOP_CARD_TABLE[two_mask as usize],
                kickers.try_into().unwrap(), 0, 0, 0
            );
            println!("Retourne HandVal pour OnePair: {:?}", hand_val);
            return hand_val;
        },
        2 => {
            // Soit deux paires, soit un brelan
            let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
            if two_mask != 0 {
                // Deux paires
                let t = ranks ^ two_mask;
                let hand_val = HandVal::new(
                    HandType::TwoPair as u8,
                    TOP_FIVE_CARDS_TABLE[two_mask as usize].try_into().unwrap(),
                    TOP_CARD_TABLE[t as usize], 0, 0, 0
                );
                println!("Retourne HandVal pour TwoPair: {:?}", hand_val);
                return hand_val;
            } else {
                // Un brelan
                let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                let t = ranks ^ three_mask;
                let second = TOP_CARD_TABLE[t as usize];
                let hand_val = HandVal::new(
                    HandType::Trips as u8,
                    TOP_CARD_TABLE[three_mask as usize],
                    second,
                    TOP_CARD_TABLE[(t ^ (1 << second)) as u16 as usize], // Convertir en u16 avant de convertir en usize
                    0, 0
                );
                println!("Retourne HandVal pour Trips: {:?}", hand_val);
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
                println!("Retourne HandVal pour Quads : {:?}", hand_val);
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
                println!("Retourne HandVal pourFullHouse: {:?}", hand_val);
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
            println!("Retourne HandVal pour TwoPair: {:?}", hand_val);
            return hand_val;
        }
        }
    }

}
