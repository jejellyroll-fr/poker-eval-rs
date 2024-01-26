#![allow(dead_code)]
// Importez les modules nécessaires
pub mod combinaison;
pub mod deck;
pub mod deck_joker;
pub mod deck_std;
pub mod enumdefs;
pub mod enumerate;
pub mod enumord;
pub mod eval;
pub mod eval_joker;
pub mod eval_joker_low;
pub mod eval_low;
pub mod eval_low27;
pub mod eval_low8;
pub mod eval_omaha;
pub mod handval;
pub mod handval_low;
pub mod lowball;
pub mod rules_joker;
pub mod rules_std;
pub mod t_botcard;
pub mod t_botfivecards;
pub mod t_cardmasks;
pub mod t_jokercardmasks;
pub mod t_jokerstraight;
pub mod t_nbits;
pub mod t_straight;
pub mod t_topcard;
pub mod t_topfivecards;

use crate::enumerate::inner_loop_holdem;
use crate::eval::Eval;
use crate::eval_low::std_deck_lowball_eval;
use crate::handval::HandVal;
use crate::handval_low::LowHandVal;
use crate::t_cardmasks::StdDeckCardMask;
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
        "AsKcTd2c7s",
    ];

    for input in hands {
        println!("Cartes en entrée: {}", input);

        // Étape 1: Convertir la chaîne en un masque de cartes
        let result = StdDeck::string_to_mask(input);
        let (mask, num_cards) = match result {
            Ok((mask, num_cards)) => (mask, num_cards),
            Err(e) => {
                eprintln!(
                    "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
                    e
                );
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

    // Cartes de poche des joueurs
    let pocket_str1 = "AsKc"; // As de pique, Roi de cœur (Joueur 1)
    let pocket_str2 = "QhJh"; // Dame de cœur, Valet de cœur (Joueur 2)

    // Cartes du board (flop, turn, river)
    let flop_str = "Td2c7s"; // Flop
    let turn_str = "5c"; // Turn
    let river_str = "9d"; // River

    // Convertir les chaînes en masques de cartes
    let pocket_cards1 = StdDeck::string_to_mask(pocket_str1).unwrap().0;
    let pocket_cards2 = StdDeck::string_to_mask(pocket_str2).unwrap().0;
    let flop_cards = StdDeck::string_to_mask(flop_str).unwrap().0;
    let turn_card = StdDeck::string_to_mask(turn_str).unwrap().0;
    let river_card = StdDeck::string_to_mask(river_str).unwrap().0;

    // Combinez le flop, le turn et la river pour créer le board
    let board = flop_cards | turn_card | river_card;

    // Évaluer les mains pour les deux joueurs
    let mut hival1 = vec![HandVal { value: 0 }; 1];
    let mut loval1 = vec![LowHandVal { value: 0 }; 1];
    inner_loop_holdem(
        &[pocket_cards1],
        &board,
        &StdDeckCardMask { mask: 0 },
        &mut hival1,
        &mut loval1,
    );

    let mut hival2 = vec![HandVal { value: 0 }; 1];
    let mut loval2 = vec![LowHandVal { value: 0 }; 1];
    inner_loop_holdem(
        &[pocket_cards2],
        &board,
        &StdDeckCardMask { mask: 0 },
        &mut hival2,
        &mut loval2,
    );

    // Afficher les résultats
    println!(
        "Représentation de la main haute pour le Joueur 1: {}",
        hival1[0].std_rules_hand_val_to_string()
    );
    //println!("Représentation de la main basse pour le Joueur 1: {}", loval1[0].to_string());
    println!(
        "Représentation de la main haute pour le Joueur 2: {}",
        hival2[0].std_rules_hand_val_to_string()
    );
    //println!("Représentation de la main basse pour le Joueur 2: {}", loval2[0].to_string());
}
