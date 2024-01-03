pub struct HandVal {
    value: u32,
}

impl HandVal {
    // Constantes pour les décalages et masques
    const HANDTYPE_SHIFT: u32 = 24;
    const HANDTYPE_MASK: u32 = 0x0F000000;
    const TOP_CARD_SHIFT: u32 = 20;
    const SECOND_CARD_SHIFT: u32 = 16;
    const THIRD_CARD_SHIFT: u32 = 12;
    const FOURTH_CARD_SHIFT: u32 = 8;
    const FIFTH_CARD_SHIFT: u32 = 4;
    const CARD_MASK: u32 = 0x0F;

    pub fn new(hand_type: u8, top: u8, second: u8, third: u8, fourth: u8, fifth: u8) -> Self {
        let mut value = (hand_type as u32) << Self::HANDTYPE_SHIFT;
        value |= ((top as u32) << Self::TOP_CARD_SHIFT) & (Self::CARD_MASK << Self::TOP_CARD_SHIFT);
        value |= ((second as u32) << Self::SECOND_CARD_SHIFT) & (Self::CARD_MASK << Self::SECOND_CARD_SHIFT);
        value |= ((third as u32) << Self::THIRD_CARD_SHIFT) & (Self::CARD_MASK << Self::THIRD_CARD_SHIFT);
        value |= ((fourth as u32) << Self::FOURTH_CARD_SHIFT) & (Self::CARD_MASK << Self::FOURTH_CARD_SHIFT);
        value |= ((fifth as u32) << Self::FIFTH_CARD_SHIFT) & (Self::CARD_MASK << Self::FIFTH_CARD_SHIFT);

        HandVal { value }
    }

    // ... autres méthodes, y compris pour extraire les cartes ...



    // Accesseurs pour obtenir les différentes parties de la valeur
    pub fn hand_type(&self) -> u8 {
        ((self.value & Self::HANDTYPE_MASK) >> Self::HANDTYPE_SHIFT) as u8
    }

    pub fn top_card(&self) -> u8 {
        ((self.value >> Self::TOP_CARD_SHIFT) & Self::CARD_MASK) as u8
    }

    pub fn second_card(&self) -> u8 {
        ((self.value >> Self::SECOND_CARD_SHIFT) & Self::CARD_MASK) as u8
    }

    pub fn third_card(&self) -> u8 {
        ((self.value >> Self::THIRD_CARD_SHIFT) & Self::CARD_MASK) as u8
    }

    pub fn fourth_card(&self) -> u8 {
        ((self.value >> Self::FOURTH_CARD_SHIFT) & Self::CARD_MASK) as u8
    }

    pub fn fifth_card(&self) -> u8 {
        ((self.value >> Self::FIFTH_CARD_SHIFT) & Self::CARD_MASK) as u8
    }
}
