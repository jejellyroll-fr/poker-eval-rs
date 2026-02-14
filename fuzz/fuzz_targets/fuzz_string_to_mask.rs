#![no_main]
use libfuzzer_sys::fuzz_target;
use poker_eval_rs::deck::StdDeck;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Just call the function and ensure it doesn't panic.
        // We ignore the result (Ok or Err are both fine, just no panic).
        let _ = StdDeck::string_to_mask(s);
    }
});
