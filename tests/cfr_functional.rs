use poker_eval_rs::solvers::cfr::CFRSolver;
use poker_eval_rs::solvers::games::KuhnGameState;

#[test]
fn test_cfr_kuhn_convergence() {
    let mut solver = CFRSolver::new(2);
    let initial_state = KuhnGameState::new(vec![0, 1, 2]); // J, Q, K

    // Train for 100,000 iterations for stable convergence
    solver.train(&initial_state, 100_000);

    // Check strategies
    // Player 1 with Jack (0)
    let node_j = solver.nodes.get("0:").expect("Node for Jack should exist");
    let strat_j = node_j.get_strategy(false);
    // Standard Kuhn: P1 should check J most of the time.
    // Optimal alpha is in [0, 1/3] for P1 betting J.
    assert!(
        strat_j[1] <= 0.45,
        "P1 should not bet J more than 45% (got {:.2}%)",
        strat_j[1] * 100.0
    );

    // Player 1 with Queen (1)
    let node_q = solver.nodes.get("1:").expect("Node for Queen should exist");
    let strat_q = node_q.get_strategy(false);
    // P1 should always check Q
    assert!(
        strat_q[0] > 0.90,
        "P1 should check Q most of the time (got {:.2}%)",
        strat_q[0] * 100.0
    );

    // Player 1 with King (2)
    let node_k = solver.nodes.get("2:").expect("Node for King should exist");
    let strat_k = node_k.get_strategy(false);
    // P1 should bet K (usually 3*alpha)
    assert!(
        strat_k[1] > 0.2,
        "P1 should bet K sometimes (got {:.2}%)",
        strat_k[1] * 100.0
    );
}

#[test]
fn test_dcfr_kuhn_convergence() {
    let mut solver = CFRSolver::new(2);
    solver.alpha = 1.5;
    solver.beta = 0.0;
    solver.gamma = 2.0;
    let initial_state = KuhnGameState::new(vec![0, 1, 2]);

    solver.train(&initial_state, 50_000); // DCFR converges faster

    // Similar checks
    let node_q = solver.nodes.get("1:").unwrap();
    let strat_q = node_q.get_strategy(false);
    assert!(
        strat_q[0] > 0.85,
        "DCFR P1 should check Q most of the time (got {:.2}%)",
        strat_q[0] * 100.0
    );
}
