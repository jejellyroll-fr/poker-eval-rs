use poker_eval_rs::deck::{StdDeckCardMask, STD_DECK_N_CARDS};
use poker_eval_rs::evaluators::lowball8::std_deck_lowball8_eval;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let _niter = 1_000_000;
    let npockets = 2;

    // Reproduce benchmark_variants logic
    loop {
        let mut rng = thread_rng();
        let mut deck: Vec<usize> = (0..STD_DECK_N_CARDS).collect();
        deck.shuffle(&mut rng);

        let mut pockets = Vec::new();
        let mut card_idx = 0;
        for _ in 0..npockets {
            let mut hand = StdDeckCardMask::new();
            for _ in 0..2 {
                // 2 hole cards for Holdem
                hand.set(deck[card_idx]);
                card_idx += 1;
            }
            pockets.push(hand);
        }

        let board_cards: Vec<usize> = deck[card_idx..card_idx + 5].to_vec();
        let mut board = StdDeckCardMask::new();
        for &c in &board_cards {
            board.set(c);
        }

        // Evaluate
        for &hand_mask in &pockets {
            let hand = hand_mask | board;
            // Calls std_deck_lowball8_eval which calls std_deck_lowball_eval
            // We verify if it crashes
            let _ = std_deck_lowball8_eval(&hand, 7);
        }
    }
}
