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
mod handval_low;
mod eval_low;
mod t_botcard;
mod lowball;
mod enumord;
mod eval_low8;
mod t_botfivecards;

use deck_std::*;
use crate::rules_std::*;
use crate::handval::*;
use crate::eval::Eval; 
use crate::eval_low::std_deck_lowball_eval;
use crate::lowball::*;


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
        "3h3d5s5h7d2c4d"
    ];

    for input in hands {
        println!("Cartes en entrée: {}", input);

        // Étape 1: Convertir la chaîne en un masque de cartes
        let (mask, num_cards) = StdDeck::string_to_mask(input);
        //println!("Masque de cartes : {:b}, Nombre de cartes : {}", mask.mask, num_cards);

        // Assurez-vous que le nombre de cartes est correct
        let actual_num_cards = mask.num_cards();
        //println!("Nombre de cartes dans le masque : {}", actual_num_cards);
        assert_eq!(num_cards, actual_num_cards, "Le nombre de cartes ne correspond pas");

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
            println!("Représentation de la main : {}", hand_val.StdRules_HandVal_toString());

           // Évaluer la main pour low
            let low_hand_val = std_deck_lowball_eval(&mask, num_cards);
            //println!("Low HandVal : {:?}", low_hand_val);
            println!("Représentation de la main low : {}", low_hand_val.to_string());

            //let low_hand_val = ace_to_five_lowball_eval(&mask); // Utilisez 'mask' ici
            //low_hand_val.print_ace_to_five_lowball();  


        } else {
            println!("Nombre de cartes insuffisant pour évaluer une main.");
        }

        println!("----------------------");
    }
}



// test unitaires
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nbits_table() {
        for (index, &value) in NBITS_TABLE.iter().enumerate() {
            let expected_nbits = format!("{:b}", index).matches('1').count();
            assert_eq!(usize::from(value), expected_nbits, "Échec au niveau de l'index {}: attendu {}, obtenu {}", index, expected_nbits, value);

        }
    }

    #[test]
    fn test_enum_ordering_rank() {
        let mut hands = [HandVal(5), HandVal(3), HandVal(8)]; // Exemple de valeurs
        let mut ranks = [0; 3];
        enum_ordering_rank(&mut hands, HandVal(0), 3, &mut ranks, false);

        assert_eq!(ranks, [1, 0, 2]); // Résultats attendus
    }
}