use crate::deck_std::*;
use crate::t_cardmasks::StdDeckCardMask;
use crate::t_jokercardmasks::*;

// Constants
pub const JOKER_DECK_N_CARDS: usize = 53;

// Function to get the mask for a specific index
pub fn joker_deck_mask(index: usize) -> JokerDeck_CardMask {
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
}

// Additional implementations, if needed, based on the C source
