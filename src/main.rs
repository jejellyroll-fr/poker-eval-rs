//! Poker Eval RS - Command Line Interface
//!
//! CLI for evaluating poker hands and calculating equity between hands.

use clap::{Parser, Subcommand};
use poker_eval_rs::board::BoardTexture;
use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::deck::StdDeckCardMask;
use poker_eval_rs::enumdefs::{EnumResult, Game, SampleType, ENUM_MAXPLAYERS};
use poker_eval_rs::enumerate::{enum_exhaustive, enum_sample};
use poker_eval_rs::evaluators::range_equity::calculate_equity;
use poker_eval_rs::evaluators::{
    Eval, HandEvaluator, LowballEvaluator, OmahaHiEvaluator, OmahaHiLoEvaluator,
};
use poker_eval_rs::range::HandRange;
use serde::Serialize;
use std::str::FromStr;

/// Poker hand evaluation tool
#[derive(Parser)]
#[command(name = "poker-eval")]
#[command(author = "poker-eval-rs")]
#[command(version = "0.1.0")]
#[command(about = "Evaluate poker hands and calculate equity")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Evaluate a single poker hand
    Eval {
        /// Hand to evaluate (e.g., "AsKsQsJsTs" for a royal flush)
        hand: String,
    },

    /// Calculate equity between hands
    Equity {
        /// Player hands separated by spaces (e.g., "AsAd" "KsKd")
        #[arg(required = true)]
        hands: Vec<String>,

        /// Board cards (e.g., "Th9h8h")
        #[arg(short, long, default_value = "")]
        board: String,

        /// Dead/removed cards
        #[arg(short, long, default_value = "")]
        dead: String,

        /// Game variant (holdem, omaha, etc.)
        #[arg(short, long, default_value = "holdem")]
        game: String,

        /// Use Monte Carlo sampling instead of exhaustive
        #[arg(short, long)]
        monte_carlo: bool,

        /// Number of iterations for Monte Carlo (default: 100000)
        #[arg(short, long, default_value = "100000")]
        iterations: usize,

        /// Output results in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Compare hands to find the winner (static evaluation)
    Compare {
        /// Player hands separated by spaces
        #[arg(required = true)]
        hands: Vec<String>,

        /// Board cards (required for comparison unless complete hands provided)
        #[arg(short, long, default_value = "")]
        board: String,

        /// Game variant
        #[arg(short, long, default_value = "holdem")]
        game: String,
    },

    /// Parse and display card mask information  
    Parse {
        /// Cards to parse (e.g., "AsKd")
        cards: String,
    },

    /// Analyze board texture
    Texture {
        /// Board cards (e.g., "AhKhQh")
        board: String,
    },
}

#[derive(Serialize)]
struct EquityResultOutput {
    game: String,
    samples: u32,
    board: String,
    players: Vec<PlayerStat>,
}

#[derive(Serialize)]
struct PlayerStat {
    hand: String,
    win_pct: f64,
    tie_pct: f64,
    lose_pct: f64,
    scoop_pct: Option<f64>,
    ev: f64,
}

fn parse_game(game_str: &str) -> Result<Game, String> {
    match game_str.to_lowercase().as_str() {
        "holdem" => Ok(Game::Holdem),
        "holdem8" => Ok(Game::Holdem8),
        "omaha" => Ok(Game::Omaha),
        "omaha5" => Ok(Game::Omaha5),
        "omaha6" => Ok(Game::Omaha6),
        "omaha8" => Ok(Game::Omaha8),
        "omaha85" => Ok(Game::Omaha85),
        "stud7" => Ok(Game::Stud7),
        "stud78" => Ok(Game::Stud78),
        "stud7nsq" => Ok(Game::Stud7nsq),
        "razz" => Ok(Game::Razz),
        "draw5" => Ok(Game::Draw5),
        "draw58" => Ok(Game::Draw58),
        "draw5nsq" => Ok(Game::Draw5nsq),
        "lowball" => Ok(Game::Lowball),
        "lowball27" => Ok(Game::Lowball27),
        "shortdeck" => Ok(Game::ShortDeck),
        _ => Err(format!("Unsupported game variant: {}", game_str)),
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Eval { hand } => cmd_eval(&hand),
        Commands::Equity {
            hands,
            board,
            dead,
            game,
            monte_carlo,
            iterations,
            json,
        } => {
            cmd_equity(&hands, &board, &dead, &game, monte_carlo, iterations, json);
        }
        Commands::Compare { hands, board, game } => {
            cmd_compare(&hands, &board, &game);
        }
        Commands::Parse { cards } => cmd_parse(&cards),
        Commands::Texture { board } => cmd_texture(&board),
    }
}

/// Evaluate a single poker hand
fn cmd_eval(hand: &str) {
    match StdDeck::string_to_mask(hand) {
        Ok((mask, num_cards)) => {
            println!("Cards: {}", hand);
            println!("Number of cards: {}", num_cards);
            println!("Mask: {:#018x}", mask.as_raw());

            if num_cards >= 5 {
                let hand_val = Eval::eval_n(&mask, num_cards);
                println!("Hand: {}", hand_val.std_rules_hand_val_to_string());
                println!("Value: {}", hand_val.value);
            } else {
                println!("Need at least 5 cards to evaluate a hand");
            }
        }
        Err(e) => {
            eprintln!("Error parsing hand: {}", e);
            std::process::exit(1);
        }
    }
}

/// Calculate equity between hands
fn cmd_equity(
    hands: &[String],
    board: &str,
    dead: &str,
    game_str: &str,
    monte_carlo: bool,
    iterations: usize,
    json: bool,
) {
    let npockets = hands.len();

    if npockets < 2 {
        eprintln!("Error: Need at least 2 hands for equity calculation");
        std::process::exit(1);
    }

    if npockets > ENUM_MAXPLAYERS {
        eprintln!("Error: Too many players (max {})", ENUM_MAXPLAYERS);
        std::process::exit(1);
    }

    let game_variant = match parse_game(game_str) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Parse hands as ranges first
    let mut ranges: Vec<HandRange> = Vec::new();
    let mut is_range_equity = false;

    for (i, hand) in hands.iter().enumerate() {
        match HandRange::from_str(hand) {
            Ok(range) => {
                if range.len() > 1 {
                    is_range_equity = true;
                }
                ranges.push(range);
            }
            Err(e) => {
                eprintln!("Error parsing hand range {}: {}", i + 1, e);
                std::process::exit(1);
            }
        }
    }

    // If we have ranges, check if supported
    if is_range_equity {
        if npockets != 2 {
            eprintln!("Error: Range equity currently only supported for exactly 2 players.");
            std::process::exit(1);
        }
        if game_variant != Game::Holdem {
            eprintln!("Error: Range equity currently only supported for Hold'em.");
            std::process::exit(1);
        }
    }

    // Parse board
    let board_mask = if board.is_empty() {
        StdDeckCardMask::new()
    } else {
        match StdDeck::string_to_mask(board) {
            Ok((mask, _)) => mask,
            Err(e) => {
                eprintln!("Error parsing board: {}", e);
                std::process::exit(1);
            }
        }
    };

    // Parse dead cards
    let dead_mask = if dead.is_empty() {
        StdDeckCardMask::new()
    } else {
        match StdDeck::string_to_mask(dead) {
            Ok((mask, _)) => mask,
            Err(e) => {
                eprintln!("Error parsing dead cards: {}", e);
                std::process::exit(1);
            }
        }
    };

    let nboard = board_mask.num_cards();

    // Initialize result
    let mut result = EnumResult::new(game_variant);
    result.sample_type = if monte_carlo {
        SampleType::Sample
    } else {
        SampleType::Exhaustive
    };
    result.nplayers = npockets as u32;

    // Run calculation
    if is_range_equity {
        // Use range_equity::calculate_equity
        // Note: This only supports 2 players and ignores dead cards for now in the CLI wrapper
        // (though calculate_equity could support them if passed, preventing overlap).
        // The implementation in range_equity.rs handles board overlap but not explicit dead cards arg.

        match calculate_equity(&ranges[0], &ranges[1], &board_mask, iterations) {
            Ok(equity_res) => {
                if json {
                    let total = equity_res.samples;
                    let players = vec![
                        PlayerStat {
                            hand: hands[0].clone(),
                            win_pct: (equity_res.wins as f64 / total as f64) * 100.0,
                            tie_pct: (equity_res.ties as f64 / total as f64) * 100.0,
                            lose_pct: (equity_res.losses as f64 / total as f64) * 100.0,
                            scoop_pct: None,
                            ev: equity_res.equity,
                        },
                        PlayerStat {
                            hand: hands[1].clone(),
                            win_pct: (equity_res.losses as f64 / total as f64) * 100.0,
                            tie_pct: (equity_res.ties as f64 / total as f64) * 100.0,
                            lose_pct: (equity_res.wins as f64 / total as f64) * 100.0,
                            scoop_pct: None,
                            ev: 1.0 - equity_res.equity,
                        },
                    ];

                    let output = EquityResultOutput {
                        game: game_str.to_string(),
                        samples: total as u32,
                        board: board.to_string(),
                        players,
                    };
                    println!("{}", serde_json::to_string_pretty(&output).unwrap());
                } else {
                    println!("=== Poker Range Equity ===\n");
                    println!("Game: {}", game_str);
                    println!("Samples: {}\n", equity_res.samples);
                    println!(
                        "Board: {}\n",
                        if board.is_empty() { "(none)" } else { board }
                    );

                    println!(
                        "{:<10} {:<20} {:>8} {:>8} {:>8} {:>10}",
                        "Player", "Range", "Win%", "Tie%", "Lose%", "Equity"
                    );
                    println!("{}", "-".repeat(70));

                    let total = equity_res.samples as f64;

                    // P1
                    println!(
                        "{:<10} {:<20} {:>7.2}% {:>7.2}% {:>7.2}% {:>10.4}",
                        "Player 1",
                        hands[0],
                        (equity_res.wins as f64 / total) * 100.0,
                        (equity_res.ties as f64 / total) * 100.0,
                        (equity_res.losses as f64 / total) * 100.0,
                        equity_res.equity
                    );

                    // P2
                    println!(
                        "{:<10} {:<20} {:>7.2}% {:>7.2}% {:>7.2}% {:>10.4}",
                        "Player 2",
                        hands[1],
                        (equity_res.losses as f64 / total) * 100.0,
                        (equity_res.ties as f64 / total) * 100.0,
                        (equity_res.wins as f64 / total) * 100.0,
                        1.0 - equity_res.equity
                    );
                }
            }
            Err(e) => {
                eprintln!("Error calculating range equity: {}", e);
                std::process::exit(1);
            }
        }
        return; // Done
    }

    // Non-range equity (pockets)
    let pockets: Vec<StdDeckCardMask> = ranges.iter().map(|r| r.hands()[0].0).collect();

    let calc_result = if monte_carlo {
        enum_sample(
            game_variant,
            &pockets,
            board_mask,
            dead_mask,
            npockets,
            nboard,
            iterations,
            false,
            &mut result,
        )
    } else {
        enum_exhaustive(
            game_variant,
            &pockets,
            board_mask,
            dead_mask,
            npockets,
            nboard,
            false,
            &mut result,
        )
    };

    match calc_result {
        Ok(_) => {
            if json {
                let mut players = Vec::new();
                for (i, hand) in hands.iter().enumerate().take(npockets) {
                    let total = result.nwinhi[i] + result.ntiehi[i] + result.nlosehi[i];
                    if total > 0 {
                        players.push(PlayerStat {
                            hand: hand.clone(),
                            win_pct: (result.nwinhi[i] as f64 / total as f64) * 100.0,
                            tie_pct: (result.ntiehi[i] as f64 / total as f64) * 100.0,
                            lose_pct: (result.nlosehi[i] as f64 / total as f64) * 100.0,
                            scoop_pct: if game_variant == Game::Holdem8
                                || game_variant == Game::Omaha8
                            {
                                Some((result.nscoop[i] as f64 / total as f64) * 100.0)
                            } else {
                                None
                            },
                            ev: result.ev[i],
                        });
                    }
                }
                let output = EquityResultOutput {
                    game: game_str.to_string(),
                    samples: result.nsamples,
                    board: board.to_string(),
                    players,
                };
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
            } else {
                // Print results
                println!("=== Poker Equity Calculator ===\n");
                println!("Game: {}", game_str);
                println!(
                    "Mode: {}",
                    if monte_carlo {
                        "Monte Carlo"
                    } else {
                        "Exhaustive"
                    }
                );
                println!("Samples: {}\n", result.nsamples);

                println!("Board: {}", if board.is_empty() { "(none)" } else { board });
                println!();

                // Print header
                println!(
                    "{:<10} {:<12} {:>8} {:>8} {:>8} {:>10}",
                    "Player", "Hand", "Win%", "Tie%", "Lose%", "EV"
                );
                println!("{}", "-".repeat(60));

                // Print each player's results
                for (i, hand) in hands.iter().enumerate().take(npockets) {
                    let total = result.nwinhi[i] + result.ntiehi[i] + result.nlosehi[i];
                    if total > 0 {
                        let win_pct = (result.nwinhi[i] as f64 / total as f64) * 100.0;
                        let tie_pct = (result.ntiehi[i] as f64 / total as f64) * 100.0;
                        let lose_pct = (result.nlosehi[i] as f64 / total as f64) * 100.0;

                        println!(
                            "{:<10} {:<12} {:>7.2}% {:>7.2}% {:>7.2}% {:>10.4}",
                            format!("Player {}", i + 1),
                            hand,
                            win_pct,
                            tie_pct,
                            lose_pct,
                            result.ev[i]
                        );
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error during calculation: {:?}", e);
            std::process::exit(1);
        }
    }
}

/// Compare hands to find the winner
fn cmd_compare(hands: &[String], board: &str, game_str: &str) {
    let game = match parse_game(game_str) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let board_mask = if board.is_empty() {
        StdDeckCardMask::new()
    } else {
        match StdDeck::string_to_mask(board) {
            Ok((mask, _)) => mask,
            Err(e) => {
                eprintln!("Error parsing board: {}", e);
                std::process::exit(1);
            }
        }
    };

    let mut results = Vec::new();

    for (i, hand_str) in hands.iter().enumerate() {
        match StdDeck::string_to_mask(hand_str) {
            Ok((mask, _)) => {
                let res = match game {
                    Game::Holdem
                    | Game::Holdem8
                    | Game::Stud7
                    | Game::Stud7nsq
                    | Game::Draw5
                    | Game::Draw5nsq => {
                        let mut combined = mask;
                        combined.or(&board_mask);
                        let count = combined.num_cards();
                        let v = Eval::eval_n(&combined, count);
                        Ok((v.value, v.std_rules_hand_val_to_string()))
                    }
                    Game::Omaha | Game::Omaha5 | Game::Omaha6 => {
                        OmahaHiEvaluator::evaluate_hand(&mask, &board_mask).map(|v| {
                            if let Some(val) = v {
                                (val.value, val.std_rules_hand_val_to_string())
                            } else {
                                (0, "No Hand".to_string())
                            }
                        })
                    }
                    Game::Omaha8 | Game::Omaha85 | Game::Stud78 | Game::Draw58 => {
                        OmahaHiLoEvaluator::evaluate_hand(&mask, &board_mask).map(|(hi, lo)| {
                            let hi_desc = if let Some(val) = hi {
                                val.std_rules_hand_val_to_string()
                            } else {
                                "No Hi".to_string()
                            };
                            let val_u32 = if let Some(val) = hi { val.value } else { 0 };

                            let lo_desc = if let Some(val) = lo {
                                if val.value > 0 {
                                    format!(" / Lo: {}", val)
                                } else {
                                    String::new()
                                }
                            } else {
                                String::new()
                            };

                            (val_u32, format!("{}{}", hi_desc, lo_desc))
                        })
                    }
                    Game::Lowball => LowballEvaluator::evaluate_hand(&mask, &board_mask)
                        .map(|v| (v.value, v.to_string())),
                    Game::Razz | Game::Lowball27 => {
                        let mut combined = mask;
                        combined.or(&board_mask);
                        let count = combined.num_cards();
                        // Uses Lowball27 evaluator logic if available or generic eval_n inverted?
                        // actually std_deck_lowball27_eval is available in evaluators module but Eval::eval_n is high only?
                        // Logic in main.rs line 518 uses Eval::eval_n which is High!
                        // Razz and 2-7 should PROBABLY use specific evaluators if available.

                        // Checking imports: pub use lowball27::std_deck_lowball27_eval; is available in evaluators.
                        // But Eval::eval_n is strictly High.
                        // I should fix Razz/Lowball27 here too.

                        if game == Game::Lowball27 {
                            use poker_eval_rs::evaluators::std_deck_lowball27_eval;
                            let v = std_deck_lowball27_eval(&combined, count);
                            Ok((v.value, v.to_string()))
                        } else {
                            // Razz (A-5 low 7 cards).
                            // Eval::eval_n is high.
                            // Need A-5 low evaluator.
                            use poker_eval_rs::evaluators::std_deck_lowball_eval;
                            let v = std_deck_lowball_eval(&combined, count);
                            Ok((v.value, v.to_string()))
                        }
                    }
                    Game::ShortDeck => {
                        // ShortDeckEvaluator takes separate hole and board
                        poker_eval_rs::evaluators::ShortDeckEvaluator::evaluate_hand(
                            &mask,
                            &board_mask,
                        )
                        .map(|v| (v.value, format_short_deck_hand(v.value)))
                    }
                    Game::NumGames => Err(poker_eval_rs::errors::PokerError::UnsupportedGameType),
                };

                match res {
                    Ok((val, desc)) => results.push((i, hand_str, val, desc)),
                    Err(e) => {
                        eprintln!("Error evaluating hand {}: {:?}", i + 1, e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error parsing hand {}: {}", i + 1, e);
                std::process::exit(1);
            }
        }
    }

    // Sort by value
    // For Lowball variants (Lowball, Lowball27, Razz), Lower value is Better.
    // For High and Hi/Lo variants, Higher value is Better (for the High hand).
    match game {
        Game::Lowball | Game::Lowball27 | Game::Razz => {
            results.sort_by(|a, b| a.2.cmp(&b.2)); // Ascending
        }
        _ => {
            results.sort_by(|a, b| b.2.cmp(&a.2)); // Descending
        }
    }

    println!("=== Hand Comparison ===\n");
    println!("Game: {}", game_str);
    println!("Board: {}", if board.is_empty() { "(none)" } else { board });
    println!();
    println!(
        "{:<8} {:<10} {:<15} {:<15}",
        "Rank", "Player", "Hand", "Description"
    );
    println!("{}", "-".repeat(50));

    let mut rank = 1;
    for (idx, (original_idx, hand_str, _, desc)) in results.iter().enumerate() {
        if idx > 0 && results[idx].2 != results[idx - 1].2 {
            rank = idx + 1;
        }
        println!(
            "{:<8} Player {:<3} {:<15} {:<15}",
            rank,
            original_idx + 1,
            hand_str,
            desc
        );
    }
}

/// Parse and display card information
fn cmd_parse(cards: &str) {
    match StdDeck::string_to_mask(cards) {
        Ok((mask, num_cards)) => {
            println!("Input: {}", cards);
            println!("Cards: {}", mask.mask_to_string());
            println!("Count: {}", num_cards);
            println!("Mask:  {:#018x}", mask.as_raw());
            println!("Binary: {:064b}", mask.as_raw());
        }
        Err(e) => {
            eprintln!("Error parsing cards: {}", e);
            std::process::exit(1);
        }
    }
}

/// Analyze board texture
fn cmd_texture(board_str: &str) {
    let (board, count) = match StdDeck::string_to_mask(board_str) {
        Ok((m, c)) => (m, c),
        Err(e) => {
            eprintln!("Error parsing board: {}", e);
            std::process::exit(1);
        }
    };

    if count < 3 {
        println!("Board must have at least 3 cards for texture analysis.");
        return;
    }

    let texture = BoardTexture::analyze(&board);

    println!("=== Board Texture Analysis ===\n");
    println!("Board: {}", board_str);
    println!("Cards: {}", board.mask_to_string());
    println!();

    println!(
        "Rainbow:       {}",
        if texture.is_rainbow { "Yes" } else { "No" }
    );
    println!(
        "Two Tone:      {}",
        if texture.is_two_tone { "Yes" } else { "No" }
    );
    println!(
        "Monotone:      {}",
        if texture.is_monotone { "Yes" } else { "No" }
    );
    println!(
        "Paired:        {}",
        if texture.is_paired { "Yes" } else { "No" }
    );
    println!(
        "Trips:         {}",
        if texture.is_trips { "Yes" } else { "No" }
    );
    println!(
        "Quads:         {}",
        if texture.is_quads { "Yes" } else { "No" }
    );
    println!(
        "Full House:    {}",
        if texture.is_full_house { "Yes" } else { "No" }
    );
    println!(
        "Straight Draw: {}",
        if texture.has_straight_draw {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "Flush Draw:    {}",
        if texture.has_flush_draw { "Yes" } else { "No" }
    );
}

/// Helper to format Short Deck hands where Flush > Full House
/// In ShortDeckEvaluator, Flush is stored as FullHouse (type 6) and FullHouse as Flush (type 5) to coerce correct integer comparison.
/// We need to swap them back for display.
fn format_short_deck_hand(val: u32) -> String {
    use poker_eval_rs::handval::HandVal;
    let hv = HandVal { value: val };
    let ht = hv.hand_type();

    if ht == 6 {
        // Stored as FullHouse, actually a Flush
        // Read 5 cards
        let ranks = [
            hv.top_card(),
            hv.second_card(),
            hv.third_card(),
            hv.fourth_card(),
            hv.fifth_card(),
        ];
        let mut s = String::from("Flush (");
        for (i, r) in ranks.iter().enumerate() {
            if i > 0 {
                s.push(' ');
            }
            s.push("23456789TJQKA".chars().nth(*r as usize).unwrap_or('?'));
        }
        s.push(')');
        s
    } else if ht == 5 {
        // Stored as Flush, actually a FullHouse
        // Read 2 cards (Trips rank, Pair rank)
        let ranks = [hv.top_card(), hv.second_card()];
        let mut s = String::from("FullHouse (");
        for (i, r) in ranks.iter().enumerate() {
            if i > 0 {
                s.push(' ');
            }
            s.push("23456789TJQKA".chars().nth(*r as usize).unwrap_or('?'));
        }
        s.push(')');
        s
    } else {
        // Standard display for others (Straight, StFlush, Quads, Trips, etc.)
        hv.to_string()
    }
}
