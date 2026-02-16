#![cfg(target_arch = "wasm32")]

use poker_eval_rs::wasm_bindings::calculate_equity;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn wasm_rejects_board_for_stud7() {
    let err = calculate_equity("AsKsQh AdKdQc", "2c", "", "stud7", true, 32).unwrap_err();
    let msg = err
        .as_string()
        .unwrap_or_else(|| "non-string-js-error".to_string())
        .to_lowercase();
    assert!(msg.contains("invalid board configuration"));
}

#[wasm_bindgen_test]
fn wasm_rejects_invalid_exhaustive_street_for_holdem() {
    let err = calculate_equity("AsKs QhQd", "2c 7d", "", "holdem", false, 32).unwrap_err();
    let msg = err
        .as_string()
        .unwrap_or_else(|| "non-string-js-error".to_string())
        .to_lowercase();
    assert!(msg.contains("invalid board configuration"));
}
