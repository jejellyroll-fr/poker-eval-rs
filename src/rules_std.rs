use crate::handval::HandVal;

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

    pub fn to_string(&self) -> String {
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
            };

            let card_char = match card_value {
                0 => '2', 1 => '3', 2 => '4', 3 => '5', 4 => '6',
                5 => '7', 6 => '8', 7 => '9', 8 => 'T', 9 => 'J',
                10 => 'Q', 11 => 'K', 12 => 'A',
                _ => '?' // pour les valeurs non reconnues
            };

            result.push(' ');
            result.push(card_char);
        }

        result.push(')');
        result
    }




    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    // Autres méthodes nécessaires pour HandVal...
}
