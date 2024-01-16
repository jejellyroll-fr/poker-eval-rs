use crate::deck_std::{
    STD_DECK_RANK_4, STD_DECK_RANK_5, STD_DECK_RANK_6, STD_DECK_RANK_7, STD_DECK_RANK_8,
    STD_DECK_RANK_ACE, STD_DECK_RANK_COUNT,
};
use crate::rules_std::HandType;

// Constantes pour les décalages et masques
pub const HANDTYPE_SHIFT: u32 = 24;
pub const HANDTYPE_MASK: u32 = 0x0F000000;
pub const CARDS_SHIFT: u32 = 0;
pub const CARDS_MASK: u32 = 0x000FFFFF;
pub const TOP_CARD_SHIFT: u32 = 16;
pub const TOP_CARD_MASK: u32 = 0x000F0000;
pub const SECOND_CARD_SHIFT: u32 = 12;
pub const SECOND_CARD_MASK: u32 = 0x0000F000;
pub const THIRD_CARD_SHIFT: u32 = 8;
pub const THIRD_CARD_MASK: u32 = 0x00000F00;
pub const FOURTH_CARD_SHIFT: u32 = 4;
pub const FOURTH_CARD_MASK: u32 = 0x000000F0;
pub const FIFTH_CARD_SHIFT: u32 = 0;
pub const FIFTH_CARD_MASK: u32 = 0x0000000F;
pub const CARD_WIDTH: u32 = 4;
pub const CARD_MASK: u32 = 0x0F;
// Définition des constantes directement sans utiliser de fonctions
pub const LOW_HAND_VAL_NOTHING: u32 =
    (HandType::StFlush as u32) << HANDTYPE_SHIFT | (STD_DECK_RANK_ACE as u32 + 1) << TOP_CARD_SHIFT;
pub const LOW_HAND_VAL_WORST_EIGHT: u32 = (HandType::NoPair as u32) << HANDTYPE_SHIFT
    | (STD_DECK_RANK_8 as u32 + 1) << TOP_CARD_SHIFT
    | (STD_DECK_RANK_7 as u32 + 1) << SECOND_CARD_SHIFT
    | (STD_DECK_RANK_6 as u32 + 1) << THIRD_CARD_SHIFT
    | (STD_DECK_RANK_5 as u32 + 1) << FOURTH_CARD_SHIFT
    | (STD_DECK_RANK_4 as u32 + 1) << FIFTH_CARD_SHIFT;

#[derive(Debug)]
pub struct LowHandVal {
    pub value: u32,
}

impl LowHandVal {
    pub fn new(hand_type: u8, top: u8, second: u8, third: u8, fourth: u8, fifth: u8) -> Self {
        let mut value = ((hand_type as u32) << HANDTYPE_SHIFT) & HANDTYPE_MASK;
        value |= ((top as u32) << TOP_CARD_SHIFT) & TOP_CARD_MASK;
        value |= ((second as u32) << SECOND_CARD_SHIFT) & SECOND_CARD_MASK;
        value |= ((third as u32) << THIRD_CARD_SHIFT) & THIRD_CARD_MASK;
        value |= ((fourth as u32) << FOURTH_CARD_SHIFT) & FOURTH_CARD_MASK;
        value |= ((fifth as u32) << FIFTH_CARD_SHIFT) & FIFTH_CARD_MASK;

        LowHandVal { value }
    }

    // Méthodes d'extraction
    pub fn hand_type(&self) -> u8 {
        ((self.value & HANDTYPE_MASK) >> HANDTYPE_SHIFT) as u8
    }

    pub fn top_card(&self) -> u8 {
        ((self.value & TOP_CARD_MASK) >> TOP_CARD_SHIFT) as u8
    }
    pub fn second_card(&self) -> u8 {
        ((self.value & SECOND_CARD_MASK) >> SECOND_CARD_SHIFT) as u8
    }

    pub fn third_card(&self) -> u8 {
        ((self.value & THIRD_CARD_MASK) >> THIRD_CARD_SHIFT) as u8
    }

    pub fn fourth_card(&self) -> u8 {
        ((self.value & FOURTH_CARD_MASK) >> FOURTH_CARD_SHIFT) as u8
    }

    pub fn fifth_card(&self) -> u8 {
        ((self.value & FIFTH_CARD_MASK) >> FIFTH_CARD_SHIFT) as u8
    }

    // Convertir en chaîne de caractères (représentation lisible)
    pub fn to_string(&self) -> String {
        let hand_type_str = match self.hand_type() {
            0 => "Low NoPair",
            1 => "No Low OnePair",
            2 => "No Low TwoPair",
            3 => "No Low Trips",
            4 => "No Low Straight",
            5 => "No Low Flush",
            6 => "No Low FullHouse",
            7 => "No Low Quads",
            8 => "No Low StFlush",
            _ => "Unknown",
        };
        // Ajouter des println! pour déboguer les valeurs extraites
        //println!("LowHandVal to_string: HandType: {}, TopCard: {}, SecondCard: {}, ThirdCard: {}, FourthCard: {}, FifthCard: {}", hand_type_str, self.top_card(), self.second_card(), self.third_card(), self.fourth_card(), self.fifth_card());

        format!(
            "{} ( {} {} {} {} {})",
            hand_type_str,
            self.top_card(),
            self.second_card(),
            self.third_card(),
            self.fourth_card(),
            self.fifth_card()
        )
    }

    // Imprimer le LowHandVal
    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    // Méthode pour faire la rotation des rangs (pour la gestion des As en Omaha Hi/Lo)
    pub fn rotate_ranks(ranks: u32) -> u32 {
        let ace_bit = ranks & (1 << STD_DECK_RANK_ACE);
        let without_ace = ranks & !ace_bit;
        let shifted = without_ace << 1;
        shifted | ace_bit
    }

    // Méthode pour ajouter un joker fictif dans les rangs (si nécessaire)
    pub fn jokerfy_ranks(mut ranks: u32) {
        for j in 0..STD_DECK_RANK_COUNT {
            if ranks & (1 << j) == 0 {
                ranks |= 1 << j;
            }
        }
    }
}
