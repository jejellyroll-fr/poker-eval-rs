use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poker_eval_rs::solvers::cfr::CFRSolver;
use poker_eval_rs::solvers::games::{kuhn_nash_conv, KuhnGameState};
use poker_eval_rs::solvers::metrics::{default_variants, VariantConfig};

fn kuhn_policy_error(solver: &CFRSolver) -> f64 {
    kuhn_nash_conv(&|key| {
        let s = solver.strategy_for_infoset(key);
        [s[0], s[1]]
    })
}

fn train_kuhn(variant: &VariantConfig, iterations: usize) -> CFRSolver {
    let initial_state = KuhnGameState::new(vec![0, 1, 2]); // J, Q, K
    let mut solver = CFRSolver::new(2);
    solver.alpha = variant.alpha;
    solver.beta = variant.beta;
    solver.gamma = variant.gamma;
    solver.use_ecfr = variant.use_ecfr;
    solver.linear_avg_power = variant.linear_avg_power;
    solver.train(&initial_state, iterations);
    solver
}

fn bench_cfr_variants(c: &mut Criterion) {
    let variants = default_variants();

    let mut train_group = c.benchmark_group("cfr_training");
    for variant in &variants {
        train_group.bench_function(format!("{}_10k", variant.name), |b| {
            b.iter(|| {
                let solver = train_kuhn(variant, 10_000);
                black_box(solver.nodes.len());
            })
        });
    }
    train_group.finish();

    let mut quality_group = c.benchmark_group("cfr_convergence_quality");
    for variant in &variants {
        quality_group.bench_function(format!("{}_score_10k", variant.name), |b| {
            b.iter(|| {
                let solver = train_kuhn(variant, 10_000);
                let err = kuhn_policy_error(&solver);
                black_box(err);
            })
        });
    }
    quality_group.finish();
}

criterion_group!(benches, bench_cfr_variants);
criterion_main!(benches);
