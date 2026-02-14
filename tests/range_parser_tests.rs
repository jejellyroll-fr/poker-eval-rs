use poker_eval_rs::range::HandRange;
use proptest::prelude::*;
use std::str::FromStr;

#[test]
fn test_parser_edge_cases() {
    // Empty string
    let range = HandRange::from_str("").unwrap();
    assert!(range.is_empty());

    // Whitespace only
    let range = HandRange::from_str("   ,  ").unwrap();
    assert!(range.is_empty());

    // Mixed valid and invalid (should fail entire string or just skip? implementation fails on first error)
    assert!(HandRange::from_str("AA, invalid").is_err());

    // Case insensitivity (if supported, currently Rank::from_char is typically case-insensitive, let's verify)
    let range = HandRange::from_str("aa").unwrap();
    assert!(!range.is_empty());
    assert_eq!(range.len(), 6); // AA is 6 combos
}

#[test]
fn test_open_intervals() {
    // "55+" -> 55, 66, ..., AA
    // Ranks: 5,6,7,8,9,T,J,Q,K,A = 10 ranks. 10 * 6 = 60 combos.
    let range = HandRange::from_str("55+").unwrap();
    assert_eq!(range.len(), 60);

    // "A2s+" -> A2s, A3s, ..., AKs
    // 2..K = 12 ranks (2,3,4,5,6,7,8,9,T,J,Q,K). Cards pairing with A.
    // 12 * 4 = 48 combos.
    let range = HandRange::from_str("A2s+").unwrap();
    assert_eq!(range.len(), 48);
}

#[test]
fn test_gap_intervals() {
    // "99-77" -> 99, 88, 77. 3 * 6 = 18.
    let range = HandRange::from_str("99-77").unwrap();
    assert_eq!(range.len(), 18);

    // "77-99" (reverse order check)
    let range = HandRange::from_str("77-99").unwrap();
    assert_eq!(range.len(), 18);

    // "KJs-K9s" -> KJs, KTs, K9s. 3 * 4 = 12.
    let range = HandRange::from_str("KJs-K9s").unwrap();
    assert_eq!(range.len(), 12);
}

#[test]
fn test_specific_combos() {
    // "AhKh"
    let range = HandRange::from_str("AhKh").unwrap();
    assert_eq!(range.len(), 1);
}

// Property Tests
proptest! {
    #[test]
    fn prop_parser_does_not_panic(s in "\\PC*") {
        // Any printable character string
        let _ = HandRange::from_str(&s);
    }

    #[test]
    fn prop_valid_pair_strings(rank_char in "[2-9TJQKA]") {
        let s = format!("{}{}", rank_char, rank_char);
        let range = HandRange::from_str(&s).unwrap();
        prop_assert_eq!(range.len(), 6);
    }

    #[test]
    fn prop_valid_offsuit_strings(
        r1 in "[2-9TJQKA]",
        r2 in "[2-9TJQKA]"
    ) {
        // Ensure distinct
        if r1 != r2 {
            let s = format!("{}{}o", r1, r2);
            // This relies on canonical handling (pair or not)
            // But if r1!=r2, it's valid non-pair
            let range = HandRange::from_str(&s);
             if range.is_err() {
                 println!("Failed to parse valid offsuit: {}", s);
             }
            prop_assert!(range.is_ok(), "Failed to parse {}", s);
            let range = range.unwrap();
            prop_assert_eq!(range.len(), 12);
        }
    }

    #[test]
    fn prop_valid_suited_strings(
        r1 in "[2-9TJQKA]",
        r2 in "[2-9TJQKA]"
    ) {
        if r1 != r2 {
             let s = format!("{}{}s", r1, r2);
             let range = HandRange::from_str(&s);
             prop_assert!(range.is_ok(), "Failed to parse {}", s);
             let range = range.unwrap();
             prop_assert_eq!(range.len(), 4);
        }
    }
}
