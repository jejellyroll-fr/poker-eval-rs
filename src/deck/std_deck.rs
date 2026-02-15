use super::traits::{CardMask, Deck};
pub use crate::tables::t_cardmasks::StdDeckCardMask;
use crate::tables::t_cardmasks::STD_DECK_CARD_MASKS_TABLE;

// Constants
/// Number of cards in a standard deck (52).
pub use crate::tables::t_cardmasks::STD_DECK_N_CARDS;
/// Characters representing ranks: 2, 3, 4, 5, 6, 7, 8, 9, T, J, Q, K, A.
pub const STD_DECK_RANK_CHARS: &str = "23456789TJQKA";
/// Characters representing suits: h, d, c, s.
pub const STD_DECK_SUIT_CHARS: &str = "hdcs";

// Ranks
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
pub use crate::tables::t_cardmasks::STD_DECK_RANK_COUNT;
// Constants for first and last ranks
pub const STD_DECK_RANK_FIRST: usize = STD_DECK_RANK_2;
pub const STD_DECK_RANK_LAST: usize = STD_DECK_RANK_ACE;
// Suits
pub const STD_DECK_SUIT_HEARTS: usize = 0;
pub const STD_DECK_SUIT_DIAMONDS: usize = 1;
pub const STD_DECK_SUIT_CLUBS: usize = 2;
pub const STD_DECK_SUIT_SPADES: usize = 3;
pub use crate::tables::t_cardmasks::STD_DECK_SUIT_COUNT;
// Constants for first and last ranks
pub const STD_DECK_SUIT_FIRST: usize = STD_DECK_SUIT_HEARTS;
pub const STD_DECK_SUIT_LAST: usize = STD_DECK_SUIT_SPADES;

// N_RANKMASKS used for bitmask calculations
pub const STD_DECK_N_RANKMASKS: usize = 1 << STD_DECK_RANK_COUNT;

/// Newtype for Card Rank (0..12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rank(pub u8);

impl Rank {
    pub const TWO: Rank = Rank(0);
    pub const THREE: Rank = Rank(1);
    pub const FOUR: Rank = Rank(2);
    pub const FIVE: Rank = Rank(3);
    pub const SIX: Rank = Rank(4);
    pub const SEVEN: Rank = Rank(5);
    pub const EIGHT: Rank = Rank(6);
    pub const NINE: Rank = Rank(7);
    pub const TEN: Rank = Rank(8);
    pub const JACK: Rank = Rank(9);
    pub const QUEEN: Rank = Rank(10);
    pub const KING: Rank = Rank(11);
    pub const ACE: Rank = Rank(12);

    /// Total number of ranks (13).
    pub const COUNT: usize = STD_DECK_RANK_COUNT;

    /// Creates a new `Rank` from a u8 value.
    pub const fn new(v: u8) -> Self {
        Self(v)
    }

    /// Returns the rank as a u8.
    pub const fn as_u8(&self) -> u8 {
        self.0
    }

    /// Returns the rank as a usize.
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    /// Parses a Rank from a character (2-9, T, J, Q, K, A). Case-insensitive.
    pub fn from_char(c: char) -> Option<Self> {
        let idx = STD_DECK_RANK_CHARS.find(c.to_ascii_uppercase())?;
        Some(Rank(idx as u8))
    }
}

impl From<usize> for Rank {
    fn from(v: usize) -> Self {
        Rank(v as u8)
    }
}

impl From<Rank> for usize {
    fn from(r: Rank) -> Self {
        r.0 as usize
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = STD_DECK_RANK_CHARS
            .as_bytes()
            .get(self.0 as usize)
            .unwrap_or(&b'?');
        write!(f, "{}", *c as char)
    }
}

/// Newtype for Card Suit (0..3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Suit(pub u8);

impl Suit {
    pub const HEARTS: Suit = Suit(0);
    pub const DIAMONDS: Suit = Suit(1);
    pub const CLUBS: Suit = Suit(2);
    pub const SPADES: Suit = Suit(3);

    /// Total number of suits (4).
    pub const COUNT: usize = STD_DECK_SUIT_COUNT;

    /// Creates a new `Suit` from a u8 value.
    pub const fn new(v: u8) -> Self {
        Self(v)
    }

    /// Returns the suit as a u8.
    pub const fn as_u8(&self) -> u8 {
        self.0
    }

    /// Returns the suit as a usize.
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    /// Parses a Suit from a character (h, d, c, s). Case-insensitive.
    pub fn from_char(c: char) -> Option<Self> {
        let idx = STD_DECK_SUIT_CHARS.find(c.to_ascii_lowercase())?;
        Some(Suit(idx as u8))
    }
}

impl From<usize> for Suit {
    fn from(v: usize) -> Self {
        Suit(v as u8)
    }
}

impl From<Suit> for usize {
    fn from(s: Suit) -> Self {
        s.0 as usize
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = STD_DECK_SUIT_CHARS
            .as_bytes()
            .get(self.0 as usize)
            .unwrap_or(&b'?');
        write!(f, "{}", *c as char)
    }
}

impl Default for StdDeckCardMask {
    fn default() -> Self {
        Self::new()
    }
}

impl StdDeckCardMask {
    /// Converts a card mask into a string representing the cards (e.g., "As Kh").
    ///
    /// # Examples
    ///
    /// ```
    /// use poker_eval_rs::deck::StdDeck;
    /// let (mask, _) = StdDeck::string_to_mask("As Kh").unwrap();
    /// // Implementation order: Hearts(0..12), Diamonds(13..25), Clubs(26..38), Spades(39..51)
    /// assert_eq!(mask.mask_to_string(), "Kh As");
    /// ```
    pub fn mask_to_string(&self) -> String {
        let mut card_strings = Vec::new();

        for card_index in 0..STD_DECK_N_CARDS {
            if self.card_is_set(card_index) {
                // Use card_to_string to get the string representation of the card
                let card_str = StdDeck::card_to_string(card_index);
                card_strings.push(card_str);
            }
        }

        // Join the card strings with a space
        card_strings.join(" ")
    }

    /// Sets spade cards from a 13-bit rank mask.
    pub fn set_spades(&mut self, ranks: u16) {
        self.set_suite(ranks, 0);
    }

    /// Returns the 13-bit rank mask for spades.
    #[inline]
    pub const fn spades(&self) -> u16 {
        (self.as_raw() & 0x1FFF) as u16
    }

    /// Sets club cards from a 13-bit rank mask.
    pub fn set_clubs(&mut self, ranks: u16) {
        self.set_suite(ranks, 16);
    }

    /// Returns the 13-bit rank mask for clubs.
    #[inline]
    pub const fn clubs(&self) -> u16 {
        ((self.as_raw() >> 16) & 0x1FFF) as u16
    }

    /// Sets diamond cards from a 13-bit rank mask.
    pub fn set_diamonds(&mut self, ranks: u16) {
        self.set_suite(ranks, 32);
    }

    /// Returns the 13-bit rank mask for diamonds.
    #[inline]
    pub const fn diamonds(&self) -> u16 {
        ((self.as_raw() >> 32) & 0x1FFF) as u16
    }

    /// Sets heart cards from a 13-bit rank mask.
    pub fn set_hearts(&mut self, ranks: u16) {
        self.set_suite(ranks, 48);
    }

    /// Returns the 13-bit rank mask for hearts.
    #[inline]
    pub const fn hearts(&self) -> u16 {
        ((self.as_raw() >> 48) & 0x1FFF) as u16
    }

    /// Generic method to set a suit's ranks.
    pub fn set_suite(&mut self, ranks: u16, offset: usize) {
        let mask: u64 = (ranks as u64) << offset;
        self.set_raw(self.as_raw() | mask);
    }

    /// Generic method to get a suit's ranks.
    pub const fn get_suite(&self, offset: usize) -> u16 {
        ((self.as_raw() >> offset) & 0x1FFF) as u16
    }

    /// Computes the bitwise OR with another mask.
    pub fn or(&mut self, other: &StdDeckCardMask) {
        self.set_raw(self.as_raw() | other.as_raw());
    }

    /// Computes the bitwise AND with another mask.
    pub fn and(&mut self, other: &StdDeckCardMask) {
        self.set_raw(self.as_raw() & other.as_raw());
    }

    /// Computes the bitwise XOR with another mask.
    pub fn xor(&mut self, other: &StdDeckCardMask) {
        self.set_raw(self.as_raw() ^ other.as_raw());
    }

    /// Computes the bitwise output NOT of the mask.
    pub fn not(&mut self) {
        self.set_raw(!self.as_raw());
    }

    /// Returns a reference to the global mask table for a specific card index.
    pub fn get_mask(index: usize) -> &'static Self {
        &STD_DECK_CARD_MASKS_TABLE[index]
    }

    /// Checks if a specific card index is set in the mask.
    pub fn card_is_set(&self, index: usize) -> bool {
        if index >= STD_DECK_CARD_MASKS_TABLE.len() {
            return false;
        }
        let card_mask = STD_DECK_CARD_MASKS_TABLE[index];
        (self.as_raw() & card_mask.as_raw()) != 0
    }

    /// Resets the mask to empty (0).
    pub fn reset(&mut self) {
        self.set_raw(0);
    }

    /// Returns true if the mask is empty.
    pub fn is_empty(&self) -> bool {
        self.as_raw() == 0
    }

    /// Returns true if two masks are equal.
    pub fn equals(&self, other: &Self) -> bool {
        self.as_raw() == other.as_raw()
    }

    /// Returns the number of cards set in the mask.
    #[inline]
    pub fn num_cards(&self) -> usize {
        (0..STD_DECK_N_CARDS)
            .filter(|&i| self.card_is_set(i))
            .count()
    }

    /// Sets a specific card index in the mask.
    pub fn set(&mut self, card_index: usize) {
        if card_index < STD_DECK_CARD_MASKS_TABLE.len() {
            self.set_raw(self.as_raw() | STD_DECK_CARD_MASKS_TABLE[card_index].as_raw());
        }
    }
}

impl CardMask for StdDeckCardMask {
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

impl TryFrom<&str> for StdDeckCardMask {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (mask, _) = StdDeck::string_to_mask(value)?;
        Ok(mask)
    }
}

/// Standard Deck utilities.
pub struct StdDeck;

impl StdDeck {
    /// Converts a single card mask to a card index (0..51).
    pub fn mask_to_index(card_mask: &StdDeckCardMask) -> Option<usize> {
        for (index, &mask) in STD_DECK_CARD_MASKS_TABLE.iter().enumerate() {
            if mask.as_raw() == card_mask.as_raw() {
                return Some(index);
            }
        }
        None
    }
    /// Returns the rank of a card index.
    #[inline]
    pub const fn rank(index: usize) -> Rank {
        Rank((index % STD_DECK_RANK_COUNT) as u8)
    }

    /// Returns the suit of a card index.
    #[inline]
    pub const fn suit(index: usize) -> Suit {
        Suit((index / STD_DECK_RANK_COUNT) as u8)
    }

    /// Creates a card index from rank and suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker_eval_rs::deck::{StdDeck, Rank, Suit};
    /// let card_idx = StdDeck::make_card(Rank::ACE, Suit::SPADES);
    /// assert_eq!(StdDeck::card_to_string(card_idx), "As");
    /// ```
    #[inline]
    pub const fn make_card(rank: Rank, suit: Suit) -> usize {
        suit.as_usize() * STD_DECK_RANK_COUNT + rank.as_usize()
    }

    /// Converts a card index to a string (e.g., "Ah").
    pub fn card_to_string(card_index: usize) -> String {
        format!("{}{}", Self::rank(card_index), Self::suit(card_index))
    }

    /// Converts a string (e.g., "Ah") to a card index.
    pub fn string_to_card(in_string: &str) -> Option<usize> {
        if in_string.len() != 2 {
            return None;
        }

        let rank_char = in_string.chars().next()?;
        let suit_char = in_string.chars().nth(1)?;

        let rank = STD_DECK_RANK_CHARS.find(rank_char.to_ascii_uppercase())?;
        let suit = STD_DECK_SUIT_CHARS.find(suit_char.to_ascii_lowercase())?;

        Some(Self::make_card(Rank::from(rank), Suit::from(suit)))
    }
    /// Parses a string of cards (e.g., "Ah Kh") into a card mask.
    /// Returns the mask and the number of cards parsed.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker_eval_rs::deck::StdDeck;
    /// let (mask, count) = StdDeck::string_to_mask("Ah Ks").unwrap();
    /// assert_eq!(count, 2);
    /// assert_eq!(mask.mask_to_string(), "Ah Ks");
    /// ```
    pub fn string_to_mask(in_string: &str) -> Result<(StdDeckCardMask, usize), String> {
        let mut out_mask = StdDeckCardMask::new();
        let mut n = 0;

        let chars: Vec<char> = in_string.chars().filter(|c| !c.is_whitespace()).collect();
        for chunk in chars.chunks(2) {
            if chunk.len() != 2 {
                return Err(format!("Invalid card format: {:?}", chunk));
            }
            let (rank_char, suit_char) = (chunk[0], chunk[1]);

            let rank = STD_DECK_RANK_CHARS.find(rank_char.to_ascii_uppercase());
            let suit = STD_DECK_SUIT_CHARS.find(suit_char.to_ascii_lowercase());

            match (rank, suit) {
                (Some(rank), Some(suit)) => {
                    let card = Self::make_card(Rank::from(rank), Suit::from(suit));
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

impl Deck for StdDeck {
    type Mask = StdDeckCardMask;
    const N_CARDS: usize = STD_DECK_N_CARDS;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_is_set() {
        // Create a card mask with a specific card set.
        // For example, let's set the 3rd card (index 2 if starting from 0).
        let mut card_mask = StdDeckCardMask::new();
        let card_index_to_set = 2; // Index of the card you want to set.
        card_mask.set(card_index_to_set); // Use the `set` method to set the card.

        // Verify that `card_is_set` returns `true` for this card.
        assert!(
            card_mask.card_is_set(card_index_to_set),
            "Card at index {} should be set.",
            card_index_to_set
        );

        // Also verify that `card_is_set` returns `false` for an unset card.
        // For example, let's check the 10th card (index 9).
        assert!(
            !card_mask.card_is_set(9),
            "Card at index 9 should not be set."
        );
    }

    #[test]
    fn test_card_is_set_for_all_cards() {
        let mut card_mask = StdDeckCardMask::new();

        // Iterate through each possible card in the deck and set it in the mask
        for card_index in 0..STD_DECK_N_CARDS {
            // Reset the mask for each card to avoid interference
            card_mask.reset();

            // Set the current card in the mask
            card_mask.set(card_index);

            // Verify that `card_is_set` indicates this card is set
            assert!(
                card_mask.card_is_set(card_index),
                "Card at index {} should be set.",
                card_index
            );

            // Verify that `card_is_set` returns `false` for all other cards
            for other_card_index in 0..STD_DECK_N_CARDS {
                if other_card_index != card_index {
                    assert!(
                        !card_mask.card_is_set(other_card_index),
                        "Card at index {} should not be set.",
                        other_card_index
                    );
                }
            }
        }
    }

    #[test]
    fn test_card_to_string_for_all_cards() {
        // Define the expected representations for all cards in the deck
        // The card order and representation depend on how the `card_to_string` function is implemented
        // The following example assumes a suit order of hearts, diamonds, clubs, then spades, and a rank order of 2 to Ace
        let expected_card_strings = vec![
            "2h", "3h", "4h", "5h", "6h", "7h", "8h", "9h", "Th", "Jh", "Qh", "Kh", "Ah", "2d",
            "3d", "4d", "5d", "6d", "7d", "8d", "9d", "Td", "Jd", "Qd", "Kd", "Ad", "2c", "3c",
            "4c", "5c", "6c", "7c", "8c", "9c", "Tc", "Jc", "Qc", "Kc", "Ac", "2s", "3s", "4s",
            "5s", "6s", "7s", "8s", "9s", "Ts", "Js", "Qs", "Ks", "As",
        ];

        for (card_index, &expected_str) in expected_card_strings.iter().enumerate() {
            let card_str = StdDeck::card_to_string(card_index);
            println!(
                "Converting index {} should give '{}', but gave '{}'.",
                card_index, expected_str, card_str
            );
            assert_eq!(
                card_str, expected_str,
                "Converting index {} should give '{}', but gave '{}'.",
                card_index, expected_str, card_str
            );
        }
    }
    #[test]
    fn test_mask_to_index() {
        // Tests if `mask_to_index` correctly returns the index for each card mask in the table
        for (expected_index, card_mask) in STD_DECK_CARD_MASKS_TABLE.iter().enumerate() {
            let index = StdDeck::mask_to_index(card_mask);
            assert_eq!(
                index,
                Some(expected_index),
                "The index returned for mask {:?} should be {}, but was {:?}",
                card_mask,
                expected_index,
                index
            );
        }
    }
    #[test]
    fn test_mask_to_index_for_all_cards() {
        // Iterate through each card in the standard card masks table
        for (expected_index, &card_mask) in STD_DECK_CARD_MASKS_TABLE.iter().enumerate() {
            // Use the mask_to_index function to get the card index based on its mask
            let obtained_index = StdDeck::mask_to_index(&card_mask);
            println!(
                "Index obtained for card mask {} -> {:?}: {:?}",
                card_mask.mask_to_string(),
                card_mask,
                obtained_index
            );

            // Verify that the obtained index matches the expected index
            assert_eq!(
                obtained_index,
                Some(expected_index),
                "The index obtained for card mask {:?} should be {}, but was {:?}",
                card_mask,
                expected_index,
                obtained_index
            );
        }
    }

    #[test]
    fn test_or_operation() {
        let mut mask1 = StdDeckCardMask::new();
        mask1.set(STD_DECK_RANK_2); // suppose 2 of hearts

        let mut mask2 = StdDeckCardMask::new();
        mask2.set(STD_DECK_RANK_3); // suppose 3 of hearts

        mask1.or(&mask2);

        assert!(mask1.card_is_set(STD_DECK_RANK_2));
        assert!(mask1.card_is_set(STD_DECK_RANK_3));
    }

    #[test]
    fn test_and_operation() {
        let mut mask1 = StdDeckCardMask::new();
        mask1.set(STD_DECK_RANK_2); // suppose 2 of hearts

        let mut mask2 = StdDeckCardMask::new();
        mask2.set(STD_DECK_RANK_2); // suppose 2 of hearts

        mask1.and(&mask2);

        assert!(mask1.card_is_set(STD_DECK_RANK_2));
        assert!(!mask1.card_is_set(STD_DECK_RANK_3));
    }

    #[test]
    fn test_xor_operation() {
        let mut mask1 = StdDeckCardMask::new();
        mask1.set(STD_DECK_RANK_2); // suppose 2 of hearts

        let mut mask2 = StdDeckCardMask::new();
        mask2.set(STD_DECK_RANK_2); // suppose 2 of hearts
        mask2.set(STD_DECK_RANK_3); // suppose 3 of hearts

        mask1.xor(&mask2);

        assert!(!mask1.card_is_set(STD_DECK_RANK_2));
        assert!(mask1.card_is_set(STD_DECK_RANK_3));
    }

    #[test]
    fn test_not_operation() {
        let mut mask = StdDeckCardMask::new();
        mask.set(STD_DECK_RANK_2); // suppose 2 of hearts

        mask.not();

        assert!(!mask.card_is_set(STD_DECK_RANK_2));
        assert!(mask.card_is_set(STD_DECK_RANK_3)); // and all other cards except 2 of hearts
    }
    #[test]
    fn test_num_cards() {
        let mut mask = StdDeckCardMask::new();
        mask.set(STD_DECK_RANK_2); // suppose 2 of hearts
        mask.set(STD_DECK_RANK_3); // suppose 3 of hearts

        assert_eq!(mask.num_cards(), 2);
    }

    #[test]
    fn test_string_to_card() {
        let card_str = "Ah"; // Ace of Hearts
        let card_index = StdDeck::string_to_card(card_str).unwrap();

        assert_eq!(
            card_index,
            STD_DECK_RANK_ACE + STD_DECK_SUIT_HEARTS * STD_DECK_RANK_COUNT
        );
    }

    #[test]
    fn test_string_to_mask() {
        let cards_str = "AhKh";
        let (mask, count) = StdDeck::string_to_mask(cards_str).unwrap();

        assert!(mask.card_is_set(STD_DECK_RANK_ACE + STD_DECK_SUIT_HEARTS * STD_DECK_RANK_COUNT));
        assert!(mask.card_is_set(STD_DECK_RANK_KING + STD_DECK_SUIT_HEARTS * STD_DECK_RANK_COUNT));
        assert_eq!(count, 2);
    }

    #[test]
    fn test_set_spades_with_specific_card() {
        // Create an instance of the StdDeck_CardMask structure (or Rust equivalent).
        // Make sure to initialize the mask with an appropriate value.

        let mut card_mask = StdDeckCardMask::new();

        // Set a specific card as a spade using the appropriate rank and suit values.
        let specific_spade_rank = 5; // For example, 5 for the 5 of spades
        let specific_spade_mask = 1u64 << (specific_spade_rank + 39); // 39-bit shift for spades

        // Call the set_spades method with the specific value.
        card_mask.set_spades(specific_spade_mask as u16);

        // Verify that the mask was correctly updated to include only the specific spade card.
        println!("card_mask.spades(): {}", card_mask.spades());
        println!("specific_spade_mask: {}", specific_spade_mask);
        assert_eq!(card_mask.spades(), specific_spade_mask as u16);
    }

    #[test]
    fn test_reset_and_is_empty() {
        let mut card_mask = StdDeckCardMask::new();

        // Set some cards in the mask
        card_mask.set(STD_DECK_RANK_3 + STD_DECK_SUIT_HEARTS * STD_DECK_RANK_COUNT);
        card_mask.set(STD_DECK_RANK_9 + STD_DECK_SUIT_DIAMONDS * STD_DECK_RANK_COUNT);

        // Verify that the mask is not empty after setting cards
        assert!(!card_mask.is_empty());

        // Reset the mask
        card_mask.reset();

        // Verify that the mask is now empty
        assert!(card_mask.is_empty());
    }
    #[test]
    fn test_spades() {
        let mut mask = StdDeckCardMask::new();
        mask.set(StdDeck::make_card(Rank::ACE, Suit::SPADES)); // Set the Ace of spades
        println!("mask.spades(): {}", mask.mask_to_string());
        let spades = StdDeckCardMask::from_raw(1 << STD_DECK_RANK_ACE);
        println!("spades: {}", spades.mask_to_string());
        assert_eq!(
            mask,
            StdDeckCardMask::from_raw(1 << STD_DECK_RANK_ACE),
            "The spades mask is incorrect"
        );
    }

    #[test]
    fn test_all_spades() {
        let mut mask = StdDeckCardMask::new();

        // Set all spade cards
        for rank in 0..STD_DECK_RANK_COUNT {
            mask.set(StdDeck::make_card(Rank::from(rank), Suit::SPADES));
        }
        println!("mask to string: {}", mask.mask_to_string());
        println!("mask.mask: {:b}", mask.as_raw());
        println!("mask.spades(): {:b}", mask.spades());

        // Calculate the expected mask for all spades: 0b1111111111111 (13 bits set to 1)
        let expected_spades_mask = (1 << STD_DECK_RANK_COUNT) - 1;

        // Compare the obtained spades mask with the expected mask
        assert_eq!(
            mask.spades(),
            expected_spades_mask,
            "The mask for all spades is incorrect"
        );
    }

    #[test]
    fn test_all_clubs() {
        let mut mask = StdDeckCardMask::new();

        // Set all club cards
        for rank in 0..STD_DECK_RANK_COUNT {
            mask.set(StdDeck::make_card(Rank::from(rank), Suit::CLUBS));
        }
        println!("mask to string: {}", mask.mask_to_string());
        println!("mask.mask: {:b}", mask.as_raw());
        println!("mask.clubs(): {:b}", mask.clubs());

        // Calculate the expected mask for all clubs
        let expected_clubs_mask = (1 << STD_DECK_RANK_COUNT) - 1;

        // Compare the obtained clubs mask with the expected mask
        assert_eq!(
            mask.clubs(),
            expected_clubs_mask,
            "The mask for all clubs is incorrect"
        );
    }

    #[test]
    fn test_all_diamonds() {
        let mut mask = StdDeckCardMask::new();

        // Set all diamond cards
        for rank in 0..STD_DECK_RANK_COUNT {
            mask.set(StdDeck::make_card(Rank::from(rank), Suit::DIAMONDS));
        }
        println!("mask to string: {}", mask.mask_to_string());
        println!("mask.mask: {:b}", mask.as_raw());
        println!("mask.diamonds(): {:b}", mask.diamonds());

        // Calculate the expected mask for all diamonds
        let expected_diamonds_mask = (1 << STD_DECK_RANK_COUNT) - 1;

        // Compare the obtained diamonds mask with the expected mask
        assert_eq!(
            mask.diamonds(),
            expected_diamonds_mask,
            "The mask for all diamonds is incorrect"
        );
    }

    #[test]
    fn test_all_hearts() {
        let mut mask = StdDeckCardMask::new();

        // Set all heart cards
        for rank in 0..STD_DECK_RANK_COUNT {
            mask.set(StdDeck::make_card(Rank::from(rank), Suit::HEARTS));
        }
        println!("mask to string: {}", mask.mask_to_string());
        println!("mask.mask: {:b}", mask.as_raw());
        println!("mask.hearts(): {:b}", mask.hearts());

        // Calculate the expected mask for all hearts
        let expected_hearts_mask = (1 << STD_DECK_RANK_COUNT) - 1;

        // Compare the obtained hearts mask with the expected mask
        assert_eq!(
            mask.hearts(),
            expected_hearts_mask,
            "The mask for all hearts is incorrect"
        );
    }
}
