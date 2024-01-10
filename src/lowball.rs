use crate::handval_low::{LowHandVal, LOW_HAND_VAL_NOTHING};
use crate::deck_std::{StdDeck, STD_DECK_RANK_ACE, STD_DECK_RANK_COUNT, STD_DECK_RANK_8, STD_DECK_RANK_7, STD_DECK_RANK_6, STD_DECK_RANK_5, STD_DECK_RANK_4, STD_DECK_RANK_2, STD_DECK_RANK_CHARS};
use crate::rules_std::{HandType, HAND_TYPE_NAMES, N_SIG_CARDS};

impl LowHandVal {
    pub fn to_lowball_string(&self) -> String {
        let mut result = String::new();

        if self.value == LOW_HAND_VAL_NOTHING {
            result.push_str("No Low");
        } else {
            let hand_type = HandType::from_usize(self.hand_type() as usize);
            let hand_type_str = HAND_TYPE_NAMES[hand_type.as_usize()];

            result.push_str(format!(" {} (", hand_type_str).as_str());

            for i in 0..N_SIG_CARDS[hand_type.as_usize()] {
                let card_rank = match i {
                    0 => self.top_card(),
                    1 => self.second_card(),
                    2 => self.third_card(),
                    3 => self.fourth_card(),
                    4 => self.fifth_card(),
                    _ => continue,
                };
                if i > 0 { result.push(' '); }
                let card_char = STD_DECK_RANK_CHARS.chars().nth(card_rank as usize).unwrap_or('?');
                result.push(card_char);
            }
            result.push(')');
        }

        result
    }

    // Cette méthode imprime la représentation de HandVal
    pub fn low_handval_print(&self) {
        let hand_string = self.to_lowball_string();
        println!("{}", hand_string);
    }
}
