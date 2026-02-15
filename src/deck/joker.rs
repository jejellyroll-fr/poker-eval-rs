use super::std_deck::*;
use super::traits::{CardMask, Deck};
use crate::tables::t_cardmasks::StdDeckCardMask;
pub use crate::tables::t_jokercardmasks::JokerDeckCardMask;
use crate::tables::t_jokercardmasks::*;

// Constants
pub const JOKER_DECK_N_CARDS: usize = 53;
pub const JOKER_DECK_RANK_CHARS: &str = "23456789TJQKA";
pub const JOKER_DECK_SUIT_CHARS: &str = "hdcs";

// Function to get the mask for a specific index
/// Gets the `JokerDeckCardMask` for a specific card index.
pub fn joker_deck_mask(index: usize) -> JokerDeckCardMask {
    JOKER_DECK_CARD_MASKS_TABLE[index]
}

// Ranks
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
// Constants for first and last ranks
pub const JOKER_DECK_RANK_FIRST: usize = STD_DECK_RANK_FIRST;
pub const JOKER_DECK_RANK_LAST: usize = STD_DECK_RANK_LAST;
// Suits
pub const JOKER_DECK_SUIT_HEARTS: usize = STD_DECK_SUIT_HEARTS;
pub const JOKER_DECK_SUIT_DIAMONDS: usize = STD_DECK_SUIT_DIAMONDS;
pub const JOKER_DECK_SUIT_CLUBS: usize = STD_DECK_SUIT_CLUBS;
pub const JOKER_DECK_SUIT_SPADES: usize = STD_DECK_SUIT_SPADES;
pub const JOKER_DECK_SUIT_COUNT: usize = STD_DECK_SUIT_COUNT;
// Constants for first and last ranks
pub const JOKER_DECK_SUIT_FIRST: usize = STD_DECK_SUIT_FIRST;
pub const JOKER_DECK_SUIT_LAST: usize = STD_DECK_SUIT_LAST;

// N_RANKMASKS used for bitmask calculations
pub const JOKER_DECK_N_RANKMASKS: usize = STD_DECK_N_RANKMASKS;
pub const JOKER_DECK_JOKER: usize = JOKER_DECK_N_CARDS - 1;

impl Default for JokerDeckCardMask {
    fn default() -> Self {
        Self::new()
    }
}

impl JokerDeckCardMask {
    /// Creates a new empty `JokerDeckCardMask`.
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
    // Other mask operations
    pub fn or(&mut self, other: &Self) {
        self.cards_n |= other.cards_n;
    }
    pub fn and(&mut self, other: &Self) {
        self.cards_n &= other.cards_n;
    }
    pub fn not(&mut self) {
        self.cards_n = !self.cards_n;
    }
    pub fn xor(&mut self, other: &Self) {
        self.cards_n ^= other.cards_n;
    }
    /// Returns the mask for a specific card index.
    pub fn get_mask(index: usize) -> JokerDeckCardMask {
        JOKER_DECK_CARD_MASKS_TABLE[index]
    }
    /// Converts the mask to a vector of card indices.
    pub fn mask_to_cards(&self) -> Vec<usize> {
        let mut cards = Vec::new();
        for i in (0..JOKER_DECK_N_CARDS).rev() {
            if self.card_is_set(i) {
                cards.push(i);
            }
        }
        cards
    }

    /// Checks if the joker is set in the mask.
    pub fn is_joker_set(&self) -> bool {
        self.cards_n & (1 << JOKER_DECK_JOKER) != 0
    }

    /// Sets or unsets the joker.
    pub fn set_joker(&mut self, joker: bool) {
        let joker_bit = 1 << 52; // Adjust the joker bit
        self.cards_n = if joker {
            self.cards_n | joker_bit
        } else {
            self.cards_n & !joker_bit
        };
    }
    /// Checks if a specific card index is set.
    pub fn card_is_set(&self, index: usize) -> bool {
        (self.cards_n & (1 << index)) != 0
    }
    /// Converts to a standard deck mask (ignoring the joker).
    pub fn to_std(self) -> StdDeckCardMask {
        let mut s_cards = StdDeckCardMask::new();
        s_cards.reset();

        s_cards.set_spades(self.spades() as u16);
        s_cards.set_hearts(self.hearts() as u16);
        s_cards.set_clubs(self.clubs() as u16);
        s_cards.set_diamonds(self.diamonds() as u16);

        s_cards
    }

    /// Resets the mask.
    pub fn reset(&mut self) {
        self.cards_n = 0;
    }

    /// Checks if the mask is empty.
    pub fn is_empty(&self) -> bool {
        self.cards_n == 0
    }

    /// Checks if two masks are equal.
    pub fn equals(&self, other: &Self) -> bool {
        self.cards_n == other.cards_n
    }

    /// Counts the number of cards in the mask.
    pub fn num_cards(&self) -> usize {
        (0..JOKER_DECK_N_CARDS)
            .filter(|&i| self.card_is_set(i))
            .count()
    }

    /// Sets a card index in the mask.
    pub fn set(&mut self, card_index: usize) {
        self.cards_n |= 1 << card_index;
    }
    pub fn mask_to_string(&self) -> String {
        let mut card_strings = Vec::new();
        for card_index in 0..JOKER_DECK_N_CARDS {
            if self.card_is_set(card_index) {
                let card_str = JokerDeck::card_to_string(card_index);
                card_strings.push(card_str);
            }
        }
        card_strings.join(" ")
    }
}

impl CardMask for JokerDeckCardMask {
    fn new() -> Self {
        Self::new()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn and(&mut self, other: &Self) {
        self.and(other)
    }

    fn or(&mut self, other: &Self) {
        self.or(other)
    }

    fn xor(&mut self, other: &Self) {
        self.xor(other)
    }

    fn not(&mut self) {
        self.not()
    }

    fn num_cards(&self) -> usize {
        self.num_cards()
    }

    fn card_is_set(&self, index: usize) -> bool {
        self.card_is_set(index)
    }

    fn set(&mut self, index: usize) {
        self.set(index)
    }

    fn mask_to_string(&self) -> String {
        self.mask_to_string()
    }
}

/// Joker Deck utilities.
pub struct JokerDeck;

impl JokerDeck {
    /// Converts a card index to a string (handles "Xx" for Joker).
    pub fn card_to_string(card_index: usize) -> String {
        if card_index == JOKER_DECK_JOKER {
            "Xx".to_string()
        } else {
            let rank_char = JOKER_DECK_RANK_CHARS
                .chars()
                .nth(StdDeck::rank(card_index).as_usize())
                .unwrap_or('?');
            let suit_char = JOKER_DECK_SUIT_CHARS
                .chars()
                .nth(StdDeck::suit(card_index).as_usize())
                .unwrap_or('?');
            format!("{}{}", rank_char, suit_char)
        }
    }

    /// Converts a string to a card index.
    pub fn string_to_card(in_string: &str) -> Option<usize> {
        if in_string.to_uppercase() == "XX" {
            Some(JOKER_DECK_JOKER)
        } else {
            StdDeck::string_to_card(in_string)
        }
    }

    /// Parses a string of cards into a `JokerDeckCardMask`.
    pub fn string_to_mask(in_string: &str) -> Result<(JokerDeckCardMask, usize), String> {
        let mut out_mask = JokerDeckCardMask::new();
        let mut n = 0;

        for chunk in in_string.chars().collect::<Vec<char>>().chunks(2) {
            if chunk.len() != 2 {
                return Err(format!("Invalid card format: {:?}", chunk));
            }
            let (rank_char, suit_char) = (chunk[0], chunk[1]);

            if rank_char == ' ' {
                continue;
            }

            // Handle joker: "Xx" or "xx" or "XX"
            if rank_char.eq_ignore_ascii_case(&'X') && suit_char.eq_ignore_ascii_case(&'x') {
                out_mask.set(JOKER_DECK_JOKER);
                n += 1;
                continue;
            }

            let rank = JOKER_DECK_RANK_CHARS.find(rank_char.to_ascii_uppercase());
            let suit = JOKER_DECK_SUIT_CHARS.find(suit_char.to_ascii_lowercase());

            match (rank, suit) {
                (Some(rank), Some(suit)) => {
                    let card = StdDeck::make_card(Rank::from(rank), Suit::from(suit));
                    out_mask.set(card);
                    n += 1;
                }
                _ => {
                    return Err(format!(
                        "Unrecognized card character: {}{}",
                        rank_char, suit_char
                    ))
                }
            }
        }

        Ok((out_mask, n))
    }
}

impl Deck for JokerDeck {
    type Mask = JokerDeckCardMask;
    const N_CARDS: usize = JOKER_DECK_N_CARDS;

    fn card_to_string(card_index: usize) -> String {
        Self::card_to_string(card_index)
    }

    fn string_to_card(card_str: &str) -> Option<usize> {
        Self::string_to_card(card_str)
    }

    fn string_to_mask(cards_str: &str) -> Result<(Self::Mask, usize), String> {
        Self::string_to_mask(cards_str)
    }
}
