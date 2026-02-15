use poker_eval_rs::deck::{Deck, ManilaDeck, StdDeck};
use poker_eval_rs::evaluators::{
    badugi_eval, std_deck_lowball27_eval, std_deck_lowball_eval, EvalJoker, HandEvaluator,
    HoldemEvaluator, OFCBoard, OmahaHiEvaluator, OmahaHiLoEvaluator, ShortDeckEvaluator,
};
use poker_eval_rs::rules::HandType;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct RegressionDataset {
    single_cases: Vec<SingleCase>,
    comparison_cases: Vec<ComparisonCase>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Variant {
    Holdem,
    Shortdeck,
    Manila,
    Badugi,
    OmahaHi,
    OmahaHilo,
    LowballA5,
    Lowball27,
    JokerHi,
    Ofc,
}

#[derive(Debug, Deserialize)]
struct SingleCase {
    id: String,
    variant: Variant,
    cards: Option<String>,
    hole: Option<String>,
    board: Option<String>,
    top: Option<String>,
    middle: Option<String>,
    bottom: Option<String>,
    expected_hand_type: Option<String>,
    expected_low_hand_type: Option<u8>,
    expected_has_low: Option<bool>,
    expected_is_foul: Option<bool>,
    expected_royalties: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct ComparisonCase {
    left_id: String,
    right_id: String,
    op: String,
}

fn parse_mask(cards: &str) -> poker_eval_rs::deck::StdDeckCardMask {
    StdDeck::string_to_mask(cards).unwrap().0
}

fn parse_hand_type(name: &str) -> HandType {
    match name {
        "NoPair" => HandType::NoPair,
        "OnePair" => HandType::OnePair,
        "TwoPair" => HandType::TwoPair,
        "Trips" => HandType::Trips,
        "Straight" => HandType::Straight,
        "Flush" => HandType::Flush,
        "FullHouse" => HandType::FullHouse,
        "Quads" => HandType::Quads,
        "StFlush" => HandType::StFlush,
        _ => panic!("Unknown HandType in regression dataset: {name}"),
    }
}

#[test]
fn regression_dataset_cases() {
    let raw = include_str!("data/regression_eval_cases.json");
    let dataset: RegressionDataset =
        serde_json::from_str(raw).expect("valid regression dataset json");

    let mut comparable_values: HashMap<String, u32> = HashMap::new();

    for case in &dataset.single_cases {
        match case.variant {
            Variant::Holdem => {
                let cards = case.cards.as_ref().expect("holdem case requires cards");
                let val =
                    HoldemEvaluator::evaluate_hand(&parse_mask(cards), &parse_mask("")).unwrap();

                if let Some(expected) = &case.expected_hand_type {
                    let expected = parse_hand_type(expected);
                    assert_eq!(
                        val.hand_type(),
                        expected as u8,
                        "case {} expected hand type {expected:?}",
                        case.id
                    );
                }
                comparable_values.insert(case.id.clone(), val.value);
            }
            Variant::Shortdeck => {
                let cards = case.cards.as_ref().expect("shortdeck case requires cards");
                let val =
                    ShortDeckEvaluator::evaluate_hand(&parse_mask(cards), &parse_mask("")).unwrap();

                if let Some(expected) = &case.expected_hand_type {
                    let expected = parse_hand_type(expected);
                    assert_eq!(
                        val.hand_type(),
                        expected as u8,
                        "case {} expected hand type {expected:?}",
                        case.id
                    );
                }
                comparable_values.insert(case.id.clone(), val.value);
            }
            Variant::Badugi => {
                let cards = case.cards.as_ref().expect("badugi case requires cards");
                let val = badugi_eval(&parse_mask(cards));
                if let Some(expected) = case.expected_low_hand_type {
                    assert_eq!(
                        val.hand_type(),
                        expected,
                        "case {} expected low hand type {}",
                        case.id,
                        expected
                    );
                }
                comparable_values.insert(case.id.clone(), val.value);
            }
            Variant::OmahaHi => {
                let hole = case.hole.as_ref().expect("omaha_hi case requires hole");
                let board = case.board.as_ref().expect("omaha_hi case requires board");
                let val = OmahaHiEvaluator::evaluate_hand(&parse_mask(hole), &parse_mask(board))
                    .unwrap()
                    .expect("omaha_hi should produce a hand value");
                if let Some(expected) = &case.expected_hand_type {
                    let expected = parse_hand_type(expected);
                    assert_eq!(
                        val.hand_type(),
                        expected as u8,
                        "case {} expected hand type {expected:?}",
                        case.id
                    );
                }
                comparable_values.insert(case.id.clone(), val.value);
            }
            Variant::OmahaHilo => {
                let hole = case.hole.as_ref().expect("omaha_hilo case requires hole");
                let board = case.board.as_ref().expect("omaha_hilo case requires board");
                let (_hi, lo) =
                    OmahaHiLoEvaluator::evaluate_hand(&parse_mask(hole), &parse_mask(board))
                        .unwrap();
                if let Some(expected) = case.expected_has_low {
                    assert_eq!(
                        lo.is_some(),
                        expected,
                        "case {} expected has_low={expected}",
                        case.id
                    );
                }
            }
            Variant::LowballA5 => {
                let cards = case.cards.as_ref().expect("lowball_a5 case requires cards");
                let mask = parse_mask(cards);
                let val = std_deck_lowball_eval(&mask, mask.num_cards());
                if let Some(expected) = case.expected_low_hand_type {
                    assert_eq!(
                        val.hand_type(),
                        expected,
                        "case {} expected low hand type {}",
                        case.id,
                        expected
                    );
                }
                comparable_values.insert(case.id.clone(), val.value);
            }
            Variant::Lowball27 => {
                let cards = case.cards.as_ref().expect("lowball_27 case requires cards");
                let mask = parse_mask(cards);
                let val = std_deck_lowball27_eval(&mask, mask.num_cards());
                if let Some(expected) = case.expected_low_hand_type {
                    assert_eq!(
                        val.hand_type(),
                        expected,
                        "case {} expected low hand type {}",
                        case.id,
                        expected
                    );
                }
                comparable_values.insert(case.id.clone(), val.value);
            }
            Variant::JokerHi => {
                let cards = case.cards.as_ref().expect("joker_hi case requires cards");
                let (mask, n) = poker_eval_rs::deck::JokerDeck::string_to_mask(cards)
                    .unwrap_or_else(|e| panic!("invalid joker cards in {}: {}", case.id, e));
                let val = EvalJoker::eval_n(mask, n);
                if let Some(expected) = &case.expected_hand_type {
                    let expected = parse_hand_type(expected);
                    assert_eq!(
                        val.hand_type(),
                        expected as u8,
                        "case {} expected hand type {expected:?}",
                        case.id
                    );
                }
                comparable_values.insert(case.id.clone(), val.value);
            }
            Variant::Manila => {
                let cards = case.cards.as_ref().expect("manila case requires cards");
                let manila_mask = ManilaDeck::string_to_mask(cards)
                    .unwrap_or_else(|e| panic!("invalid manila cards in {}: {}", case.id, e))
                    .0;
                let std_mask = manila_mask.to_std_mask();
                let val = ShortDeckEvaluator::evaluate_hand(&std_mask, &parse_mask("")).unwrap();

                if let Some(expected) = &case.expected_hand_type {
                    let expected = parse_hand_type(expected);
                    assert_eq!(
                        val.hand_type(),
                        expected as u8,
                        "case {} expected hand type {expected:?}",
                        case.id
                    );
                }
                comparable_values.insert(case.id.clone(), val.value);
            }
            Variant::Ofc => {
                let board = OFCBoard {
                    top: parse_mask(case.top.as_ref().expect("ofc case requires top")),
                    middle: parse_mask(case.middle.as_ref().expect("ofc case requires middle")),
                    bottom: parse_mask(case.bottom.as_ref().expect("ofc case requires bottom")),
                };
                let result = board.evaluate();
                if let Some(expected) = case.expected_is_foul {
                    assert_eq!(result.is_foul, expected, "case {} foul mismatch", case.id);
                }
                if let Some(expected) = case.expected_royalties {
                    assert_eq!(
                        result.royalties, expected,
                        "case {} royalties mismatch",
                        case.id
                    );
                }
            }
        }
    }

    for cmp in &dataset.comparison_cases {
        let left = *comparable_values
            .get(&cmp.left_id)
            .unwrap_or_else(|| panic!("Unknown left id {}", cmp.left_id));
        let right = *comparable_values
            .get(&cmp.right_id)
            .unwrap_or_else(|| panic!("Unknown right id {}", cmp.right_id));
        match cmp.op.as_str() {
            "gt" => assert!(left > right, "{} should be > {}", cmp.left_id, cmp.right_id),
            "lt" => assert!(left < right, "{} should be < {}", cmp.left_id, cmp.right_id),
            "eq" => assert_eq!(left, right, "{} should be == {}", cmp.left_id, cmp.right_id),
            _ => panic!("Unknown comparison op {}", cmp.op),
        }
    }
}
