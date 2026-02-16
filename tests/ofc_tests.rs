use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::solvers::ofc_game::{OFCGameState, OFCPlacementSolver};

#[test]
fn test_ofc_game_state_placement() {
    let deck = (0..52).collect::<Vec<_>>();
    let mut state = OFCGameState::new(deck);

    // Place 5 cards
    let initial_cards = vec![0, 1, 2, 3, 4]; // 2h 3h 4h 5h 6h
    for (i, &card) in initial_cards.iter().enumerate() {
        let row = if i < 2 {
            2
        } else if i < 4 {
            1
        } else {
            0
        };
        state.place_card(card, row);
    }

    assert_eq!(state.cards_placed, 5);
    let (t, m, b) = state.row_counts();
    assert_eq!(t, 1);
    assert_eq!(m, 2);
    assert_eq!(b, 2);
}

#[test]
fn test_ofc_greedy_solver_one_card() {
    let deck = (0..52).collect::<Vec<_>>();
    let mut state = OFCGameState::new(deck);

    // We want to place Ah (index 12).
    // In a new board, it should probably go to bottom row for strength.
    let best_rows = OFCPlacementSolver::find_best_placement(&state, &[12]);
    assert_eq!(best_rows.len(), 1);
    // Our heuristic currently prefers bottom (row 2)
    assert_eq!(best_rows[0], 2);
}
