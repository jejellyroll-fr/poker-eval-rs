use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use poker_eval_rs::solvers::betting::{DeckProfile, GameConfig, PokerVariant, TreeProfile};
use poker_eval_rs::solvers::core::CfrPlusSolver;
use poker_eval_rs::solvers::variant_tree::VariantGameTree;
use std::fs;
use std::path::Path;
use std::sync::Once;
use std::time::Instant;

static WRITE_ONCE: Once = Once::new();

fn variant_name(v: PokerVariant) -> &'static str {
    match v {
        PokerVariant::Holdem => "holdem",
        PokerVariant::Omaha => "omaha",
        PokerVariant::ShortDeck => "shortdeck",
    }
}

fn make_cfg(variant: PokerVariant, players: usize) -> GameConfig {
    let mut cfg = GameConfig::no_limit(players, 8, 1, 2);
    cfg.variant = variant;
    cfg.tree_profile = TreeProfile::Tight;
    cfg.raise_cap = Some(1);
    cfg.chance_scenarios = 2;
    cfg.chance_stride = 7;
    if variant == PokerVariant::ShortDeck {
        cfg.deck_profile = DeckProfile::Short36;
    }
    cfg
}

fn run_train(variant: PokerVariant, players: usize, iters: usize, cache_on: bool) -> f64 {
    let cfg = make_cfg(variant, players);
    let root = VariantGameTree::new(cfg, 0);
    let mut solver = CfrPlusSolver::new(root);
    solver.config.cache_opponent_strategies = cache_on;
    solver.config.cache_subtree_actions = cache_on;
    solver.config.cache_subtree_values = false;

    let t0 = Instant::now();
    solver.train(iters);
    let sec = t0.elapsed().as_secs_f64();
    black_box(solver.iteration);
    sec
}

fn write_snapshot(path: &Path) {
    let mut lines = Vec::new();
    lines.push("variant,players,iters,cache_on,time_sec".to_string());
    for variant in [
        PokerVariant::Holdem,
        PokerVariant::Omaha,
        PokerVariant::ShortDeck,
    ] {
        for players in 3..=6 {
            for cache_on in [true, false] {
                let sec = run_train(variant, players, 10, cache_on);
                lines.push(format!(
                    "{},{},{},{},{}",
                    variant_name(variant),
                    players,
                    10,
                    cache_on,
                    sec
                ));
            }
        }
    }
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, lines.join("\n"));
}

fn bench_solver_core_traversal_cache(c: &mut Criterion) {
    WRITE_ONCE.call_once(|| {
        write_snapshot(Path::new(
            "benches/results/solver_core_traversal_cache_snapshot.csv",
        ))
    });

    let mut group = c.benchmark_group("solver_core_traversal_cache_train");
    for variant in [
        PokerVariant::Holdem,
        PokerVariant::Omaha,
        PokerVariant::ShortDeck,
    ] {
        for players in 3..=6 {
            for cache_on in [true, false] {
                let label = if cache_on { "cache_on" } else { "cache_off" };
                group.bench_with_input(
                    BenchmarkId::new(format!("{}_{}p", variant_name(variant), players), label),
                    &(variant, players, cache_on),
                    |b, (v, p, co)| {
                        b.iter(|| {
                            let sec = run_train(*v, *p, 12, *co);
                            black_box(sec);
                        })
                    },
                );
            }
        }
    }
    group.finish();
}

criterion_group!(benches, bench_solver_core_traversal_cache);
criterion_main!(benches);
