use crate::t_cardmasks::{ StdDeckCardMask, STD_DECK_CARD_MASKS_TABLE};

// Constantes
pub const STD_DECK_N_CARDS: usize = 52;
pub const STD_DECK_RANK_CHARS: &str = "23456789TJQKA";
pub const STD_DECK_SUIT_CHARS: &str = "hdcs";

// Rangs 
pub const STD_DECK_RANK_2: usize = 0;
pub const STD_DECK_RANK_3: usize = 1;
pub const STD_DECK_RANK_4: usize = 2;
pub const STD_DECK_RANK_5: usize = 3;
pub const STD_DECK_RANK_6: usize = 4;
pub const STD_DECK_RANK_7: usize = 5;
pub const STD_DECK_RANK_8: usize = 6;
pub const STD_DECK_RANK_9: usize = 7;
pub const STD_DECK_RANK_TEN: usize = 8;
pub const STD_DECK_RANK_JACK: usize = 9;
pub const STD_DECK_RANK_QUEEN: usize = 10;
pub const STD_DECK_RANK_KING: usize = 11;
pub const STD_DECK_RANK_ACE: usize = 12;
pub const STD_DECK_RANK_COUNT: usize = 13;
// Constantes pour les premiers et derniers rangs
pub const STD_DECK_RANK_FIRST: usize = STD_DECK_RANK_2;
pub const STD_DECK_RANK_LAST: usize = STD_DECK_RANK_ACE;
// Couleurs
pub const STD_DECK_SUIT_HEARTS: usize = 0;
pub const STD_DECK_SUIT_DIAMONDS: usize = 1;
pub const STD_DECK_SUIT_CLUBS: usize = 2;
pub const STD_DECK_SUIT_SPADES: usize = 3;
pub const STD_DECK_SUIT_COUNT: usize = 4;
// Constantes pour les premiers et derniers rangs
pub const STD_DECK_SUIT_FIRST: usize = STD_DECK_SUIT_HEARTS;
pub const STD_DECK_SUIT_LAST: usize = STD_DECK_SUIT_SPADES;

// N_RANKMASKS utilisé pour les calculs de masque de bit
pub const STD_DECK_N_RANKMASKS: usize = 1 << STD_DECK_RANK_COUNT;

pub const STD_DECK_CARDMASK_SPADES: u64 = 0x1fff; // 13 bits pour les piques
pub const STD_DECK_CARDMASK_CLUBS: u64 = 0x1fff << 16; // 13 bits pour les trèfles
pub const STD_DECK_CARDMASK_DIAMONDS: u64 = 0x1fff << 32; // 13 bits pour les carreaux
pub const STD_DECK_CARDMASK_HEARTS: u64 = 0x1fff << 48; // 13 bits pour les cœurs




impl StdDeckCardMask {
    pub fn new() -> Self {
        StdDeckCardMask { mask: 0 }
    }

    pub fn spades(&self) -> u16 {
        ((self.mask >> 48) & 0x1FFF) as u16 // 13 bits pour les piques
    }

    pub fn clubs(&self) -> u16 {
        ((self.mask >> 32) & 0x1FFF) as u16 // 13 bits pour les trèfles
    }

    pub fn diamonds(&self) -> u16 {
        ((self.mask >> 16) & 0x1FFF) as u16 // 13 bits pour les carreaux
    }

    pub fn hearts(&self) -> u16 {
        (self.mask & 0x1FFF) as u16 // 13 bits pour les cœurs
    }

    pub fn set_spades(&mut self, ranks: u16) {
        self.mask = (self.mask & !(0x1FFF << 48)) | ((ranks as u64) << 48);
    }

    pub fn set_clubs(&mut self, ranks: u16) {
        self.mask = (self.mask & !(0x1FFF << 32)) | ((ranks as u64) << 32);
    }

    pub fn set_diamonds(&mut self, ranks: u16) {
        self.mask = (self.mask & !(0x1FFF << 16)) | ((ranks as u64) << 16);
    }

    pub fn set_hearts(&mut self, ranks: u16) {
        self.mask = (self.mask & !0x1FFF) | ranks as u64;
    }


    // Autres opérations sur les masques (exemple: OR, AND, etc.)
    pub fn or(&mut self, other: &StdDeckCardMask) {
        self.mask |= other.mask;
    }

    pub fn and(&mut self, other: &StdDeckCardMask) {
        self.mask &= other.mask;
    }

    pub fn xor(&mut self, other: &StdDeckCardMask) {
        self.mask ^= other.mask;
    }

    pub fn not(&mut self) {
        self.mask = !self.mask;
    }


    // Autres méthodes si nécessaires...
    pub fn get_mask(index: usize) -> &'static Self {
        &STD_DECK_CARD_MASKS_TABLE[index]
    }

    // Méthode pour vérifier si une carte est présente dans le masque
    pub fn card_is_set(&self, index: usize) -> bool {
        let result = (self.mask & (1 << index)) != 0;
        // println!("Vérification de la carte à l'indice {}: {}", index, result); //debug
        result
    }

    // Méthode pour réinitialiser le masque
    pub fn reset(&mut self) {
        self.mask = 0;
    }

    // Méthode pour vérifier si le masque est vide
    pub fn is_empty(&self) -> bool {
        self.mask == 0
    }

    // Méthode pour vérifier si deux masques sont égaux
    pub fn equals(&self, other: &Self) -> bool {
        self.mask == other.mask
    }

    // Méthode pour compter le nombre de cartes dans un masque
    pub fn num_cards(&self) -> usize {
        (0..STD_DECK_N_CARDS).filter(|&i| self.card_is_set(i)).count()
    }

    // Méthode pour ajouter une carte au masque
    pub fn set(&mut self, card_index: usize) {
        // println!("Masque avant ajout: {:b}", self.mask); //debug
        self.mask |= 1 << card_index;
        // println!("Masque après ajout: {:b}", self.mask); //debug
    }



}

// Structure StdDeck
pub struct StdDeck;

impl StdDeck {
    // Méthode pour obtenir le rang d'une carte
    pub fn rank(index: usize) -> usize {
        index % STD_DECK_RANK_COUNT
    }

    // Méthode pour obtenir la couleur d'une carte
    pub fn suit(index: usize) -> usize {
        index / STD_DECK_RANK_COUNT
    }

    // Méthode pour créer une carte
    pub fn make_card(rank: usize, suit: usize) -> usize {
        suit * STD_DECK_RANK_COUNT + rank
    }

    // Conversion d'une carte en chaîne de caractères
    pub fn card_to_string(card_index: usize) -> String {
        let rank_char = STD_DECK_RANK_CHARS.chars().nth(Self::rank(card_index)).unwrap();
        let suit_char = STD_DECK_SUIT_CHARS.chars().nth(Self::suit(card_index)).unwrap();
        format!("{}{}", rank_char, suit_char)
    }

    // Conversion d'une chaîne de caractères en carte
    pub fn string_to_card(in_string: &str) -> Option<usize> {
        if in_string.len() != 2 {
            return None;
        }

        let rank_char = in_string.chars().next()?;
        let suit_char = in_string.chars().nth(1)?;

        let rank = STD_DECK_RANK_CHARS.find(rank_char.to_ascii_uppercase())?;
        let suit = STD_DECK_SUIT_CHARS.find(suit_char.to_ascii_lowercase())?;

        Some(Self::make_card(rank, suit))
    }
    // Conversion d'une chaîne de caractères représentant des cartes en un masque de cartes
    pub fn string_to_mask(in_string: &str) -> (StdDeckCardMask, usize) {
        let mut out_mask = StdDeckCardMask::new();
        let mut n = 0;

        for (rank_char, suit_char) in in_string.chars().collect::<Vec<char>>().chunks(2).map(|chunk| (chunk[0], chunk[1])) {
            if rank_char == ' ' {
                continue;
            }
            let rank = STD_DECK_RANK_CHARS.find(rank_char.to_ascii_uppercase());
            let suit = STD_DECK_SUIT_CHARS.find(suit_char.to_ascii_lowercase());

            if let (Some(rank), Some(suit)) = (rank, suit) {
                let card = Self::make_card(rank, suit);
                out_mask.set(card);
                // println!("Carte ajoutée: Rang = {}, Couleur = {}, Index = {}", rank_char, suit_char, card); //debug
                n += 1;
            }
        }

        // println!("Masque de cartes généré : {:b}", out_mask.mask); //debug

        (out_mask, n)
    }
}

