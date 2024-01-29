// Importez les modules nécessaires
use poker_eval_rs::eval::Eval;
use poker_eval_rs::eval_low::std_deck_lowball_eval;
use poker_eval_rs::deck_std::StdDeck;
use poker_eval_rs::enumdefs::{EnumResult, Game, SampleType, ENUM_MAXPLAYERS};
use poker_eval_rs::enumerate::enum_sample;
use poker_eval_rs::enumerate::inner_loop_holdem;
use poker_eval_rs::enumord::EnumOrdering;
use poker_eval_rs::handval::HandVal;
use poker_eval_rs::handval_low::LowHandVal;
use poker_eval_rs::t_cardmasks::StdDeckCardMask;
use std::env;
use std::io::{self, BufRead};
use std::str::FromStr;

// fn main() {
//     let hands = vec![
//         "2h4d5s6h7d",
//         "3h4d5s6h7d",
//         "3h4h5h6h7h",
//         "2h4h5h6h7h",
//         "3h3d5s6h7d",
//         "3h3d5s5h7d",
//         "3h3d3s6h7d",
//         "3h3d3s6h6d",
//         "3h3d3s6h3c",
//         "3h3d3s6h2c2d",
//         "3h3d3s6h2c2d3c",
//         "3h3d5s5h7d2c4d",
//         "Ac2s4d6c8h",
//         "2s3s4d5c7h",
//         "As2d4h3c5d",
//         "KhQhJhTh4h",
//         "AsKcTd2c7s",
//     ];

//     for input in hands {
//         println!("Cartes en entrée: {}", input);

//         // Étape 1: Convertir la chaîne en un masque de cartes
//         let result = StdDeck::string_to_mask(input);
//         let (mask, num_cards) = match result {
//             Ok((mask, num_cards)) => (mask, num_cards),
//             Err(e) => {
//                 eprintln!(
//                     "Erreur lors de la conversion de la chaîne en masque de cartes : {}",
//                     e
//                 );
//                 return; // ou gestion d'erreur alternative
//             }
//         };
//         //println!("Masque de cartes : {:b}, Nombre de cartes : {}", mask.mask, num_cards);

//         // Assurez-vous que le nombre de cartes est correct
//         let actual_num_cards = mask.num_cards();
//         //println!("Nombre de cartes dans le masque : {}", actual_num_cards);
//         assert_eq!(
//             num_cards, actual_num_cards,
//             "Le nombre de cartes ne correspond pas"
//         );

//         // Afficher le masque de cartes
//         //println!("Masque de cartes : {:b}", mask.mask);

//         // Étape 2: Évaluer la main à partir du masque de cartes
//         if num_cards >= 5 {
//             //println!("dans main.rs: nombre de cartes : {:?}", num_cards);
//             //println!("dans main.rs: masque de cartes : {:b}", mask.mask);

//             let hand_val = Eval::eval_n(&mask, num_cards);
//             //println!("HandVal : {:?}", hand_val);

//             // Étape 3: Afficher les informations de HandVal
//             //println!("Type de main : {:?}", hand_val.get_hand_type());
//             println!(
//                 "Représentation de la main : {}",
//                 hand_val.std_rules_hand_val_to_string()
//             );

//             // Évaluer la main pour low
//             let low_hand_val = std_deck_lowball_eval(&mask, num_cards);
//             //println!("Low HandVal : {:?}", low_hand_val);
//             println!(
//                 "Représentation de la main low : {}",
//                 low_hand_val.to_string()
//             );

//             //let low_hand_val = ace_to_five_lowball_eval(&mask); // Utilisez 'mask' ici
//             //low_hand_val.print_ace_to_five_lowball();
//         } else {
//             println!("Nombre de cartes insuffisant pour évaluer une main.");
//         }

//         println!("----------------------");
//     }

//     // Cartes de poche des joueurs
//     let pocket_str1 = "AsKc"; // As de pique, Roi de cœur (Joueur 1)
//     let pocket_str2 = "QhJh"; // Dame de cœur, Valet de cœur (Joueur 2)

//     // Cartes du board (flop, turn, river)
//     let flop_str = "Td2c7s"; // Flop
//     let turn_str = "5c"; // Turn
//     let river_str = "9d"; // River

//     // Convertir les chaînes en masques de cartes
//     let pocket_cards1 = StdDeck::string_to_mask(pocket_str1).unwrap().0;
//     let pocket_cards2 = StdDeck::string_to_mask(pocket_str2).unwrap().0;
//     let flop_cards = StdDeck::string_to_mask(flop_str).unwrap().0;
//     let turn_card = StdDeck::string_to_mask(turn_str).unwrap().0;
//     let river_card = StdDeck::string_to_mask(river_str).unwrap().0;

//     // Combinez le flop, le turn et la river pour créer le board
//     let board = flop_cards | turn_card | river_card;

//     // Évaluer les mains pour les deux joueurs
//     let mut hival1 = vec![HandVal { value: 0 }; 1];
//     let mut loval1 = vec![LowHandVal { value: 0 }; 1];
//     inner_loop_holdem(
//         &[pocket_cards1],
//         &board,
//         &StdDeckCardMask { mask: 0 },
//         &mut hival1,
//         &mut loval1,
//     );

//     let mut hival2 = vec![HandVal { value: 0 }; 1];
//     let mut loval2 = vec![LowHandVal { value: 0 }; 1];
//     inner_loop_holdem(
//         &[pocket_cards2],
//         &board,
//         &StdDeckCardMask { mask: 0 },
//         &mut hival2,
//         &mut loval2,
//     );

//     // Afficher les résultats
//     println!(
//         "Représentation de la main haute pour le Joueur 1: {}",
//         hival1[0].std_rules_hand_val_to_string()
//     );
//     //println!("Représentation de la main basse pour le Joueur 1: {}", loval1[0].to_string());
//     println!(
//         "Représentation de la main haute pour le Joueur 2: {}",
//         hival2[0].std_rules_hand_val_to_string()
//     );
//     //println!("Représentation de la main basse pour le Joueur 2: {}", loval2[0].to_string());
// }

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines();

    // Déterminer si le programme doit lire depuis stdin
    let from_stdin = args.len() == 2 && args[1] == "-i";

    if from_stdin {
        // Lire chaque ligne depuis stdin et traiter comme des arguments
        while let Some(Ok(line)) = input_lines.next() {
            let line_args: Vec<String> = line.split_whitespace().map(String::from).collect();
            if let Err(e) = process_args(&line_args) {
                eprintln!("Erreur lors du traitement des arguments: {}", e);
                return Err(io::Error::new(io::ErrorKind::Other, e));
            }
        }
    } else {
        // Traiter les arguments de la ligne de commande
        if let Err(e) = process_args(&args[1..]) {
            eprintln!("Erreur lors du traitement des arguments: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    }

    Ok(())
}


fn process_args(args: &[String]) -> Result<(), String> {
    let (game, enum_type, niter, pockets, board, dead, npockets, nboard, orderflag, terse) =
        parse_args(args.to_vec())?;

    let mut result = EnumResult {
        game,
        sample_type: enum_type,
        nsamples: 0,
        nplayers: npockets as u32,
        nwinhi: [0; ENUM_MAXPLAYERS],
        ntiehi: [0; ENUM_MAXPLAYERS],
        nlosehi: [0; ENUM_MAXPLAYERS],
        nwinlo: [0; ENUM_MAXPLAYERS],
        ntielo: [0; ENUM_MAXPLAYERS],
        nloselo: [0; ENUM_MAXPLAYERS],
        nscoop: [0; ENUM_MAXPLAYERS],
        nsharehi: [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
        nsharelo: [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
        nshare: [[[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
        ev: [0.0; ENUM_MAXPLAYERS],
        ordering: None,
    };

    let sample_result = match enum_type {
        SampleType::Sample => {
            enum_sample(
                game,
                &pockets,
                board.clone(),
                dead.clone(),
                npockets,
                nboard,
                niter,
                orderflag,
                &mut result,
            )
        }
        SampleType::Exhaustive => {
            // Implémentez ou gérez le cas SampleType::Exhaustive si nécessaire
            todo!()
        }
        // Ajoutez d'autres cas pour SampleType::Exhaustive ou d'autres types si nécessaire
    };

    // Gérez le résultat de l'énumération
    sample_result.map_err(|e| format!("Erreur lors de l'énumération: {}", e))?;

    if terse {
        result.enum_result_print_terse(&pockets, board);
    } else {
        result.enum_result_print(&pockets, board);
    }

    Ok(())
}


// Définition de la fonction parse_args
fn parse_args(
    args: Vec<String>,
) -> Result<
    (
        Game,
        SampleType,
        usize,
        Vec<StdDeckCardMask>,
        StdDeckCardMask,
        StdDeckCardMask,
        usize,
        usize,
        bool,
        bool,
    ),
    String,
> {
    let mut game = Game::Holdem; // Valeur par défaut
    let mut sample_type = SampleType::Sample; // Valeur par défaut
    let mut niter = 0; // Utilisé seulement pour SampleType::Sample
    let mut pockets = Vec::new();
    let mut board = StdDeckCardMask::new();
    let mut dead = StdDeckCardMask::new();
    let mut npockets = 0;
    let nboard = 0;
    let mut orderflag = false;
    let mut terse = false;

    let mut current_pocket = Vec::new();
    let mut parsing_section = "pockets"; // Commencez par analyser les poches des joueurs

    for arg in args.into_iter().skip(1) {
        // Skip le nom du programme
        match arg.as_str() {
            "-mc" => {
                sample_type = SampleType::Sample;
                parsing_section = "niter"; // Prochain argument doit être le nombre d'itérations
            }
            "-t" => terse = true,
            "-O" => orderflag = true,
            "-h" => game = Game::Holdem,
            "-h8" => game = Game::Holdem8,
            // Ajoutez d'autres options ici...
            "--" => {
                // Terminez de traiter la poche actuelle et commencez à traiter le tableau
                if !current_pocket.is_empty() {
                    let (mask, _) = StdDeck::string_to_mask(&current_pocket.join(""))?;
                    pockets.push(mask);
                    current_pocket.clear();
                }
                parsing_section = "board";
            }
            "/" => {
                // Terminez de traiter la section actuelle et commencez à traiter les cartes mortes
                parsing_section = "dead";
            }
            "-" => {
                // Terminez de traiter la poche actuelle et commencez une nouvelle poche
                if !current_pocket.is_empty() {
                    let (mask, _) = StdDeck::string_to_mask(&current_pocket.join(""))?;
                    pockets.push(mask);
                    current_pocket.clear();
                }
                npockets += 1; // Augmentez le compteur de poches
            }
            _ => match parsing_section {
                "niter" => niter = arg.parse().map_err(|_| "Nombre d'itérations invalide")?,
                "pockets" | "board" | "dead" => current_pocket.push(arg.to_string()), // Ajoutez la carte à la poche, au tableau ou aux cartes mortes actuels
                _ => return Err("Section d'analyse inconnue".to_string()),
            },
        }
    }

    // Assurez-vous de traiter la dernière poche ou le dernier tableau
    if !current_pocket.is_empty() {
        let (mask, _) = StdDeck::string_to_mask(&current_pocket.join(""))?;
        match parsing_section {
            "pockets" => {
                pockets.push(mask);
                npockets += 1;
            }
            "board" => board = mask,
            "dead" => dead = mask,
            _ => (),
        }
    }

    Ok((
        game,
        sample_type,
        niter,
        pockets,
        board,
        dead,
        npockets,
        nboard,
        orderflag,
        terse,
    ))
}
