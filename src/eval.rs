use crate::handval::{HandVal, CARD_WIDTH, FIFTH_CARD_MASK} ;
use crate::t_cardmasks::StdDeckCardMask;
use crate::deck_std::*;
use crate::rules_std::HandType;
use crate::t_nbits::NBITS_TABLE;
use crate::t_straight::STRAIGHT_TABLE;
use crate::t_topfivecards::TOP_FIVE_CARDS_TABLE;
use crate::t_topcard::TOP_CARD_TABLE;


pub struct Eval;


impl Eval {
    pub fn eval_n(cards: &StdDeckCardMask, n_cards: usize) -> HandVal {
        let ss = 0x1FFF;
        let sc = 0x1FFF << 13;
        let sd = 0x1FFF << 26;
        let sh = 0x1FFF << 39;
    
        let ranks = ss | sc | sd | sh;
        let n_ranks = NBITS_TABLE[ranks as usize];
        let n_dups = n_cards - n_ranks as usize;

    
        if n_ranks >= 5 {
            // Vérifier les flushes et les straight flushes
            for suit in [ss, sc, sd, sh].iter() {
                if NBITS_TABLE[*suit as usize] >= 5 {
                    if let Some(top_card) = STRAIGHT_TABLE.get(*suit as usize) {
                        return HandVal::new(HandType::StFlush as u8, *top_card, 0, 0, 0, 0);
                    } else {
                        return HandVal::new(
                            HandType::Flush as u8,
                            TOP_FIVE_CARDS_TABLE[*suit as usize].try_into().unwrap(),
                            0, 0, 0, 0
                        );                    }
                }
            }
    
            if let Some(top_card) = STRAIGHT_TABLE.get(ranks as usize) {
                return HandVal::new(HandType::Straight as u8, *top_card, 0, 0, 0, 0);
            }
        }
    
        if n_dups < 3 {
            // Si on a déjà une main formée et pas assez de duplicatas pour un full house ou un carré
            return HandVal::new(0, 0, 0, 0, 0, 0); // ou toute autre valeur par défaut
        }
    
        // Ici, implémenter la logique pour les autres types de mains
        // comme les paires, les brelans, les fulls, les carrés, etc.
    
        // ...
    
        // Retour par défaut si aucune main n'est formée
        HandVal::new(0, 0, 0, 0, 0, 0);


    match n_dups {
        0 => {
            // C'est une main sans paire
            HandVal::new(
                HandType::NoPair as u8,
                TOP_FIVE_CARDS_TABLE[ranks as usize].try_into().unwrap(),
                0, 0, 0, 0
            )
            
        },
        1 => {
            // C'est une main avec une paire
            let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
            let t = ranks ^ two_mask;
            let kickers = (TOP_FIVE_CARDS_TABLE[t as usize] >> CARD_WIDTH) & !FIFTH_CARD_MASK;
            HandVal::new(
                HandType::OnePair as u8,
                TOP_CARD_TABLE[two_mask as usize],
                kickers.try_into().unwrap(), 0, 0, 0
            )
            
        },
        2 => {
            // Soit deux paires, soit un brelan
            let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
            if two_mask != 0 {
                // Deux paires
                let t = ranks ^ two_mask;
                HandVal::new(
                    HandType::TwoPair as u8,
                    TOP_FIVE_CARDS_TABLE[two_mask as usize].try_into().unwrap(),
                    TOP_CARD_TABLE[t as usize], 0, 0, 0
                )
                
            } else {
                // Un brelan
                let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                let t = ranks ^ three_mask;
                let second = TOP_CARD_TABLE[t as usize];
                HandVal::new(
                    HandType::Trips as u8,
                    TOP_CARD_TABLE[three_mask as usize],
                    second, TOP_CARD_TABLE[t ^ (1 << second) as usize], 0, 0
                )
            }
        },
        _ => {
            // Carré (Quads)
            let four_mask = sh & sd & sc & ss;
            if four_mask != 0 {
                let tc = TOP_CARD_TABLE[four_mask as usize];
                return HandVal::new(
                    HandType::Quads as u8,
                    tc,
                    TOP_CARD_TABLE[(ranks ^ (1 << tc)) as usize],
                    0, 0, 0
                );
            }
        
            // Full House ou Quads
            let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
            if usize::from(NBITS_TABLE[two_mask as usize]) != n_dups {
                let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                let tc = TOP_CARD_TABLE[three_mask as usize];
                let t = (two_mask | three_mask) ^ (1 << tc);
                return HandVal::new(
                    HandType::FullHouse as u8,
                    tc,
                    TOP_CARD_TABLE[t as usize],
                    0, 0, 0
                );
            }
        
            // Deux Paires
            let top = TOP_CARD_TABLE[two_mask as usize];
            let second = TOP_CARD_TABLE[(two_mask ^ (1 << top)) as usize];
            return HandVal::new(
                HandType::TwoPair as u8,
                top,
                second,
                TOP_CARD_TABLE[(ranks ^ (1 << top) ^ (1 << second)) as usize],
                0, 0
            );
        }
        }
    }

}
