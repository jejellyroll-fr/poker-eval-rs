mod deck;
mod deck_std;
mod rules_std;
mod handval;

use deck_std::*;
use crate::rules_std::*;
use crate::handval::*;

fn main() {
    let input = "3h4d5s6h7d";
    println!("Cartes en entrée: {}", input);

    // Étape 1: Convertir la chaîne en un masque de cartes
    let (mask, num_cards) = StdDeck::string_to_mask(input);
    println!("Masque de cartes : {:b}, Nombre de cartes : {}", mask.mask, num_cards);

    // Étape 2: Extraire les rangs des cartes du masque
    let mut card_ranks = Vec::new();
    for i in 0..STD_DECK_N_CARDS {
        if mask.card_is_set(i) {
            let rank = StdDeck::rank(i);
            card_ranks.push(rank);
            println!("Carte trouvée - Index: {}, Rang: {}", i, rank);
        }
    }

    // Tri et préparation des rangs
    card_ranks.sort();
    card_ranks.reverse(); // Pour un ordre décroissant (si nécessaire)

    // Étape 3: Créer un HandVal à partir des rangs extraits
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
}


