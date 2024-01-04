// Importez les modules nécessaires
mod deck;
mod deck_std;
mod rules_std;
mod handval;
mod t_cardmasks;
mod t_nbits;
mod t_straight;
mod t_topfivecards;
mod t_topcard;
mod eval;

use deck_std::*;
use crate::rules_std::*;
use crate::handval::*;
use crate::eval::Eval; // Assurez-vous d'importer eval pour accéder à eval_n

fn main() {
    let input = "3h4d5s6h7d";
    println!("Cartes en entrée: {}", input);

    // Étape 1: Convertir la chaîne en un masque de cartes
    let (mask, num_cards) = StdDeck::string_to_mask(input);
    println!("Masque de cartes : {:b}, Nombre de cartes : {}", mask.mask, num_cards);

    // Assurez-vous que le nombre de cartes est correct
    let actual_num_cards = mask.num_cards();
    println!("Nombre de cartes dans le masque : {}", actual_num_cards);
    assert_eq!(num_cards, actual_num_cards, "Le nombre de cartes ne correspond pas");

    // Étape 2: Évaluer la main à partir du masque de cartes
    if num_cards >= 5 {
        let hand_val = Eval::eval_n(&mask, num_cards);

        // Étape 3: Afficher les informations de HandVal
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

