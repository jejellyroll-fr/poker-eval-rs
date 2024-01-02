const STRING_CARDS: usize = 100;

pub struct Deck {
    pub n_cards: i32,
    pub name: String,
    pub card_to_string: Box<dyn Fn(i32) -> String>,
    pub string_to_card: Box<dyn Fn(&str) -> i32>,
    pub mask_to_cards: Box<dyn Fn(&CardMask, &mut Vec<i32>) -> i32>,
    pub num_cards: Box<dyn Fn(&CardMask) -> i32>,
}

pub struct CardMask {
    pub mask: u64, // Utilisation d'un u64 pour couvrir tous les cas (jusqu'à 64 bits)
}

impl Deck {
    // Correspond à GenericDeck_maskToString 
    pub fn generic_deck_mask_to_string(&self, card_mask: &CardMask) -> String {
        let mut cards = vec![0; STRING_CARDS];
        let n = (self.mask_to_cards)(card_mask, &mut cards); 
        let mut out_string = String::new();

        for i in 0..n {
            if i > 0 {
                out_string.push(' ');
            }
            // Conversion de i en usize pour l'indexation
            out_string.push_str(&self.generic_deck_card_to_string(cards[i as usize]));
        }
        out_string
    }

    // Correspond à GenericDeck_printMask 
    pub fn generic_deck_print_mask(&self, card_mask: &CardMask) {
        let out_string = self.generic_deck_mask_to_string(card_mask);
        println!("{}", out_string);
    }

    // Correspond à GenericDeck_maskString 
    pub fn generic_deck_mask_string(&self, card_mask: &CardMask) -> String {
        self.generic_deck_mask_to_string(card_mask)
    }

    // Correspond à GenericDeck_numCards 
    pub fn generic_deck_num_cards(&self, card_mask: &CardMask) -> i32 {
        (self.num_cards)(card_mask)
    }

    // Correspond à GenericDeck_cardString 
    pub fn generic_deck_card_to_string(&self, card_index: i32) -> String {
        // Votre logique de conversion ici
        format!("Card {}", card_index)
    }

    // Correspond à GenericDeck_printCard 
    pub fn generic_deck_print_card(&self, card_index: i32) {
        let out_string = self.generic_deck_card_to_string(card_index);
        if !out_string.is_empty() {
            println!("{}", out_string);
        }
    }
}