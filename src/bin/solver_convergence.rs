use clap::Parser;
use poker_eval_rs::solvers::betting::{DeckProfile, GameConfig, PokerVariant, TreeProfile};
use poker_eval_rs::solvers::core::CfrPlusSolver;
use poker_eval_rs::solvers::variant_tree::VariantGameTree;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "solver_convergence")]
#[command(about = "Run solver-core convergence and write CSV/Markdown reports")]
struct Args {
    #[arg(long, default_value = "holdem,omaha,shortdeck")]
    variants: String,

    #[arg(long, default_value = "3,4")]
    players: String,

    #[arg(long, default_value_t = 600)]
    iterations: usize,

    #[arg(long, default_value_t = 50)]
    checkpoint_every: usize,

    #[arg(long, default_value = "tight")]
    tree_profile: String,

    #[arg(long, default_value_t = 12)]
    stack: u64,

    #[arg(long, default_value_t = 3)]
    raise_cap: usize,

    #[arg(long, default_value_t = 6)]
    chance_scenarios: usize,

    #[arg(long, default_value_t = false)]
    fast: bool,

    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    cache_opponent_strategies: bool,

    #[arg(long, default_value = "docs/reports/solver_convergence.csv")]
    csv_out: String,

    #[arg(long, default_value = "docs/reports/solver_convergence.md")]
    md_out: String,
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
        vec![3, 4]
    } else {
        out
    }
}

fn parse_profile(input: &str) -> TreeProfile {
    match input.trim().to_lowercase().as_str() {
        "tight" => TreeProfile::Tight,
        "wide" => TreeProfile::Wide,
        _ => TreeProfile::Standard,
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

fn make_config(variant: PokerVariant, players: usize, profile: TreeProfile) -> GameConfig {
    let mut cfg = GameConfig::no_limit(players, 12, 1, 2);
    cfg.variant = variant;
    cfg.tree_profile = profile;
    cfg.raise_cap = Some(3);
    if variant == PokerVariant::ShortDeck {
        cfg.deck_profile = DeckProfile::Short36;
    }
    cfg
}

fn main() {
    let args = Args::parse();
    let variants = parse_variants(&args.variants);
    let players = parse_players(&args.players);
    let mut profile = parse_profile(&args.tree_profile);
    let mut iterations = args.iterations;
    let mut checkpoint_every = args.checkpoint_every;
    let mut stack = args.stack;
    let mut raise_cap = args.raise_cap;
    let mut chance_scenarios = args.chance_scenarios;

    if args.fast {
        profile = TreeProfile::Tight;
        iterations = iterations.min(200);
        checkpoint_every = checkpoint_every.clamp(1, 20);
        stack = stack.min(8);
        raise_cap = raise_cap.min(1);
        chance_scenarios = chance_scenarios.clamp(1, 2);
    }

    let mut csv_lines = Vec::new();
    csv_lines.push("variant,players,iteration,exploitability".to_string());

    let mut md = Vec::new();
    md.push("# Solver Convergence Report".to_string());
    md.push(String::new());
    md.push(format!(
        "- Iterations: `{}` | Checkpoint: `{}` | Tree profile: `{:?}`",
        iterations, checkpoint_every, profile
    ));
    md.push(format!(
        "- Stack: `{}` | Raise cap: `{}` | Chance scenarios: `{}` | Fast mode: `{}`",
        stack, raise_cap, chance_scenarios, args.fast
    ));
    md.push(String::new());
    md.push(
        "| Variant | Players | Start Exploitability | Final Exploitability | Delta |".to_string(),
    );
    md.push("|---|---:|---:|---:|---:|".to_string());

    for variant in variants {
        for n_players in &players {
            let mut cfg = make_config(variant, *n_players, profile);
            cfg.stack_start = stack;
            cfg.raise_cap = Some(raise_cap);
            cfg.chance_scenarios = chance_scenarios;
            let root = VariantGameTree::new(cfg, 0);
            let mut solver = CfrPlusSolver::new(root);
            solver.config.cache_opponent_strategies = args.cache_opponent_strategies;
            solver.config.cache_subtree_actions = args.cache_opponent_strategies;
            solver.config.cache_subtree_values = false;
            let points = solver.train_with_n_player_exploitability(iterations, checkpoint_every);

            for p in &points {
                csv_lines.push(format!(
                    "{},{},{},{}",
                    variant_name(variant),
                    n_players,
                    p.iteration,
                    p.exploitability
                ));
            }

            let (start, end) = if points.is_empty() {
                (0.0, 0.0)
            } else {
                (
                    points.first().map(|p| p.exploitability).unwrap_or(0.0),
                    points.last().map(|p| p.exploitability).unwrap_or(0.0),
                )
            };
            md.push(format!(
                "| {} | {} | {:.6} | {:.6} | {:.6} |",
                variant_name(variant),
                n_players,
                start,
                end,
                end - start
            ));
        }
    }

    ensure_parent(&args.csv_out);
    ensure_parent(&args.md_out);
    let _ = fs::write(&args.csv_out, csv_lines.join("\n"));
    let _ = fs::write(&args.md_out, md.join("\n"));

    println!("Wrote {}", args.csv_out);
    println!("Wrote {}", args.md_out);
}
