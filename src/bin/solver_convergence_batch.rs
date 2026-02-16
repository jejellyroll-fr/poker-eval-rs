use clap::Parser;
use poker_eval_rs::solvers::betting::{DeckProfile, GameConfig, PokerVariant, TreeProfile};
use poker_eval_rs::solvers::core::CfrPlusSolver;
use poker_eval_rs::solvers::variant_tree::VariantGameTree;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "solver_convergence_batch")]
#[command(about = "Batch convergence pipeline with aggregation and profile comparisons")]
struct Args {
    #[arg(long, default_value = "holdem,omaha,shortdeck")]
    variants: String,
    #[arg(long, default_value = "3,4")]
    players: String,
    #[arg(long, default_value = "tight,standard,wide")]
    profiles: String,
    #[arg(long, default_value_t = 2)]
    repeats: usize,
    #[arg(long, default_value_t = 200)]
    iterations: usize,
    #[arg(long, default_value_t = 20)]
    checkpoint_every: usize,
    #[arg(long, default_value_t = 8)]
    stack: u64,
    #[arg(long, default_value_t = 1)]
    raise_cap: usize,
    #[arg(long, default_value_t = 2)]
    chance_scenarios: usize,
    #[arg(long, default_value_t = 7)]
    chance_stride: usize,
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    cache_opponent_strategies: bool,
    #[arg(long, default_value = "docs/reports/solver_batch_runs.csv")]
    runs_csv: String,
    #[arg(long, default_value = "docs/reports/solver_batch_agg.csv")]
    agg_csv: String,
    #[arg(long, default_value = "docs/reports/solver_batch_report.md")]
    md_out: String,
}

#[derive(Clone)]
struct Job {
    variant: PokerVariant,
    players: usize,
    profile: TreeProfile,
    repeat: usize,
}

#[derive(Clone)]
struct RunRow {
    variant: String,
    players: usize,
    profile: String,
    repeat: usize,
    iteration: usize,
    exploitability: f64,
    time_sec: f64,
}

type FinalKey = (String, usize, String, usize);
type FinalVal = (usize, f64, f64);

#[derive(Clone)]
struct AggEntry {
    variant: String,
    players: usize,
    profile: String,
    runs: usize,
    exp_mean: f64,
    exp_min: f64,
    exp_max: f64,
    time_mean: f64,
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

fn parse_profiles(input: &str) -> Vec<TreeProfile> {
    let mut out = Vec::new();
    for token in input.split(',').map(|s| s.trim().to_lowercase()) {
        match token.as_str() {
            "tight" => out.push(TreeProfile::Tight),
            "wide" => out.push(TreeProfile::Wide),
            "standard" | "std" => out.push(TreeProfile::Standard),
            _ => {}
        }
    }
    if out.is_empty() {
        vec![TreeProfile::Tight, TreeProfile::Standard, TreeProfile::Wide]
    } else {
        out
    }
}

fn profile_name(p: TreeProfile) -> &'static str {
    match p {
        TreeProfile::Tight => "tight",
        TreeProfile::Standard => "standard",
        TreeProfile::Wide => "wide",
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

fn run_job(job: &Job, args: &Args) -> Vec<RunRow> {
    let mut cfg = GameConfig::no_limit(job.players, args.stack, 1, 2);
    cfg.variant = job.variant;
    cfg.tree_profile = job.profile;
    cfg.raise_cap = Some(args.raise_cap);
    cfg.chance_scenarios = args.chance_scenarios;
    cfg.chance_stride = args.chance_stride;
    if job.variant == PokerVariant::ShortDeck {
        cfg.deck_profile = DeckProfile::Short36;
    }
    let root = VariantGameTree::new(cfg, 0);
    let mut solver = CfrPlusSolver::new(root);
    solver.config.cache_opponent_strategies = args.cache_opponent_strategies;
    solver.config.cache_subtree_actions = args.cache_opponent_strategies;
    solver.config.cache_subtree_values = false;

    let t0 = Instant::now();
    let points = solver.train_with_n_player_exploitability(args.iterations, args.checkpoint_every);
    let sec = t0.elapsed().as_secs_f64();
    points
        .into_iter()
        .map(|p| RunRow {
            variant: variant_name(job.variant).to_string(),
            players: job.players,
            profile: profile_name(job.profile).to_string(),
            repeat: job.repeat,
            iteration: p.iteration,
            exploitability: p.exploitability,
            time_sec: sec,
        })
        .collect()
}

fn main() {
    let args = Args::parse();
    let variants = parse_variants(&args.variants);
    let players = parse_players(&args.players);
    let profiles = parse_profiles(&args.profiles);

    let mut jobs = Vec::new();
    for v in variants {
        for p in &players {
            for prof in &profiles {
                for r in 0..args.repeats.max(1) {
                    jobs.push(Job {
                        variant: v,
                        players: *p,
                        profile: *prof,
                        repeat: r,
                    });
                }
            }
        }
    }

    #[cfg(feature = "parallel")]
    let rows_nested = jobs
        .par_iter()
        .map(|j| run_job(j, &args))
        .collect::<Vec<_>>();
    #[cfg(not(feature = "parallel"))]
    let rows_nested = jobs.iter().map(|j| run_job(j, &args)).collect::<Vec<_>>();

    let rows = rows_nested.into_iter().flatten().collect::<Vec<_>>();

    let mut runs_csv = Vec::new();
    runs_csv.push("variant,players,profile,repeat,iteration,exploitability,time_sec".to_string());
    for r in &rows {
        runs_csv.push(format!(
            "{},{},{},{},{},{},{}",
            r.variant, r.players, r.profile, r.repeat, r.iteration, r.exploitability, r.time_sec
        ));
    }

    // Aggregate on final checkpoint per run.
    let mut final_per_run: HashMap<FinalKey, FinalVal> = HashMap::new();
    for r in &rows {
        let k = (r.variant.clone(), r.players, r.profile.clone(), r.repeat);
        let e = final_per_run
            .entry(k)
            .or_insert((r.iteration, r.exploitability, r.time_sec));
        if r.iteration >= e.0 {
            *e = (r.iteration, r.exploitability, r.time_sec);
        }
    }

    let mut grouped: HashMap<(String, usize, String), Vec<(f64, f64)>> = HashMap::new();
    for ((v, p, prof, _rep), (_it, exp, t)) in final_per_run {
        grouped.entry((v, p, prof)).or_default().push((exp, t));
    }

    let mut agg_rows = Vec::new();
    let mut agg_entries = Vec::new();
    agg_rows.push("variant,players,profile,runs,exp_mean,exp_min,exp_max,time_mean".to_string());
    let mut best_profile: HashMap<(String, usize), (String, f64)> = HashMap::new();
    let mut best_efficiency: HashMap<(String, usize), (String, f64)> = HashMap::new();

    for ((v, p, prof), vals) in grouped {
        let n = vals.len().max(1) as f64;
        let exp_mean = vals.iter().map(|x| x.0).sum::<f64>() / n;
        let exp_min = vals.iter().map(|x| x.0).fold(f64::INFINITY, f64::min);
        let exp_max = vals.iter().map(|x| x.0).fold(f64::NEG_INFINITY, f64::max);
        let time_mean = vals.iter().map(|x| x.1).sum::<f64>() / n;
        agg_entries.push(AggEntry {
            variant: v.clone(),
            players: p,
            profile: prof.clone(),
            runs: vals.len(),
            exp_mean,
            exp_min,
            exp_max,
            time_mean,
        });

        let key = (v.clone(), p);
        let bp = best_profile.entry(key).or_insert((prof.clone(), exp_mean));
        if exp_mean < bp.1 {
            *bp = (prof.clone(), exp_mean);
        }

        let eff_score = exp_mean * time_mean.max(1e-9);
        let key_eff = (v.clone(), p);
        let be = best_efficiency
            .entry(key_eff)
            .or_insert((prof.clone(), eff_score));
        if eff_score < be.1 {
            *be = (prof, eff_score);
        }
    }

    for a in &agg_entries {
        agg_rows.push(format!(
            "{},{},{},{},{},{},{},{}",
            a.variant, a.players, a.profile, a.runs, a.exp_mean, a.exp_min, a.exp_max, a.time_mean
        ));
    }

    let mut md = Vec::new();
    md.push("# Solver Batch Convergence Report".to_string());
    md.push(String::new());
    md.push(format!(
        "- Iterations: `{}` | Checkpoint: `{}` | Repeats: `{}`",
        args.iterations, args.checkpoint_every, args.repeats
    ));
    md.push(format!(
        "- Stack: `{}` | Raise cap: `{}` | Chance scenarios: `{}` | Chance stride: `{}` | Opponent strategy cache: `{}`",
        args.stack,
        args.raise_cap,
        args.chance_scenarios,
        args.chance_stride,
        args.cache_opponent_strategies
    ));
    md.push(String::new());
    md.push("| Variant | Players | Best Profile | Mean Final Exploitability |".to_string());
    md.push("|---|---:|---|---:|".to_string());
    let mut best_keys = best_profile.keys().cloned().collect::<Vec<_>>();
    best_keys.sort();
    for (v, p) in best_keys {
        if let Some((prof, exp)) = best_profile.get(&(v.clone(), p)) {
            md.push(format!("| {} | {} | {} | {:.6} |", v, p, prof, exp));
        }
    }
    md.push(String::new());
    md.push("| Variant | Players | Recommended (Cost/Quality) | Score (exp*time) |".to_string());
    md.push("|---|---:|---|---:|".to_string());
    let mut eff_keys = best_efficiency.keys().cloned().collect::<Vec<_>>();
    eff_keys.sort();
    for (v, p) in eff_keys {
        if let Some((prof, score)) = best_efficiency.get(&(v.clone(), p)) {
            md.push(format!("| {} | {} | {} | {:.6} |", v, p, prof, score));
        }
    }
    md.push(String::new());
    md.push("| Variant | Players | Recommended (Cost/Quality) | Mean Exp | Mean Time (s) | Delta Exp vs BestExp | Speedup vs Slowest |".to_string());
    md.push("|---|---:|---|---:|---:|---:|---:|".to_string());
    let mut detail_keys = best_efficiency.keys().cloned().collect::<Vec<_>>();
    detail_keys.sort();
    for (v, p) in detail_keys {
        if let Some((prof, _)) = best_efficiency.get(&(v.clone(), p)) {
            let stats = agg_entries
                .iter()
                .filter(|a| a.variant == v && a.players == p)
                .collect::<Vec<_>>();
            if stats.is_empty() {
                continue;
            }
            let best_exp = stats
                .iter()
                .map(|a| a.exp_mean)
                .fold(f64::INFINITY, f64::min);
            let slowest = stats
                .iter()
                .map(|a| a.time_mean)
                .fold(f64::NEG_INFINITY, f64::max)
                .max(1e-9);
            if let Some(chosen) = stats.iter().find(|a| a.profile == *prof) {
                let delta_exp = chosen.exp_mean - best_exp;
                let speedup = slowest / chosen.time_mean.max(1e-9);
                md.push(format!(
                    "| {} | {} | {} | {:.6} | {:.4} | {:.6} | {:.3}x |",
                    v, p, prof, chosen.exp_mean, chosen.time_mean, delta_exp, speedup
                ));
            }
        }
    }

    ensure_parent(&args.runs_csv);
    ensure_parent(&args.agg_csv);
    ensure_parent(&args.md_out);
    let _ = fs::write(&args.runs_csv, runs_csv.join("\n"));
    let _ = fs::write(&args.agg_csv, agg_rows.join("\n"));
    let _ = fs::write(&args.md_out, md.join("\n"));

    println!("Wrote {}", args.runs_csv);
    println!("Wrote {}", args.agg_csv);
    println!("Wrote {}", args.md_out);
}
