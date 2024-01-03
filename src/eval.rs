use crate::handval::HandVal;
use crate::deck_std::StdDeckCardMask;
use crate::rules_std::HandType;

impl Eval {
    pub fn eval_n(cards: &StdDeckCardMask, n_cards: usize) -> HandVal {
        let ss = cards.spades();
        let sc = cards.clubs();
        let sd = cards.diamonds();
        let sh = cards.hearts();

        let ranks = ss | sc | sd | sh;
        let n_ranks = Self::count_bits(ranks);
        let n_dups = n_cards - n_ranks as usize;

        if n_ranks >= 5 {
            if Self::is_flush(ss) {
                if let Some(top_card) = Self::is_straight(ss) {
                    return HandVal::new(HandType::StFlush as u8, top_card, 0, 0, 0, 0);
                } else {
                    return HandVal::new(HandType::Flush as u8, Self::top_card(ss), 0, 0, 0, 0);
                }
            }
            // Répéter pour les autres couleurs
            if Self::is_flush(sc) {
                if let Some(top_card) = Self::is_straight(sc) {
                    return HandVal::new(HandType::StFlush as u8, top_card, 0, 0, 0, 0);
                } else {
                    return HandVal::new(HandType::Flush as u8, Self::top_card(sc), 0, 0, 0, 0);
                }
            }
            // Répéter pour sd
            if Self::is_flush(sd) {
                if let Some(top_card) = Self::is_straight(sd) {
                    return HandVal::new(HandType::StFlush as u8, top_card, 0, 0, 0, 0);
                } else {
                    return HandVal::new(HandType::Flush as u8, Self::top_card(sd), 0, 0, 0, 0);
                }
            }
            // Répéter pour sh
            if Self::is_flush(sh) {
                if let Some(top_card) = Self::is_straight(sh) {
                    return HandVal::new(HandType::StFlush as u8, top_card, 0, 0, 0, 0);
                } else {
                    return HandVal::new(HandType::Flush as u8, Self::top_card(sh), 0, 0, 0, 0);
                }
            }



            if let Some(top_card) = Self::is_straight(ranks) {
                return HandVal::new(HandType::Straight as u8, top_card, 0, 0, 0, 0);
            }
        }

        if n_dups < 3 {
            // Renvoie immédiatement si aucun full house ou quads n'est possible
            return HandVal::new(HandType::NoPair as u8, 0, 0, 0, 0, 0);
        }

        // Autre logique pour les mains de poker...
    }

    fn count_bits(bits: u64) -> u32 {
        bits.count_ones()
    }

    fn is_flush(ranks: u64) -> bool {
        // Implémentez la logique pour déterminer si c'est un flush
    }

    fn is_straight(ranks: u64) -> Option<u8> {
        // Implémentez la logique pour déterminer si c'est un straight
        // Renvoie Some(top_card) si c'est un straight, sinon None
    }

    fn top_card(ranks: u64) -> u8 {
        // Implémentez la logique pour trouver la carte supérieure
    }

    // Autres fonctions auxiliaires...
}
