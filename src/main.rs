#![allow(dead_code)]
// Importez les modules nécessaires
pub mod deck;
pub mod deck_std;
pub mod enumord;
pub mod eval;
pub mod eval_low;
pub mod eval_low27;
pub mod eval_low8;
pub mod handval;
pub mod handval_low;
pub mod lowball;
pub mod rules_std;
pub mod t_botcard;
pub mod t_botfivecards;
pub mod t_cardmasks;
pub mod t_nbits;
pub mod t_straight;
pub mod t_topcard;
pub mod t_topfivecards;
pub mod eval_omaha;
pub mod t_jokercardmasks;
pub mod deck_joker;
pub mod eval_joker_low;
pub mod enumdefs;
pub mod combinaison;
pub mod enumerate;

use crate::eval::Eval;
use crate::eval_low::std_deck_lowball_eval;
use deck_std::*;


fn main() {
    let hands = vec![
        "2h4d5s6h7d",
        "3h4d5s6h7d",
        "3h4h5h6h7h",
        "2h4h5h6h7h",
        "3h3d5s6h7d",
        "3h3d5s5h7d",
        "3h3d3s6h7d",
        "3h3d3s6h6d",
        "3h3d3s6h3c",
        "3h3d3s6h2c2d",
        "3h3d3s6h2c2d3c",
        "3h3d5s5h7d2c4d",
        "Ac2s4d6c8h",
        "2s3s4d5c7h",
        "As2d4h3c5d",
        "KhQhJhTh4h",

    ];

    for input in hands {
        println!("Cartes en entrée: {}", input);

        // Étape 1: Convertir la chaîne en un masque de cartes
        let result = StdDeck::string_to_mask(input);
        let (mask, num_cards) = match result {
            Ok((mask, num_cards)) => (mask, num_cards),
            Err(e) => {
                eprintln!("Erreur lors de la conversion de la chaîne en masque de cartes : {}", e);
                return; // ou gestion d'erreur alternative
            }
        };
        //println!("Masque de cartes : {:b}, Nombre de cartes : {}", mask.mask, num_cards);

        // Assurez-vous que le nombre de cartes est correct
        let actual_num_cards = mask.num_cards();
        //println!("Nombre de cartes dans le masque : {}", actual_num_cards);
        assert_eq!(
            num_cards, actual_num_cards,
            "Le nombre de cartes ne correspond pas"
        );

        // Afficher le masque de cartes
        //println!("Masque de cartes : {:b}", mask.mask);

        // Étape 2: Évaluer la main à partir du masque de cartes
        if num_cards >= 5 {
            //println!("dans main.rs: nombre de cartes : {:?}", num_cards);
            //println!("dans main.rs: masque de cartes : {:b}", mask.mask);

            let hand_val = Eval::eval_n(&mask, num_cards);
            //println!("HandVal : {:?}", hand_val);

            // Étape 3: Afficher les informations de HandVal
            //println!("Type de main : {:?}", hand_val.get_hand_type());
            println!(
                "Représentation de la main : {}",
                hand_val.std_rules_hand_val_to_string()
            );

            // Évaluer la main pour low
            let low_hand_val = std_deck_lowball_eval(&mask, num_cards);
            //println!("Low HandVal : {:?}", low_hand_val);
            println!(
                "Représentation de la main low : {}",
                low_hand_val.to_string()
            );

            //let low_hand_val = ace_to_five_lowball_eval(&mask); // Utilisez 'mask' ici
            //low_hand_val.print_ace_to_five_lowball();
        } else {
            println!("Nombre de cartes insuffisant pour évaluer une main.");
        }

        println!("----------------------");
    }

}

