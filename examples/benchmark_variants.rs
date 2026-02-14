use poker_eval_rs::deck::{StdDeckCardMask, STD_DECK_N_CARDS};
use poker_eval_rs::enumdefs::{EnumResult, Game};
use poker_eval_rs::enumerate::enum_sample;
use std::time::Instant;

// We need rand for shuffling
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let variants = vec![
        (Game::Holdem, "Hold'em", 2),
        (Game::Holdem8, "Hold'em 8", 2),
        (Game::Omaha, "Omaha", 2),
        (Game::Omaha5, "Omaha 5", 2),
        (Game::Omaha6, "Omaha 6", 2),
        (Game::Omaha8, "Omaha 8", 2),
        (Game::Omaha85, "Omaha 8 (5-card)", 2),
        (Game::Stud7, "7-Card Stud", 2),
        (Game::Stud78, "7-Card Stud 8", 2),
        (Game::Stud7nsq, "7-Card Stud NSQ", 2),
        (Game::Razz, "Razz", 2),
        (Game::Lowball, "Lowball A-5", 2),
        (Game::Lowball27, "Lowball 2-7", 2),
        (Game::Draw5, "5-Card Draw", 2),
        (Game::Draw58, "5-Card Draw 8", 2),
        (Game::Draw5nsq, "5-Card Draw NSQ", 2),
        (Game::ShortDeck, "Short Deck", 2),
    ];

    println!("| Variant | Players | Iterations | Time (s) | Speed (sim/sec) |");
    println!("|---|---|---|---|---|");

    for (game, name, players) in variants {
        let (speed, time) = benchmark_variant(game, players);
        println!(
            "| {} | {} | 500,000 | {:.2}s | {} |",
            name,
            players,
            time,
            format_speed(speed)
        );
    }
}

fn benchmark_variant(game: Game, n_players: usize) -> (f64, f64) {
    let n_iter = 500_000;

    // Create a deck and shuffle it
    let mut rng = thread_rng();
    let mut deck: Vec<usize> = (0..STD_DECK_N_CARDS).collect();
    deck.shuffle(&mut rng);

    // Deal pockets
    let mut pockets = Vec::with_capacity(n_players);
    let mut card_idx = 0;

    // Determine cards per pocket based on game type approximation
    // Most games use 2 or more cards for pockets in simulation input
    // Only Stud/Draw require attention if simulation expects specific input size?
    // Independent game simulation usually deals "the rest"
    // So if we give 2 cards, it deals the rest to reach 5 or 7.
    // Let's give 2 cards per player as a standard "starting hand".
    // For Draw, input hand should be 5 cards?
    // Actually `simulate_draw_game` converts input pockets to initial hand and then draws.
    // If input is empty, it deals 5?
    // Let's provide 2 cards to verify dealing logic works for all.
    // Except Draw/Lowball usually starts with 5.

    let cards_per_pocket = match game {
        Game::Draw5 | Game::Draw58 | Game::Draw5nsq | Game::Lowball | Game::Lowball27 => 5,
        Game::Omaha | Game::Omaha8 => 4,
        Game::Omaha5 | Game::Omaha85 => 5,
        Game::Omaha6 => 6,
        Game::Stud7 | Game::Stud78 | Game::Stud7nsq | Game::Razz => 3,
        _ => 2, // Holdem, ShortDeck
    };

    for _ in 0..n_players {
        let mut hand = StdDeckCardMask::new();
        for _ in 0..cards_per_pocket {
            if card_idx < deck.len() {
                hand.set(deck[card_idx]);
                card_idx += 1;
            }
        }
        pockets.push(hand);
    }

    let board = StdDeckCardMask::new();
    let dead = StdDeckCardMask::new();
    let mut result = EnumResult::default();

    let start = Instant::now();
    let _ = enum_sample(
        game,
        &pockets,
        board,
        dead,
        n_players,
        0,
        n_iter,
        true,
        &mut result,
    );
    let duration = start.elapsed();
    let time_secs = duration.as_secs_f64();

    (n_iter as f64 / time_secs, time_secs)
}

fn format_speed(speed: f64) -> String {
    if speed >= 1_000_000.0 {
        format!("{:.2} M/s", speed / 1_000_000.0)
    } else {
        format!("{:.2} K/s", speed / 1_000.0)
    }
}
