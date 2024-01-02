mod deck;
mod deck_std;

use deck_std::*;


fn main() {
    // Test de la conversion de carte en chaîne et vice-versa
    let card_index = StdDeck::make_card(STD_DECK_RANK_3, STD_DECK_SUIT_HEARTS);
    let card_str = StdDeck::card_to_string(card_index);
    println!("Card String: {}", card_str);

    if let Some(index) = StdDeck::string_to_card(&card_str) {
        println!("Card Index: {}", index);
        println!("Rank: {}", StdDeck::rank(index));
        println!("Suit: {}", StdDeck::suit(index));
    }
    // Test de la fonction card_to_string
    let card_str = StdDeck::card_to_string(StdDeck::make_card(STD_DECK_RANK_3, STD_DECK_SUIT_HEARTS));
    println!("Card String: {}", card_str);

    // Création et test des méthodes de StdDeckCardMask
    let mut card_mask = StdDeckCardMask::new();
    
    // Test de set_spades, set_clubs, set_diamonds, set_hearts
    card_mask.set_spades(0b101); // Exemple : définit certains masques de pique
    card_mask.set_clubs(0b110);  // Exemple : définit certains masques de trèfle
    card_mask.set_diamonds(0b100); // Exemple : définit certains masques de carreau
    card_mask.set_hearts(0b111); // Exemple : définit certains masques de cœur

    // Test des opérations bitwise
    let mut other_mask = StdDeckCardMask::new();
    other_mask.set_spades(0b010);
    
    card_mask.or(&other_mask); // Test de l'opération OR
    println!("OR Operation Result: {}", card_mask.mask);
    
    card_mask.and(&other_mask); // Test de l'opération AND
    println!("AND Operation Result: {}", card_mask.mask);
    
    card_mask.xor(&other_mask); // Test de l'opération XOR
    println!("XOR Operation Result: {}", card_mask.mask);

    card_mask.not(); // Test de l'opération NOT
    println!("NOT Operation Result: {}", card_mask.mask);

    // Test des méthodes reset, is_empty et equals
    card_mask.reset();
    println!("Is Empty after reset: {}", card_mask.is_empty());

    let is_equal = card_mask.equals(&other_mask);
    println!("Is Equal to other mask: {}", is_equal);

    // Autres tests...
    let input = "3h4d";
    let (mask, num_cards) = StdDeck::string_to_mask(input);
    println!("Number of cards: {}", num_cards);

    let count = mask.num_cards();
    println!("Count from mask: {}", count);
}