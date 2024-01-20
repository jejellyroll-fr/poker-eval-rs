use crate::deck_std::*;
use crate::t_cardmasks::StdDeckCardMask;
use crate::t_jokercardmasks::*;


// Constants
pub const JOKER_DECK_N_CARDS: usize = 53;
pub const JOKER_DECK_RANK_CHARS: &str = "23456789TJQKA";
pub const JOKER_DECK_SUIT_CHARS: &str = "hdcs";

// Function to get the mask for a specific index
// Function to get the mask for a specific index
pub fn joker_deck_mask(index: usize) -> JokerDeckCardMask {
    JOKER_DECK_CARD_MASKS_TABLE[index]
}


// Rangs
pub const JOKER_DECK_RANK_2: usize = STD_DECK_RANK_2;
pub const JOKER_DECK_RANK_3: usize = STD_DECK_RANK_3;
pub const JOKER_DECK_RANK_4: usize = STD_DECK_RANK_4;
pub const JOKER_DECK_RANK_5: usize = STD_DECK_RANK_5;
pub const JOKER_DECK_RANK_6: usize = STD_DECK_RANK_6;
pub const JOKER_DECK_RANK_7: usize = STD_DECK_RANK_7;
pub const JOKER_DECK_RANK_8: usize = STD_DECK_RANK_8;
pub const JOKER_DECK_RANK_9: usize = STD_DECK_RANK_9;
pub const JOKER_DECK_RANK_TEN: usize = STD_DECK_RANK_TEN;
pub const JOKER_DECK_RANK_JACK: usize = STD_DECK_RANK_JACK;
pub const JOKER_DECK_RANK_QUEEN: usize = STD_DECK_RANK_QUEEN;
pub const JOKER_DECK_RANK_KING: usize = STD_DECK_RANK_KING;
pub const JOKER_DECK_RANK_ACE: usize = STD_DECK_RANK_ACE;
pub const JOKER_DECK_RANK_COUNT: usize = STD_DECK_RANK_COUNT;
// Constantes pour les premiers et derniers rangs
pub const JOKER_DECK_RANK_FIRST: usize = STD_DECK_RANK_FIRST;
pub const JOKER_DECK_RANK_LAST: usize = STD_DECK_RANK_LAST;
// Couleurs
pub const JOKER_DECK_SUIT_HEARTS: usize = STD_DECK_SUIT_HEARTS;
pub const JOKER_DECK_SUIT_DIAMONDS: usize = STD_DECK_SUIT_DIAMONDS;
pub const JOKER_DECK_SUIT_CLUBS: usize = STD_DECK_SUIT_CLUBS;
pub const JOKER_DECK_SUIT_SPADES: usize = STD_DECK_SUIT_SPADES;
pub const JOKER_DECK_SUIT_COUNT: usize = STD_DECK_SUIT_COUNT;
// Constantes pour les premiers et derniers rangs
pub const JOKER_DECK_SUIT_FIRST: usize = STD_DECK_SUIT_FIRST;
pub const JOKER_DECK_SUIT_LAST: usize = STD_DECK_SUIT_LAST;


// N_RANKMASKS utilisé pour les calculs de masque de bit
pub const JOKER_DECK_N_RANKMASKS: usize = STD_DECK_N_RANKMASKS;
pub const JOKER_DECK_JOKER: usize = JOKER_DECK_N_CARDS - 1;



impl JokerDeckCardMask {
    pub fn new() -> Self {
        JokerDeckCardMask { cards_n: 0 }
    }

    pub fn spades(&self) -> u64 {
        (self.cards_n >> 39) & 0x1FFF
    }

    pub fn hearts(&self) -> u64 {
        (self.cards_n >> 26) & 0x1FFF
    }

    pub fn clubs(&self) -> u64 {
        (self.cards_n >> 13) & 0x1FFF
    }

    pub fn diamonds(&self) -> u64 {
        self.cards_n & 0x1FFF
    }
    // Autres opérations sur les masques (exemple: OR, AND, etc.)
    pub fn or (&self, other: Self) -> Self {
        JokerDeckCardMask { cards_n: self.cards_n | other.cards_n }
    }
    pub fn and (&self, other: Self) -> Self {
        JokerDeckCardMask { cards_n: self.cards_n & other.cards_n }
    }
    pub fn not (&self) -> Self {
        JokerDeckCardMask { cards_n:!self.cards_n }
    }
    pub fn xor (&self, other: Self) -> Self {
        JokerDeckCardMask { cards_n: self.cards_n ^ other.cards_n }
    }
    // Autres méthodes si nécessaires...
    pub fn get_mask(index: usize) -> JokerDeckCardMask {
        JOKER_DECK_CARD_MASKS_TABLE[index]
    }
    

    pub fn mask_to_cards(&self) -> Vec<usize> {
        let mut cards = Vec::new();
        for i in (0..JOKER_DECK_N_CARDS).rev() {
            if self.card_is_set(i) {
                cards.push(i);
            }
        }
        cards
    }

    // ...

    // Assuming Joker is a single bit, adjust this based on your representation
    pub fn set_joker(&mut self, joker: bool) {
        let joker_bit = 1 << 52; // Adjust this based on where the joker bit is
        self.cards_n = if joker { self.cards_n | joker_bit } else { self.cards_n & !joker_bit };
    }
    // Méthode pour vérifier si une carte est présente dans le masque
    pub fn card_is_set(&self, index: usize) -> bool {
        (self.cards_n & (1 << index)) != 0
    }

    pub fn to_std(&self) -> StdDeckCardMask {
        let mut s_cards = StdDeckCardMask::new(); // Assuming you have a constructor for StdDeckCardMask
        s_cards.reset();

        // Set spades, hearts, clubs, diamonds from JokerDeckCardMask
        // Assuming you have methods to get these values from JokerDeckCardMask
        // and set these values in StdDeckCardMask
        s_cards.set_spades(self.spades() as u16);
        s_cards.set_hearts(self.hearts() as u16);
        s_cards.set_clubs(self.clubs() as u16);
        s_cards.set_diamonds(self.diamonds() as u16);

        s_cards
    }    

    // Méthode pour réinitialiser le masque
    pub fn reset(&mut self) {
        self.cards_n = 0;
    }

    // Méthode pour vérifier si le masque est vide
    pub fn is_empty(&self) -> bool {
        self.cards_n == 0
    }

    // Méthode pour vérifier si deux masques sont égaux
    pub fn equals(&self, other: &Self) -> bool {
        self.cards_n == other.cards_n
    }

    // Méthode pour compter le nombre de cartes dans un masque
    pub fn num_cards(&self) -> usize {
        (0..JOKER_DECK_N_CARDS)
            .filter(|&i| self.card_is_set(i))
            .count()
    }   
    

    // Méthode pour ajouter une carte au masque
    pub fn set(&mut self, card_index: usize) {
        self.cards_n |= 1 << card_index;
    }    
}


pub struct JokerDeck;

impl JokerDeck {
    // Define the JokerDeck_RANK function
    fn joker_deck_rank(index: usize) -> usize {
        StdDeck::rank(index)
    }

    // Define the JokerDeck_SUIT function
    fn joker_deck_suit(index: usize) -> usize {
        StdDeck::suit(index)
    }

    // Define the JokerDeck_MAKE_CARD function
    fn joker_deck_make_card(rank: usize, suit: usize) -> usize {
        StdDeck::make_card(rank, suit)
    }

    // Conversion d'une carte en chaîne de caractères
    pub fn card_to_string(card_index: usize) -> String {
        if card_index == JOKER_DECK_JOKER {
            "Xx".to_string()
        } else {
            let rank_char = JOKER_DECK_RANK_CHARS
                .chars()
                .nth(StdDeck::rank(card_index))
                .unwrap();
            let suit_char = JOKER_DECK_SUIT_CHARS
                .chars()
                .nth(StdDeck::suit(card_index))
                .unwrap();
            format!("{}{}", rank_char, suit_char)
        }
    }
    
    // Conversion d'une chaîne de caractères en carte
    pub fn string_to_card(in_string: &str) -> Option<usize> {
        if in_string.to_uppercase() == "XX" {
            Some(JOKER_DECK_JOKER)
        } else {
            StdDeck::string_to_card(in_string)  // Assuming StdDeck has a similar method
        }
    }

    // Conversion d'une chaîne de caractères représentant des cartes en un masque de cartes
    pub fn string_to_mask(in_string: &str) -> Result<(JokerDeckCardMask, usize), String> {
        let mut out_mask = JokerDeckCardMask::new();
        let mut n = 0;

        for chunk in in_string.chars().collect::<Vec<char>>().chunks(2) {
            if chunk.len() != 2 {
                return Err(format!("Format de carte invalide : {:?}", chunk));
            }
            let (rank_char, suit_char) = (chunk[0], chunk[1]);

            if rank_char == ' ' {
                continue;
            }

            let rank = JOKER_DECK_RANK_CHARS.find(rank_char.to_ascii_uppercase());
            let suit = JOKER_DECK_SUIT_CHARS.find(suit_char.to_ascii_lowercase());

            match (rank, suit) {
                (Some(rank), Some(suit)) => {
                    let card = StdDeck::make_card(rank, suit); // Using StdDeck::make_card
                    out_mask.set(card);
                    n += 1;
                },
                _ => return Err(format!("Caractère de carte non reconnu : {}{}", rank_char, suit_char)),
            }
        }

        Ok((out_mask, n))
    }
}


// Additional implementations, if needed, based on the C source
