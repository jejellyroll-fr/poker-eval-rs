use crate::t_cardmasks::{StdDeckCardMask, STD_DECK_CARD_MASKS_TABLE};

// Constantes
pub const STD_DECK_N_CARDS: usize = 52;
pub const STD_DECK_RANK_CHARS: &str = "23456789TJQKA";
pub const STD_DECK_SUIT_CHARS: &str = "hdcs";

// Rangs
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
pub const STD_DECK_RANK_COUNT: usize = 13;
// Constantes pour les premiers et derniers rangs
pub const STD_DECK_RANK_FIRST: usize = STD_DECK_RANK_2;
pub const STD_DECK_RANK_LAST: usize = STD_DECK_RANK_ACE;
// Couleurs
pub const STD_DECK_SUIT_HEARTS: usize = 0;
pub const STD_DECK_SUIT_DIAMONDS: usize = 1;
pub const STD_DECK_SUIT_CLUBS: usize = 2;
pub const STD_DECK_SUIT_SPADES: usize = 3;
pub const STD_DECK_SUIT_COUNT: usize = 4;
// Constantes pour les premiers et derniers rangs
pub const STD_DECK_SUIT_FIRST: usize = STD_DECK_SUIT_HEARTS;
pub const STD_DECK_SUIT_LAST: usize = STD_DECK_SUIT_SPADES;

// N_RANKMASKS utilisé pour les calculs de masque de bit
pub const STD_DECK_N_RANKMASKS: usize = 1 << STD_DECK_RANK_COUNT;

pub const STD_DECK_CARDMASK_SPADES: u64 = 0x1fff; // 13 bits pour les piques
pub const STD_DECK_CARDMASK_CLUBS: u64 = 0x1fff << 16; // 13 bits pour les trèfles
pub const STD_DECK_CARDMASK_DIAMONDS: u64 = 0x1fff << 32; // 13 bits pour les carreaux
pub const STD_DECK_CARDMASK_HEARTS: u64 = 0x1fff << 48; // 13 bits pour les cœurs

impl StdDeckCardMask {
    pub fn new() -> Self {
        StdDeckCardMask { mask: 0 }
    }

    // Convertit un masque de cartes en une chaîne de caractères représentant les cartes
    pub fn mask_to_string(&self) -> String {
        let mut card_strings = Vec::new();

        for card_index in 0..STD_DECK_N_CARDS {
            if self.card_is_set(card_index) {
                // Utilisez card_to_string pour obtenir la représentation en chaîne de la carte
                let card_str = StdDeck::card_to_string(card_index);
                card_strings.push(card_str);
            }
        }

        // Joignez les chaînes de cartes avec un espace
        card_strings.join(" ")
    }

    // Méthodes pour définir et obtenir les suites avec gestion de l'endianness et du padding
    pub fn set_spades(&mut self, ranks: u16) {
        let offset = if cfg!(target_endian = "big") {
            3
        } else {
            4 * 13 + 3 * 3
        };
        self.set_suite(ranks, offset);
    }

    pub fn spades(&self) -> u16 {
        (self.mask & 0x1FFF) as u16
    }

    // Méthodes pour les trefles
    pub fn set_clubs(&mut self, ranks: u16) {
        let offset = if cfg!(target_endian = "big") {
            19
        } else {
            3 * 13 + 2 * 3
        };
        self.set_suite(ranks, offset);
    }

    pub fn clubs(&self) -> u16 {
        ((self.mask >> 16) & 0x1FFF) as u16
    }

    // Méthodes pour les carreaux
    pub fn set_diamonds(&mut self, ranks: u16) {
        let offset = if cfg!(target_endian = "big") {
            35
        } else {
            2 * 13 + 1 * 3
        };
        self.set_suite(ranks, offset);
    }

    pub fn diamonds(&self) -> u16 {
        // Les carreaux suivent les clubs et un padding de 3 bits, donc un décalage total de 32 bits
        ((self.mask >> 32) & 0x1FFF) as u16
    }

    // Méthodes pour les trèfles
    pub fn set_hearts(&mut self, ranks: u16) {
        let offset = if cfg!(target_endian = "big") {
            51
        } else {
            1 * 13
        };
        self.set_suite(ranks, offset);
    }

    pub fn hearts(&self) -> u16 {
        // Les cœurs suivent les carreaux et un padding de 3 bits, donc un décalage total de 48 bits
        ((self.mask >> 48) & 0x1FFF) as u16
    }

    // Méthodes génériques pour définir et obtenir une suite
    pub fn set_suite(&mut self, ranks: u16, offset: usize) {
        let mask: u64 = (ranks as u64) << offset;
        self.mask |= mask;
    }

    pub fn get_suite(&self, offset: usize) -> u16 {
        ((self.mask >> offset) & 0x1FFF) as u16
    }

    // Autres opérations sur les masques (exemple: OR, AND, etc.)
    pub fn or(&mut self, other: &StdDeckCardMask) {
        self.mask |= other.mask;
    }

    pub fn and(&mut self, other: &StdDeckCardMask) {
        self.mask &= other.mask;
    }

    pub fn xor(&mut self, other: &StdDeckCardMask) {
        self.mask ^= other.mask;
    }

    pub fn not(&mut self) {
        self.mask = !self.mask;
    }

    // Méthode pour obtenir le masque d'une carte
    pub fn get_mask(index: usize) -> &'static Self {
        &STD_DECK_CARD_MASKS_TABLE[index]
    }

    // Méthode pour vérifier si une carte est présente dans le masque
    pub fn card_is_set(&self, index: usize) -> bool {
        if index >= STD_DECK_CARD_MASKS_TABLE.len() {
            return false;
        }
        let card_mask = STD_DECK_CARD_MASKS_TABLE[index].mask;
        self.mask & card_mask != 0
    }

    // Méthode pour réinitialiser le masque
    pub fn reset(&mut self) {
        self.mask = 0;
    }

    // Méthode pour vérifier si le masque est vide
    pub fn is_empty(&self) -> bool {
        self.mask == 0
    }

    // Méthode pour vérifier si deux masques sont égaux
    pub fn equals(&self, other: &Self) -> bool {
        self.mask == other.mask
    }

    // Méthode pour compter le nombre de cartes dans un masque
    pub fn num_cards(&self) -> usize {
        (0..STD_DECK_N_CARDS)
            .filter(|&i| self.card_is_set(i))
            .count()
    }

    // Méthode pour ajouter une carte au masque
    pub fn set(&mut self, card_index: usize) {
        if card_index < STD_DECK_CARD_MASKS_TABLE.len() {
            self.mask |= STD_DECK_CARD_MASKS_TABLE[card_index].mask;
        }
    }
}

// Structure StdDeck
pub struct StdDeck;

impl StdDeck {
    // Convertit un masque de carte unique en un index de carte
    pub fn mask_to_index(card_mask: &StdDeckCardMask) -> Option<usize> {
        for (index, &mask) in STD_DECK_CARD_MASKS_TABLE.iter().enumerate() {
            if mask.mask == card_mask.mask {
                return Some(index);
            }
        }
        None
    }
    // Méthode pour obtenir le rang d'une carte
    pub fn rank(index: usize) -> usize {
        index % STD_DECK_RANK_COUNT
    }

    // Méthode pour obtenir la couleur d'une carte
    pub fn suit(index: usize) -> usize {
        index / STD_DECK_RANK_COUNT
    }

    // Méthode pour créer une carte
    pub fn make_card(rank: usize, suit: usize) -> usize {
        suit * STD_DECK_RANK_COUNT + rank
    }

    // Conversion d'une carte en chaîne de caractères
    pub fn card_to_string(card_index: usize) -> String {
        let rank_char = STD_DECK_RANK_CHARS
            .chars()
            .nth(Self::rank(card_index))
            .unwrap();
        let suit_char = STD_DECK_SUIT_CHARS
            .chars()
            .nth(Self::suit(card_index))
            .unwrap();
        format!("{}{}", rank_char, suit_char)
    }

    // Conversion d'une chaîne de caractères en carte
    pub fn string_to_card(in_string: &str) -> Option<usize> {
        if in_string.len() != 2 {
            return None;
        }

        let rank_char = in_string.chars().next()?;
        let suit_char = in_string.chars().nth(1)?;
        //println!("Carte convertie en chaîne de caractères: {}{}", rank_char, suit_char);

        let rank = STD_DECK_RANK_CHARS.find(rank_char.to_ascii_uppercase())?;
        let suit = STD_DECK_SUIT_CHARS.find(suit_char.to_ascii_lowercase())?;
        //println!("Conversion de chaîne en carte: Rang = {}, Couleur = {}", rank_char, suit_char);

        Some(Self::make_card(rank, suit))
    }
    // Conversion d'une chaîne de caractères représentant des cartes en un masque de cartes
    pub fn string_to_mask(in_string: &str) -> Result<(StdDeckCardMask, usize), String> {
        let mut out_mask = StdDeckCardMask::new();
        let mut n = 0;

        for chunk in in_string.chars().collect::<Vec<char>>().chunks(2) {
            if chunk.len() != 2 {
                return Err(format!("Format de carte invalide : {:?}", chunk));
            }
            let (rank_char, suit_char) = (chunk[0], chunk[1]);

            if rank_char == ' ' {
                continue;
            }

            let rank = STD_DECK_RANK_CHARS.find(rank_char.to_ascii_uppercase());
            let suit = STD_DECK_SUIT_CHARS.find(suit_char.to_ascii_lowercase());

            match (rank, suit) {
                (Some(rank), Some(suit)) => {
                    let card = Self::make_card(rank, suit);
                    out_mask.set(card);
                    n += 1;
                }
                _ => {
                    return Err(format!(
                        "Caractère de carte non reconnu : {}{}",
                        rank_char, suit_char
                    ))
                }
            }
        }

        //println!("Masque de cartes généré : {:b}", out_mask.mask); //debug

        Ok((out_mask, n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_is_set() {
        // Créez un masque de carte avec une carte spécifique définie.
        // Par exemple, définissons la 3ème carte (index 2 si on commence à 0).
        let mut card_mask = StdDeckCardMask::new();
        let card_index_to_set = 2; // Index de la carte que vous voulez définir.
        card_mask.set(card_index_to_set); // Utilisez la méthode `set` pour définir la carte.

        // Vérifiez que `card_is_set` retourne `true` pour cette carte.
        assert!(
            card_mask.card_is_set(card_index_to_set),
            "La carte à l'index {} devrait être définie.",
            card_index_to_set
        );

        // Vérifiez également que `card_is_set` retourne `false` pour une carte non définie.
        // Par exemple, vérifions la 10ème carte (index 9).
        assert!(
            !card_mask.card_is_set(9),
            "La carte à l'index 9 ne devrait pas être définie."
        );
    }

    #[test]
    fn test_card_is_set_for_all_cards() {
        let mut card_mask = StdDeckCardMask::new();

        // Parcourez chaque carte possible dans le deck et définissez-la dans le masque
        for card_index in 0..STD_DECK_N_CARDS {
            // Réinitialisez le masque pour chaque carte pour éviter les interférences
            card_mask.reset();

            // Définissez la carte actuelle dans le masque
            card_mask.set(card_index);
            //println!("Masque pour la carte {}: {:b}", card_index, card_mask.mask);

            // Vérifiez que `card_is_set` indique que cette carte est bien définie
            assert!(
                card_mask.card_is_set(card_index),
                "La carte à l'index {} devrait être définie.",
                card_index
            );
            //println!("La carte à l'index {} est définie.", card_index);

            // Vérifiez que `card_is_set` retourne `false` pour toutes les autres cartes
            for other_card_index in 0..STD_DECK_N_CARDS {
                if other_card_index != card_index {
                    assert!(
                        !card_mask.card_is_set(other_card_index),
                        "La carte à l'index {} ne devrait pas être définie.",
                        other_card_index
                    );
                }
            }
        }
    }

    #[test]
    fn test_card_to_string_for_all_cards() {
        // Définissez les représentations attendues pour toutes les cartes dans le deck
        // L'ordre des cartes et leur représentation dépendent de la manière dont vous avez implémenté la fonction `card_to_string`
        // L'exemple suivant suppose un ordre de couleurs cœurs, diamants, trèfles, puis piques, et un ordre de valeurs de 2 à As
        let expected_card_strings = vec![
            "2h", "3h", "4h", "5h", "6h", "7h", "8h", "9h", "Th", "Jh", "Qh", "Kh", "Ah", "2d",
            "3d", "4d", "5d", "6d", "7d", "8d", "9d", "Td", "Jd", "Qd", "Kd", "Ad", "2c", "3c",
            "4c", "5c", "6c", "7c", "8c", "9c", "Tc", "Jc", "Qc", "Kc", "Ac", "2s", "3s", "4s",
            "5s", "6s", "7s", "8s", "9s", "Ts", "Js", "Qs", "Ks", "As",
        ];

        for (card_index, &expected_str) in expected_card_strings.iter().enumerate() {
            let card_str = StdDeck::card_to_string(card_index);
            println!(
                "La conversion de l'indice {} devrait donner '{}', mais a donné '{}'.",
                card_index, expected_str, card_str
            );
            assert_eq!(
                card_str, expected_str,
                "La conversion de l'indice {} devrait donner '{}', mais a donné '{}'.",
                card_index, expected_str, card_str
            );
        }
    }
    #[test]
    fn test_mask_to_index() {
        // Teste si `mask_to_index` renvoie correctement l'index pour chaque masque de carte dans la table
        for (expected_index, card_mask) in STD_DECK_CARD_MASKS_TABLE.iter().enumerate() {
            let index = StdDeck::mask_to_index(card_mask);
            assert_eq!(
                index,
                Some(expected_index),
                "L'index retourné pour le masque {:?} devrait être {}, mais était {:?}",
                card_mask,
                expected_index,
                index
            );
        }
    }
    #[test]
    fn test_mask_to_index_for_all_cards() {
        // Parcourez chaque carte dans la table des masques de cartes standard
        for (expected_index, &card_mask) in STD_DECK_CARD_MASKS_TABLE.iter().enumerate() {
            // Utilisez la fonction mask_to_index pour obtenir l'index de la carte basé sur son masque
            let obtained_index = StdDeck::mask_to_index(&card_mask);
            println!(
                "Index obtenu pour le masque de la carte {} -> {:?}: {:?}",
                card_mask.mask_to_string(),
                card_mask,
                obtained_index
            );

            // Vérifiez que l'index obtenu correspond à l'index attendu
            assert_eq!(
                obtained_index,
                Some(expected_index),
                "L'index obtenu pour le masque de la carte {:?} devrait être {}, mais était {:?}",
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
        // Créez une instance de votre structure StdDeck_CardMask (ou équivalente en Rust).
        // Assurez-vous d'initialiser le masque avec une valeur appropriée.

        let mut card_mask = StdDeckCardMask::new();

        // Définissez une carte spécifique comme un pique en utilisant les valeurs de rang et de suite appropriées.
        let specific_spade_rank = 5; // Par exemple, 5 pour le 5 de pique
        let specific_spade_mask = 1u64 << (specific_spade_rank + 39); // Décalage de 39 bits pour les piques

        // Appelez la méthode set_spades avec la valeur spécifique.
        card_mask.set_spades(specific_spade_mask as u16);

        // Vérifiez que le masque a été correctement mis à jour pour inclure uniquement la carte spécifique des piques.
        println!("card_mask.spades(): {}", card_mask.spades());
        println!("specific_spade_mask: {}", specific_spade_mask);
        assert_eq!(card_mask.spades(), specific_spade_mask as u16);
    }

    #[test]
    fn test_reset_and_is_empty() {
        let mut card_mask = StdDeckCardMask::new();

        // Définissez quelques cartes dans le masque
        card_mask.set(STD_DECK_RANK_3 + STD_DECK_SUIT_HEARTS * STD_DECK_RANK_COUNT);
        card_mask.set(STD_DECK_RANK_9 + STD_DECK_SUIT_DIAMONDS * STD_DECK_RANK_COUNT);

        // Vérifiez que le masque n'est pas vide après la définition de cartes
        assert!(!card_mask.is_empty());

        // Réinitialisez le masque
        card_mask.reset();

        // Vérifiez que le masque est maintenant vide
        assert!(card_mask.is_empty());
    }
    #[test]
    fn test_spades() {
        let mut mask = StdDeckCardMask::new();
        mask.set(StdDeck::make_card(STD_DECK_RANK_ACE, STD_DECK_SUIT_SPADES)); // Définit l'As de piques
        println!("mask.spades(): {}", mask.mask_to_string());
        let spades = StdDeckCardMask {
            mask: 1 << STD_DECK_RANK_ACE,
        };
        println!("spades: {}", spades.mask_to_string());
        assert_eq!(
            mask,
            StdDeckCardMask {
                mask: 1 << STD_DECK_RANK_ACE
            },
            "Le masque des piques est incorrect"
        );
    }

    #[test]
    fn test_all_spades() {
        let mut mask = StdDeckCardMask::new();

        // Définir toutes les cartes de pique
        for rank in 0..13 {
            mask.set(StdDeck::make_card(rank, STD_DECK_SUIT_SPADES));
        }
        println!("mask to string: {}", mask.mask_to_string());
        println!("mask.mask: {:b}", mask.mask);
        println!("mask.spades(): {:b}", mask.spades());

        // Calculer le masque attendu pour tous les piques: 0b1111111111111 (13 bits à 1)
        let expected_spades_mask = (1 << 13) - 1; // 8191 ou 0x1FFF

        // Comparer le masque des piques obtenu avec le masque attendu
        assert_eq!(
            mask.spades(),
            expected_spades_mask,
            "Le masque pour tous les piques est incorrect"
        );
    }

    #[test]
    fn test_all_clubs() {
        let mut mask = StdDeckCardMask::new();

        // Définir toutes les cartes de trèfle
        for rank in 0..13 {
            mask.set(StdDeck::make_card(rank, STD_DECK_SUIT_CLUBS));
        }
        println!("mask to string: {}", mask.mask_to_string());
        println!("mask.mask: {:b}", mask.mask);
        println!("mask.clubs(): {:b}", mask.clubs());

        // Calculer le masque attendu pour tous les clubs
        let expected_clubs_mask = (1 << 13) - 1;

        // Comparer le masque des clubs obtenu avec le masque attendu
        assert_eq!(
            mask.clubs(),
            expected_clubs_mask,
            "Le masque pour tous les clubs est incorrect"
        );
    }

    #[test]
    fn test_all_diamonds() {
        let mut mask = StdDeckCardMask::new();

        // Définir toutes les cartes de carreau
        for rank in 0..13 {
            mask.set(StdDeck::make_card(rank, STD_DECK_SUIT_DIAMONDS));
        }
        println!("mask to string: {}", mask.mask_to_string());
        println!("mask.mask: {:b}", mask.mask);
        println!("mask.diamonds(): {:b}", mask.diamonds());

        // Calculer le masque attendu pour tous les diamonds
        let expected_diamonds_mask = (1 << 13) - 1;

        // Comparer le masque des diamonds obtenu avec le masque attendu
        assert_eq!(
            mask.diamonds(),
            expected_diamonds_mask,
            "Le masque pour tous les diamands est incorrect"
        );
    }

    #[test]
    fn test_all_hearts() {
        let mut mask = StdDeckCardMask::new();

        // Définir toutes les cartes de coeur
        for rank in 0..13 {
            mask.set(StdDeck::make_card(rank, STD_DECK_SUIT_HEARTS));
        }
        println!("mask to string: {}", mask.mask_to_string());
        println!("mask.mask: {:b}", mask.mask);
        println!("mask.hearts(): {:b}", mask.hearts());

        // Calculer le masque attendu pour tous les coeurs
        let expected_hearts_mask = (1 << 13) - 1;

        // Comparer le masque des hearts obtenu avec le masque attendu
        assert_eq!(
            mask.hearts(),
            expected_hearts_mask,
            "Le masque pour tous les hearts est incorrect"
        );
    }
}
