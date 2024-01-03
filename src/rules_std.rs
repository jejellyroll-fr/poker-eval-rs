use crate::handval::HandVal;
// Dans rules_std.rs
use crate::deck_std::STD_DECK_RANK_CHARS;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandType {
    NoPair,
    OnePair,
    TwoPair,
    Trips,
    Straight,
    Flush,
    FullHouse,
    Quads,
    StFlush,
}


pub static HAND_TYPE_NAMES: [&str; 9] = [
    "NoPair", "OnePair", "TwoPair", "Trips", "Straight", "Flush", "FlHouse", "Quads", "StFlush",
];

// Ajout des noms de type de main rembourrés
pub static HAND_TYPE_NAMES_PADDED: [&str; 9] = [
    "NoPair  ", "OnePair ", "TwoPair ", "Trips   ", "Straight", 
    "Flush   ", "FlHouse ", "Quads   ", "StFlush "
];

pub static N_SIG_CARDS: [usize; 9] = [5, 4, 3, 3, 1, 5, 2, 2, 1];

// Représentation du "Five Straight" pour les straights de basse valeur.
pub const FIVE_STRAIGHT: u64 = (1 << 14) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5);


impl HandType {
    // Cette méthode doit retourner un HandType
    pub fn from_usize(value: usize) -> HandType {
        match value {
            0 => HandType::NoPair,
            1 => HandType::OnePair,
            2 => HandType::TwoPair,
            3 => HandType::Trips,
            4 => HandType::Straight,
            5 => HandType::Flush,
            6 => HandType::FullHouse,
            7 => HandType::Quads,
            8 => HandType::StFlush,
            _ => panic!("Invalid hand type value"),
        }
    }
    
    // Cette méthode retourne l'index de l'enum HandType
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl HandVal {
    pub fn get_hand_type(&self) -> HandType {
        // Ici, vous convertissez la valeur numérique du type de main en HandType
        HandType::from_usize(self.hand_type() as usize)
    }





    pub fn StdRules_HandVal_toString(&self) -> String {
        let hand_type = self.get_hand_type();
        let mut result = format!("{} (", HAND_TYPE_NAMES[hand_type.as_usize()]);

        for i in 0..N_SIG_CARDS[hand_type.as_usize()] {
            let card_value = match i {
                0 => self.top_card(),
                1 => self.second_card(),
                2 => self.third_card(),
                3 => self.fourth_card(),
                4 => self.fifth_card(),
                _ => unreachable!(),
            } as usize; // Convertissez card_value en usize ici

            let card_char = STD_DECK_RANK_CHARS.chars().nth(card_value).unwrap_or('?');
            result.push_str(&format!(" {}", card_char));
        }

        result.push(')');
        result
    }

    pub fn StdRules_HandVal_print(&self) {
        println!("{}", self.StdRules_HandVal_toString());
    }

    // Autres méthodes nécessaires pour HandVal...
}
