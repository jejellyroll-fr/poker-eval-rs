pub const JOKER_DECK_N_CARDS: usize = 53;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JokerDeckCardMask {
    pub cards_n: u64,
}

pub const JOKER_DECK_CARD_MASKS_TABLE: [JokerDeckCardMask; JOKER_DECK_N_CARDS] = [
    JokerDeckCardMask {
        cards_n: 0x0001000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0002000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0004000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0008000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0010000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0020000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0040000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0080000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0100000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0200000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0400000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0800000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x1000000000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000100000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000200000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000400000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000800000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000001000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000002000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000004000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000008000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000010000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000020000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000040000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000080000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000100000000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000010000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000020000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000040000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000080000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000100000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000200000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000400000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000800000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000001000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000002000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000004000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000008000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000010000000,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000001,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000002,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000004,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000008,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000010,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000020,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000040,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000080,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000100,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000200,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000400,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000000800,
    },
    JokerDeckCardMask {
        cards_n: 0x0000000000001000,
    },
    JokerDeckCardMask {
        cards_n: 0x2000000000000000,
    },
];
