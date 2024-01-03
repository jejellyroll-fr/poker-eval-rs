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

        // Exemple de logique pour obtenir les cartes significatives
        if N_SIG_CARDS[hand_type.as_usize()] >= 1 {
            result.push_str(&format!(" {}", self.top_card())); // Utilisez votre propre logique pour obtenir le caractère de la carte
        }
        if N_SIG_CARDS[hand_type.as_usize()] >= 2 {
            result.push_str(&format!(" {}", self.second_card()));
        }
        if N_SIG_CARDS[hand_type.as_usize()] >= 3 {
            result.push_str(&format!(" {}", self.third_card()));
        }
        if N_SIG_CARDS[hand_type.as_usize()] >= 4 {
            result.push_str(&format!(" {}", self.fourth_card()));
        }
        if N_SIG_CARDS[hand_type.as_usize()] >= 5 {
            result.push_str(&format!(" {}", self.fifth_card()));
        }
        // ... et ainsi de suite pour third_card, fourth_card, fifth_card ...

        result.push(')');
        result
    }




    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    // Autres méthodes nécessaires pour HandVal...
}
