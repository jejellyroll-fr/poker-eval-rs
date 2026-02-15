use poker_eval_rs::gpu::GPUEvaluator;
use poker_eval_rs::solvers::cfr::CFRSolver;

fn main() {
    let mut gpu = match pollster::block_on(GPUEvaluator::new()) {
        Some(g) => g,
        None => {
            eprintln!("No compatible GPU adapter found.");
            return;
        }
    };

    let mut solver = CFRSolver::new(2);
    solver.train_push_fold_with_gpu(20_000, 8_192, &mut gpu);

    let root = solver.strategy_for_infoset("pf:");
    println!("Trained iterations: {}", solver.iteration);
    println!("Root strategy [fold/check, shove]: {:?}", root);
}
