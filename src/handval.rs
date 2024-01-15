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

#[derive(Debug,Copy, Clone)]
pub struct HandVal {
    pub value: u32,
}

impl HandVal {
    pub fn new(hand_type: u8, top: u8, second: u8, third: u8, fourth: u8, fifth: u8) -> Self {
        let mut value = ((hand_type as u32) << HANDTYPE_SHIFT) & HANDTYPE_MASK;
        value |= ((top as u32) << TOP_CARD_SHIFT) & TOP_CARD_MASK;
        value |= ((second as u32) << SECOND_CARD_SHIFT) & SECOND_CARD_MASK;
        value |= ((third as u32) << THIRD_CARD_SHIFT) & THIRD_CARD_MASK;
        value |= ((fourth as u32) << FOURTH_CARD_SHIFT) & FOURTH_CARD_MASK;
        value |= ((fifth as u32) << FIFTH_CARD_SHIFT) & FIFTH_CARD_MASK;

        HandVal { value }
    }

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

    // ... autres méthodes, y compris pour extraire les cartes ...
}
