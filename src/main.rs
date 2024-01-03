mod deck;
mod deck_std;
mod rules_std;
mod handval;
mod t_cardmasks;

use deck_std::*;
use crate::rules_std::*;
use crate::handval::*;

fn main() {
    let input = "3h4d5s6h7d";
    println!("Cartes en entrée: {}", input);

    // Étape 1: Convertir la chaîne en un masque de cartes
    let (mask, num_cards) = StdDeck::string_to_mask(input);
    println!("Masque de cartes : {:b}, Nombre de cartes : {}", mask.mask, num_cards);

    // Vérifier le nombre de cartes dans le masque
    let actual_num_cards = mask.num_cards();
    println!("Nombre de cartes dans le masque : {}", actual_num_cards);
    assert_eq!(num_cards, actual_num_cards, "Le nombre de cartes ne correspond pas");

    // Étape 2: Extraire les rangs des cartes du masque
    let mut card_ranks = Vec::new();
    println!("Début de l'extraction des cartes...");
    for i in 0..STD_DECK_N_CARDS {
        if mask.card_is_set(i) {
            let rank = StdDeck::rank(i);
            let suit = StdDeck::suit(i);
            card_ranks.push(rank);
            println!("Carte trouvée - Index: {}, Rang: {}, Couleur: {}", i, rank, suit);
        }
    }
    println!("Extraction terminée. Nombre de cartes extraites : {}", card_ranks.len());


    // Tri et préparation des rangs
    card_ranks.sort();
    card_ranks.reverse(); // Pour un ordre décroissant (si nécessaire)

// Étape 3: Créer un HandVal à partir des rangs extraits
if card_ranks.len() >= 5 {
    let hand_val = HandVal::new(4, // Exemple : type de main, ici Straight
                                card_ranks[0] as u8,
                                card_ranks[1] as u8,
                                card_ranks[2] as u8,
                                card_ranks[3] as u8,
                                card_ranks[4] as u8);

    // Étape 4: Afficher les informations de HandVal
    println!("Type de main : {:?}", hand_val.get_hand_type());
    println!("Représentation de la main : {}", hand_val.StdRules_HandVal_toString());

    // Affichage des valeurs individuelles des cartes
    println!("Carte supérieure : {}", hand_val.top_card());
    println!("Deuxième carte : {}", hand_val.second_card());
    println!("Troisième carte : {}", hand_val.third_card());
    println!("Quatrième carte : {}", hand_val.fourth_card());
    println!("Cinquième carte : {}", hand_val.fifth_card());
} else {
    println!("Nombre de cartes insuffisant pour évaluer une main.");
}

}


