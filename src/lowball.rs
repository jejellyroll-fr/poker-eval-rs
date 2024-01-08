use crate::handval_low::{LOW_HAND_VAL_NOTHING}
use crate::deck_std::{StdDeck, STD_DECK_RANK_ACE, STD_DECK_RANK_COUNT, STD_DECK_RANK_8, STD_DECK_RANK_7, STD_DECK_RANK_6, STD_DECK_RANK_5, STD_DECK_RANK_4, STD_DECK_RANK_2};


impl LowHandVal {
    // ...

    // Méthode pour convertir une carte en son rang en lowball
    pub fn card_to_lowball_rank(card: u8) -> u8 {
        if card == STD_DECK_RANK_2 as u8 {
            STD_DECK_RANK_ACE as u8
        } else {
            card - 1
        }
    }

    pub fn to_lowball_string(&self) -> String {
        let mut result = String::new();

        if self.value == LOW_HAND_VAL_NOTHING {
            result.push_str("NoLow");
        } else {
            let hand_type = self.hand_type();
            result.push_str(format!("{} (", HandType::from_u8(hand_type).to_string()).as_str());

            let sig_cards = HandType::get_num_significant_cards(hand_type);
            for i in 0..sig_cards {
                let card_rank = match i {
                    0 => Self::card_to_lowball_rank(self.top_card()),
                    1 => Self::card_to_lowball_rank(self.second_card()),
                    2 => Self::card_to_lowball_rank(self.third_card()),
                    3 => Self::card_to_lowball_rank(self.fourth_card()),
                    4 => Self::card_to_lowball_rank(self.fifth_card()),
                    _ => continue,
                };
                if i > 0 {
                    result.push(' ');
                }
                result.push(StdDeck::Rank::from_u8(card_rank).to_char());
            }
            result.push(')');
        }

        result
    }

    // Imprimer la représentation lowball de LowHandVal
    pub fn print_lowball(&self) {
        println!("{}", self.to_lowball_string());
    }
}

// Les implémentations de HandType et StdDeck::Rank doivent être complétées pour gérer les conversions.

