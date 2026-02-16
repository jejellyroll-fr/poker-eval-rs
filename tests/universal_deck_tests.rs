use poker_eval_rs::deck::*;

#[test]
fn test_universal_card_mask_std() {
    let mut mask = UniversalCardMask::new(DeckVariant::Standard52);
    mask.set(0); // 2h
    mask.set(51); // As

    assert_eq!(mask.num_cards(), 2);
    assert_eq!(mask.mask_to_string(), "2h As");

    let std = mask.to_std_mask().unwrap();
    assert!(std.card_is_set(0));
    assert!(std.card_is_set(51));
}

#[test]
fn test_universal_card_mask_short() {
    let mut mask = UniversalCardMask::new(DeckVariant::Short36);
    // Short deck index 0 is mapped to StdDeck index 4 (6h)
    mask.set(0);

    assert_eq!(mask.mask_to_string(), "6h");

    let std = mask.to_std_mask().unwrap();
    assert!(std.card_is_set(4));
}

#[test]
fn test_universal_deck_variants() {
    let std_deck = UniversalDeck::new(DeckVariant::Standard52);
    let (m1, n1) = std_deck.variant_string_to_mask("As Ks").unwrap();
    assert_eq!(n1, 2);
    assert_eq!(m1.variant, DeckVariant::Standard52);
    assert_eq!(m1.mask_to_string(), "Ks As");

    let short_deck = UniversalDeck::new(DeckVariant::Short36);
    let (m2, n2) = short_deck.variant_string_to_mask("6h 7h").unwrap();
    assert_eq!(n2, 2);
    assert_eq!(m2.variant, DeckVariant::Short36);
    assert_eq!(m2.mask_to_string(), "6h 7h");
}

#[test]
fn test_universal_mask_not() {
    let mut mask = UniversalCardMask::new(DeckVariant::Standard52);
    mask.set(0);
    mask.not();
    assert!(!mask.card_is_set(0));
    assert!(mask.card_is_set(1));
}
