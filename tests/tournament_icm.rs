use poker_eval_rs::tournament::calculate_icm;

#[test]
fn icm_equities_sum_to_paid_places_when_more_prizes_than_players() {
    let stacks = vec![100.0, 100.0];
    let prizes = vec![70.0, 30.0, 10.0];
    let equities = calculate_icm(&stacks, &prizes).unwrap();

    let total: f64 = equities.iter().sum();
    assert!((total - 100.0).abs() < 1e-9);
}

#[test]
fn icm_dominant_stack_gets_more_equity() {
    let stacks = vec![9000.0, 500.0, 500.0];
    let prizes = vec![50.0, 30.0, 20.0];
    let equities = calculate_icm(&stacks, &prizes).unwrap();

    assert!(equities[0] > equities[1]);
    assert!(equities[0] > equities[2]);
}
