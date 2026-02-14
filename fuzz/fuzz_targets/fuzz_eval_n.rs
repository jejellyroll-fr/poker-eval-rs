#![no_main]
use libfuzzer_sys::fuzz_target;
use poker_eval_rs::deck::{StdDeckCardMask, StdDeck};
use poker_eval_rs::evaluators::Eval;

fuzz_target!(|data: &[u8]| {
    // Need at least 1 byte for n_cards + some bytes for mask
    if data.len() < 2 {
        return;
    }
    
    // First byte determines number of cards (modulo 8, or perhaps up to 7)
    let n_cards_byte = data[0];
    // n_cards useful range: 0..=7. Maybe stretch it to see if it panics > 7.
    // Let's allow up to 10 to catch out of bounds.
    let n_cards = (n_cards_byte % 10) as usize;
    
    // Remaining bytes used to construct mask.
    // We can interpret bytes as card indices modulo 52.
    let mut mask = StdDeckCardMask::new();
    for &b in &data[1..] {
        let card_idx = (b as usize) % 52;
        mask.set(card_idx);
    }
    
    // Call eval_n. It should return a valid HandVal or panic?
    // Ideally it should not panic for any input mask even if n_cards is weird.
    // If n_cards > mask.count(), behavior is undefined but shouldn't be UB in safe Rust (no segfault).
    // It might panic or return garbage.
    // We want to catch panics.
    // Fuzzing catches panics by default.
    let _ = Eval::eval_n(&mask, n_cards);
});
