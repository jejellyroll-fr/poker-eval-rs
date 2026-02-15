use poker_eval_rs::solvers::metrics::{default_variants, generate_kuhn_convergence_report};

fn main() {
    let schedule = vec![100, 500, 1_000, 5_000, 10_000];
    let variants = default_variants();
    let report = generate_kuhn_convergence_report(&schedule, &variants);
    let json = serde_json::to_string_pretty(&report).expect("serialize convergence report");
    println!("{json}");
}
