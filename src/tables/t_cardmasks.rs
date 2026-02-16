use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Copy, Debug, Hash, Eq, Serialize, Deserialize)]
pub struct StdDeckCardMask {
    mask: u64,
}
pub const STD_DECK_N_CARDS: usize = 52;
pub const STD_DECK_RANK_COUNT: usize = 13;
pub const STD_DECK_SUIT_COUNT: usize = 4;

impl StdDeckCardMask {
    pub const fn new() -> Self {
        StdDeckCardMask { mask: 0 }
    }

    /// Creates a card mask from a raw 64-bit integer.
    pub const fn from_raw(mask: u64) -> Self {
        StdDeckCardMask { mask }
    }

    /// Returns the raw 64-bit integer mask.
    pub const fn as_raw(&self) -> u64 {
        self.mask
    }

    /// Sets the raw 64-bit integer mask.
    pub fn set_raw(&mut self, mask: u64) {
        self.mask = mask;
    }

    /// Returns a mask with all 52 cards set.
    pub const fn all_cards() -> Self {
        StdDeckCardMask {
            mask: 0x000FFFFFFFFFFFFF,
        } // 52 bits set
    }

    /// Creates a mask with a single card set by index (0-51).
    pub fn from_card_index(index: usize) -> Self {
        if index < STD_DECK_N_CARDS {
            STD_DECK_CARD_MASKS_TABLE[index]
        } else {
            Self::new()
        }
    }
}

impl std::fmt::Display for StdDeckCardMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for i in 0..STD_DECK_N_CARDS {
            if self.card_is_set(i) {
                if !first {
                    write!(f, " ")?;
                }
                let rank = i % STD_DECK_RANK_COUNT;
                let suit = i / STD_DECK_RANK_COUNT;
                let rank_char = match rank {
                    0 => '2',
                    1 => '3',
                    2 => '4',
                    3 => '5',
                    4 => '6',
                    5 => '7',
                    6 => '8',
                    7 => '9',
                    8 => 'T',
                    9 => 'J',
                    10 => 'Q',
                    11 => 'K',
                    12 => 'A',
                    _ => '?',
                };
                let suit_char = match suit {
                    0 => 'h',
                    1 => 'd',
                    2 => 'c',
                    3 => 's',
                    _ => '?',
                };
                write!(f, "{}{}", rank_char, suit_char)?;
                first = false;
            }
        }
        Ok(())
    }
}
// Implementation of the StdDeck_cardMasksTable in Rust
pub const STD_DECK_CARD_MASKS_TABLE: [StdDeckCardMask; STD_DECK_N_CARDS] = [
    StdDeckCardMask {
        mask: 0x0001000000000000,
    },
    StdDeckCardMask {
        mask: 0x0002000000000000,
    },
    StdDeckCardMask {
        mask: 0x0004000000000000,
    },
    StdDeckCardMask {
        mask: 0x0008000000000000,
    },
    StdDeckCardMask {
        mask: 0x0010000000000000,
    },
    StdDeckCardMask {
        mask: 0x0020000000000000,
    },
    StdDeckCardMask {
        mask: 0x0040000000000000,
    },
    StdDeckCardMask {
        mask: 0x0080000000000000,
    },
    StdDeckCardMask {
        mask: 0x0100000000000000,
    },
    StdDeckCardMask {
        mask: 0x0200000000000000,
    },
    StdDeckCardMask {
        mask: 0x0400000000000000,
    },
    StdDeckCardMask {
        mask: 0x0800000000000000,
    },
    StdDeckCardMask {
        mask: 0x1000000000000000,
    },
    StdDeckCardMask {
        mask: 0x0000000100000000,
    },
    StdDeckCardMask {
        mask: 0x0000000200000000,
    },
    StdDeckCardMask {
        mask: 0x0000000400000000,
    },
    StdDeckCardMask {
        mask: 0x0000000800000000,
    },
    StdDeckCardMask {
        mask: 0x0000001000000000,
    },
    StdDeckCardMask {
        mask: 0x0000002000000000,
    },
    StdDeckCardMask {
        mask: 0x0000004000000000,
    },
    StdDeckCardMask {
        mask: 0x0000008000000000,
    },
    StdDeckCardMask {
        mask: 0x0000010000000000,
    },
    StdDeckCardMask {
        mask: 0x0000020000000000,
    },
    StdDeckCardMask {
        mask: 0x0000040000000000,
    },
    StdDeckCardMask {
        mask: 0x0000080000000000,
    },
    StdDeckCardMask {
        mask: 0x0000100000000000,
    },
    StdDeckCardMask {
        mask: 0x0000000000010000,
    },
    StdDeckCardMask {
        mask: 0x0000000000020000,
    },
    StdDeckCardMask {
        mask: 0x0000000000040000,
    },
    StdDeckCardMask {
        mask: 0x0000000000080000,
    },
    StdDeckCardMask {
        mask: 0x0000000000100000,
    },
    StdDeckCardMask {
        mask: 0x0000000000200000,
    },
    StdDeckCardMask {
        mask: 0x0000000000400000,
    },
    StdDeckCardMask {
        mask: 0x0000000000800000,
    },
    StdDeckCardMask {
        mask: 0x0000000001000000,
    },
    StdDeckCardMask {
        mask: 0x0000000002000000,
    },
    StdDeckCardMask {
        mask: 0x0000000004000000,
    },
    StdDeckCardMask {
        mask: 0x0000000008000000,
    },
    StdDeckCardMask {
        mask: 0x0000000010000000,
    },
    StdDeckCardMask {
        mask: 0x0000000000000001,
    },
    StdDeckCardMask {
        mask: 0x0000000000000002,
    },
    StdDeckCardMask {
        mask: 0x0000000000000004,
    },
    StdDeckCardMask {
        mask: 0x0000000000000008,
    },
    StdDeckCardMask {
        mask: 0x0000000000000010,
    },
    StdDeckCardMask {
        mask: 0x0000000000000020,
    },
    StdDeckCardMask {
        mask: 0x0000000000000040,
    },
    StdDeckCardMask {
        mask: 0x0000000000000080,
    },
    StdDeckCardMask {
        mask: 0x0000000000000100,
    },
    StdDeckCardMask {
        mask: 0x0000000000000200,
    },
    StdDeckCardMask {
        mask: 0x0000000000000400,
    },
    StdDeckCardMask {
        mask: 0x0000000000000800,
    },
    StdDeckCardMask {
        mask: 0x0000000000001000,
    },
];

// ...
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_deck_card_masks_table() {
        // Test to ensure each mask in the table is unique
        let mut mask_set = std::collections::HashSet::new();
        for mask in STD_DECK_CARD_MASKS_TABLE.iter() {
            assert!(mask_set.insert(mask.mask));
        }
        assert_eq!(mask_set.len(), STD_DECK_CARD_MASKS_TABLE.len());
        println!("{} {}", mask_set.len(), STD_DECK_CARD_MASKS_TABLE.len());
    }

    // You can add additional tests here if needed
    #[test]
    fn test_display_trait() {
        let mut mask = StdDeckCardMask::new();
        mask.set(
            crate::deck::STD_DECK_RANK_ACE
                + crate::deck::STD_DECK_SUIT_SPADES * crate::deck::STD_DECK_RANK_COUNT,
        ); // As
        mask.set(
            crate::deck::STD_DECK_RANK_KING
                + crate::deck::STD_DECK_SUIT_HEARTS * crate::deck::STD_DECK_RANK_COUNT,
        ); // Kh

        // The order depends on iteration (0..52).
        // Kh is rank 11, suit 0 -> index 11.
        // As is rank 12, suit 3 -> index 51.
        // Loop 0..52 hits Kh (11) then As (51).
        // Expected output: "Kh As"

        assert_eq!(mask.to_string(), "Kh As");
    }
}
