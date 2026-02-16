use poker_eval_rs::deck::traits::{CardMask, Deck};
use poker_eval_rs::deck::{DeckVariant, JokerDeck, ManilaDeck, ShortDeck};
use poker_eval_rs::evaluators::Eval;

#[test]
fn test_regression_short_deck() {
    let (m, _) = ShortDeck::string_to_mask("As Ks Qs Js Ts").unwrap();
    // In short deck, A-6-7-8-9 is a straight (if A is low) but
    // usually A-K-Q-J-T is the Broadway straight.
    let val = Eval::eval_n(&m.to_std_mask(), 5);
    assert_eq!(val.hand_type(), 8); // Straight Flush
}

#[test]
fn test_regression_manila_deck() {
    let (m, _) = ManilaDeck::string_to_mask("7s 8s 9s Ts Js").unwrap();
    let val = Eval::eval_n(&m.to_std_mask(), 5);
    assert_eq!(val.hand_type(), 8); // Straight Flush
}

#[test]
fn test_regression_joker_deck() {
    // Joker acts as a wild card or just the 53rd card.
    // In our JokerDeck implementation, it depends on the evaluator.
    let (m, _) = JokerDeck::string_to_mask("AsKsQsJsXx").unwrap();
    assert!(m.card_is_set(52)); // Joker is index 52
}

#[test]
fn test_regression_universal_deck_unification() {
    let mut u_mask = poker_eval_rs::deck::UniversalCardMask::new(DeckVariant::Standard52);
    u_mask.set(0); // 2h
    assert!(u_mask.card_is_set(0));

    let mut u_mask_short = poker_eval_rs::deck::UniversalCardMask::new(DeckVariant::Short36);
    u_mask_short.set(0); // 6h in short deck?
    assert!(u_mask_short.card_is_set(0));
}
