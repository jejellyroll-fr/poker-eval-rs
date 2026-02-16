use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poker_eval_rs::solvers::betting::{DeckProfile, GameConfig, PokerVariant, TreeProfile};
use poker_eval_rs::solvers::core::{CfrPlusSolver, ExploitabilityPoint};
use poker_eval_rs::solvers::variant_tree::VariantGameTree;
use std::fs;
use std::path::Path;
use std::sync::Once;

static WRITE_CURVES_ONCE: Once = Once::new();

fn variant_name(v: PokerVariant) -> &'static str {
    match v {
        PokerVariant::Holdem => "holdem",
        PokerVariant::Omaha => "omaha",
        PokerVariant::ShortDeck => "shortdeck",
    }
}

fn make_cfg(variant: PokerVariant, players: usize) -> GameConfig {
    let mut cfg = GameConfig::no_limit(players, 12, 1, 2);
    cfg.variant = variant;
    cfg.tree_profile = TreeProfile::Tight;
    cfg.raise_cap = Some(2);
    if variant == PokerVariant::ShortDeck {
        cfg.deck_profile = DeckProfile::Short36;
    }
    cfg
}

fn train_curve(
    variant: PokerVariant,
    players: usize,
    iters: usize,
    checkpoint_every: usize,
) -> Vec<ExploitabilityPoint> {
    let cfg = make_cfg(variant, players);
    let root = VariantGameTree::new(cfg, 0);
    let mut solver = CfrPlusSolver::new(root);
    solver.train_with_n_player_exploitability(iters, checkpoint_every)
}

fn write_curves_csv(path: &Path) {
    let mut lines = Vec::new();
    lines.push("variant,players,iteration,exploitability".to_string());
    for variant in [
        PokerVariant::Holdem,
        PokerVariant::Omaha,
        PokerVariant::ShortDeck,
    ] {
        for players in [3_usize, 4_usize] {
            let points = train_curve(variant, players, 300, 20);
            for p in points {
                lines.push(format!(
                    "{},{},{},{}",
                    variant_name(variant),
                    players,
                    p.iteration,
                    p.exploitability
                ));
            }
        }
    }

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, lines.join("\n"));
}

fn bench_solver_core_multiway(c: &mut Criterion) {
    WRITE_CURVES_ONCE.call_once(|| {
        write_curves_csv(Path::new("benches/results/solver_core_multiway_curve.csv"))
    });

    let mut group = c.benchmark_group("solver_core_multiway_train");
    for variant in [
        PokerVariant::Holdem,
        PokerVariant::Omaha,
        PokerVariant::ShortDeck,
    ] {
        for players in [3_usize, 4_usize] {
            group.bench_function(
                format!("{}_{}p_200it", variant_name(variant), players),
                |b| {
                    b.iter(|| {
                        let points = train_curve(variant, players, 200, 20);
                        black_box(points.last().map(|p| p.exploitability).unwrap_or(0.0));
                    })
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_solver_core_multiway);
criterion_main!(benches);
