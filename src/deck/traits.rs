//! Traits for universal deck and card mask support.

use std::fmt::Debug;

/// Common interface for card masks across different deck sizes.
pub trait CardMask: Clone + Copy + Debug + PartialEq + Eq + Default {
    /// Creates an empty mask.
    fn new() -> Self;

    /// Returns true if the mask is empty.
    fn is_empty(&self) -> bool;

    /// Performs bitwise AND with another mask.
    fn and(&mut self, other: &Self);

    /// Performs bitwise OR with another mask.
    fn or(&mut self, other: &Self);

    /// Performs bitwise XOR with another mask.
    fn xor(&mut self, other: &Self);

    /// Performs bitwise NOT on the mask.
    fn not(&mut self);

    /// Returns the number of cards set in the mask.
    fn num_cards(&self) -> usize;

    /// Checks if a specific card index is set.
    fn card_is_set(&self, index: usize) -> bool;

    /// Sets a specific card index in the mask.
    fn set(&mut self, index: usize);

    /// Converts the mask to a string representation.
    fn mask_to_string(&self) -> String;
}

/// Interface for different poker decks.
pub trait Deck {
    /// The type of card mask used by this deck.
    type Mask: CardMask;

    /// Total number of cards in the deck.
    const N_CARDS: usize;

    /// Converts a card index to its string representation (e.g., "Ah").
    fn card_to_string(card_index: usize) -> String;

    /// Converts a string representation to a card index.
    fn string_to_card(card_str: &str) -> Option<usize>;

    /// Parses a string of cards into a card mask.
    fn string_to_mask(cards_str: &str) -> Result<(Self::Mask, usize), String>;
}
