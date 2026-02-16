use clap::Parser;
use poker_eval_rs::solvers::betting::{DeckProfile, GameConfig, PokerVariant, TreeProfile};
use poker_eval_rs::solvers::core::CfrPlusSolver;
use poker_eval_rs::solvers::variant_tree::VariantGameTree;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "solver_traversal_cache_report")]
#[command(about = "Run cache on/off traversal benchmark matrix and write CSV/Markdown report")]
struct Args {
    #[arg(long, default_value = "holdem,omaha,shortdeck")]
    variants: String,
    #[arg(long, default_value = "3,4,5,6")]
    players: String,
    #[arg(long, default_value_t = 2)]
    repeats: usize,
    #[arg(long, default_value_t = 20)]
    iterations: usize,
    #[arg(long, default_value_t = 8)]
    stack: u64,
    #[arg(long, default_value_t = 1)]
    raise_cap: usize,
    #[arg(long, default_value_t = 2)]
    chance_scenarios: usize,
    #[arg(long, default_value_t = 7)]
    chance_stride: usize,
    #[arg(long, default_value = "docs/reports/solver_traversal_cache_runs.csv")]
    runs_csv: String,
    #[arg(long, default_value = "docs/reports/solver_traversal_cache_report.md")]
    md_out: String,
}

#[derive(Clone)]
struct Row {
    variant: String,
    players: usize,
    cache_on: bool,
    repeat: usize,
    time_sec: f64,
    it_per_sec: f64,
}

fn parse_variants(input: &str) -> Vec<PokerVariant> {
    let mut out = Vec::new();
    for token in input.split(',').map(|s| s.trim().to_lowercase()) {
        match token.as_str() {
            "holdem" | "he" => out.push(PokerVariant::Holdem),
            "omaha" | "plo" => out.push(PokerVariant::Omaha),
            "shortdeck" | "sd" | "6plus" => out.push(PokerVariant::ShortDeck),
            _ => {}
        }
    }
    if out.is_empty() {
        vec![
            PokerVariant::Holdem,
            PokerVariant::Omaha,
            PokerVariant::ShortDeck,
        ]
    } else {
        out
    }
}

fn parse_players(input: &str) -> Vec<usize> {
    let mut out = input
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .filter(|n| *n >= 2)
        .collect::<Vec<_>>();
    out.sort_unstable();
    out.dedup();
    if out.is_empty() {
        vec![3, 4, 5, 6]
    } else {
        out
    }
}

fn variant_name(v: PokerVariant) -> &'static str {
    match v {
        PokerVariant::Holdem => "holdem",
        PokerVariant::Omaha => "omaha",
        PokerVariant::ShortDeck => "shortdeck",
    }
}

fn ensure_parent(path: &str) {
    let p = PathBuf::from(path);
    if let Some(parent) = p.parent() {
        let _ = fs::create_dir_all(parent);
    }
}

fn run_once(
    variant: PokerVariant,
    players: usize,
    cache_on: bool,
    args: &Args,
    repeat: usize,
) -> Row {
    let mut cfg = GameConfig::no_limit(players, args.stack, 1, 2);
    cfg.variant = variant;
    cfg.tree_profile = TreeProfile::Tight;
    cfg.raise_cap = Some(args.raise_cap);
    cfg.chance_scenarios = args.chance_scenarios;
    cfg.chance_stride = args.chance_stride;
    if variant == PokerVariant::ShortDeck {
        cfg.deck_profile = DeckProfile::Short36;
    }
    let root = VariantGameTree::new(cfg, 0);
    let mut solver = CfrPlusSolver::new(root);
    solver.config.cache_opponent_strategies = cache_on;
    solver.config.cache_subtree_actions = cache_on;
    solver.config.cache_subtree_values = false;

    let t0 = Instant::now();
    solver.train(args.iterations);
    let sec = t0.elapsed().as_secs_f64().max(1e-9);
    let it_per_sec = args.iterations as f64 / sec;

    Row {
        variant: variant_name(variant).to_string(),
        players,
        cache_on,
        repeat,
        time_sec: sec,
        it_per_sec,
    }
}

fn main() {
    let args = Args::parse();
    let variants = parse_variants(&args.variants);
    let players = parse_players(&args.players);

    let mut rows = Vec::new();
    for v in variants {
        for p in &players {
            for r in 0..args.repeats.max(1) {
                rows.push(run_once(v, *p, false, &args, r));
                rows.push(run_once(v, *p, true, &args, r));
            }
        }
    }

    let mut lines = Vec::new();
    lines.push("variant,players,cache_on,repeat,time_sec,it_per_sec".to_string());
    for r in &rows {
        lines.push(format!(
            "{},{},{},{},{},{}",
            r.variant, r.players, r.cache_on, r.repeat, r.time_sec, r.it_per_sec
        ));
    }

    let mut grouped: HashMap<(String, usize, bool), Vec<&Row>> = HashMap::new();
    for r in &rows {
        grouped
            .entry((r.variant.clone(), r.players, r.cache_on))
            .or_default()
            .push(r);
    }

    let mut md = Vec::new();
    md.push("# Solver Traversal Cache Report".to_string());
    md.push(String::new());
    md.push(format!(
        "- Iterations: `{}` | Repeats: `{}` | Stack: `{}` | Raise cap: `{}` | Chance scenarios: `{}` | Chance stride: `{}`",
        args.iterations,
        args.repeats,
        args.stack,
        args.raise_cap,
        args.chance_scenarios,
        args.chance_stride
    ));
    md.push(String::new());
    md.push("| Variant | Players | Mean time off (s) | Mean time on (s) | Speedup (off/on) | Mean it/s off | Mean it/s on |".to_string());
    md.push("|---|---:|---:|---:|---:|---:|---:|".to_string());

    let mut keys = grouped
        .keys()
        .map(|(v, p, _)| (v.clone(), *p))
        .collect::<Vec<_>>();
    keys.sort();
    keys.dedup();
    for (v, p) in keys {
        let off = grouped
            .get(&(v.clone(), p, false))
            .cloned()
            .unwrap_or_default();
        let on = grouped
            .get(&(v.clone(), p, true))
            .cloned()
            .unwrap_or_default();
        if off.is_empty() || on.is_empty() {
            continue;
        }
        let off_t = off.iter().map(|r| r.time_sec).sum::<f64>() / off.len() as f64;
        let on_t = on.iter().map(|r| r.time_sec).sum::<f64>() / on.len() as f64;
        let off_ips = off.iter().map(|r| r.it_per_sec).sum::<f64>() / off.len() as f64;
        let on_ips = on.iter().map(|r| r.it_per_sec).sum::<f64>() / on.len() as f64;
        let speedup = off_t / on_t.max(1e-9);
        md.push(format!(
            "| {} | {} | {:.4} | {:.4} | {:.3}x | {:.2} | {:.2} |",
            v, p, off_t, on_t, speedup, off_ips, on_ips
        ));
    }

    ensure_parent(&args.runs_csv);
    ensure_parent(&args.md_out);
    let _ = fs::write(&args.runs_csv, lines.join("\n"));
    let _ = fs::write(&args.md_out, md.join("\n"));

    println!("Wrote {}", args.runs_csv);
    println!("Wrote {}", args.md_out);
}
