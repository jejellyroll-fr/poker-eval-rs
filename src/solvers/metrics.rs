//! Solver convergence metrics and reporting helpers.

use crate::solvers::cfr::CFRSolver;
use crate::solvers::games::{kuhn_nash_conv, KuhnGameState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantConfig {
    pub name: String,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub use_ecfr: bool,
    pub linear_avg_power: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationMetric {
    pub iterations: usize,
    pub nash_conv: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantReport {
    pub name: String,
    pub points: Vec<IterationMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceReport {
    pub variants: Vec<VariantReport>,
}

pub fn default_variants() -> Vec<VariantConfig> {
    vec![
        VariantConfig {
            name: "cfr".to_string(),
            alpha: 1000.0,
            beta: 1000.0,
            gamma: 0.0,
            use_ecfr: false,
            linear_avg_power: 0.0,
        },
        VariantConfig {
            name: "dcfr".to_string(),
            alpha: 1.5,
            beta: 0.0,
            gamma: 2.0,
            use_ecfr: false,
            linear_avg_power: 1.0,
        },
        VariantConfig {
            name: "ecfr".to_string(),
            alpha: 1.5,
            beta: 0.0,
            gamma: 2.0,
            use_ecfr: true,
            linear_avg_power: 1.0,
        },
    ]
}

pub fn generate_kuhn_convergence_report(
    iteration_schedule: &[usize],
    variants: &[VariantConfig],
) -> ConvergenceReport {
    let mut schedule = iteration_schedule.to_vec();
    schedule.sort_unstable();
    schedule.dedup();
    schedule.retain(|&n| n > 0);

    let initial_state = KuhnGameState::new(vec![0, 1, 2]);
    let mut reports = Vec::with_capacity(variants.len());

    for variant in variants {
        let mut solver = CFRSolver::new(2);
        solver.alpha = variant.alpha;
        solver.beta = variant.beta;
        solver.gamma = variant.gamma;
        solver.use_ecfr = variant.use_ecfr;
        solver.linear_avg_power = variant.linear_avg_power;

        let mut points = Vec::with_capacity(schedule.len());
        let mut trained = 0usize;
        for &target in &schedule {
            let delta = target.saturating_sub(trained);
            if delta > 0 {
                solver.train(&initial_state, delta);
                trained = target;
            }

            let nash_conv = kuhn_nash_conv(&|key| {
                let s = solver.strategy_for_infoset(key);
                [s[0], s[1]]
            });
            points.push(IterationMetric {
                iterations: target,
                nash_conv,
            });
        }

        reports.push(VariantReport {
            name: variant.name.clone(),
            points,
        });
    }

    ConvergenceReport { variants: reports }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_kuhn_convergence_report_shape() {
        let schedule = vec![100, 500, 1000];
        let variants = default_variants();
        let report = generate_kuhn_convergence_report(&schedule, &variants);

        assert_eq!(report.variants.len(), variants.len());
        for v in &report.variants {
            assert_eq!(v.points.len(), schedule.len());
            for p in &v.points {
                assert!(p.nash_conv.is_finite());
                assert!(p.nash_conv >= 0.0);
            }
        }
    }
}
