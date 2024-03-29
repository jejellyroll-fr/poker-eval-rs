#[derive(Clone, PartialEq, Copy, Debug, Hash, Eq)]
pub struct StdDeckCardMask {
    pub mask: u64,
}
// Implémentation de la table StdDeck_cardMasksTable en Rust
pub const STD_DECK_CARD_MASKS_TABLE: [StdDeckCardMask; 52] = [
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
        // Test pour s'assurer que chaque masque dans la table est unique
        let mut mask_set = std::collections::HashSet::new();
        for mask in STD_DECK_CARD_MASKS_TABLE.iter() {
            assert!(mask_set.insert(mask.mask));
        }
        assert_eq!(mask_set.len(), STD_DECK_CARD_MASKS_TABLE.len());
        println!("{} {}", mask_set.len(), STD_DECK_CARD_MASKS_TABLE.len());
    }

    // Vous pouvez ajouter ici des tests supplémentaires si nécessaire
    // test chaque masque en les convertissant en cartes
}
