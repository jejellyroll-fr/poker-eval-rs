use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use poker_eval_rs::deck::StdDeckCardMask;
use poker_eval_rs::evaluators::{HandEvaluator, HoldemEvaluator};
use poker_eval_rs::gpu::GPUEvaluator;
use rand::prelude::*;

fn generate_random_masks(n: usize) -> Vec<u64> {
    let mut rng = thread_rng();
    let mut masks = Vec::with_capacity(n);
    let all_cards: Vec<usize> = (0..52).collect();

    for _ in 0..n {
        let mut indices = all_cards.clone();
        indices.shuffle(&mut rng);
        let mut mask = StdDeckCardMask::new();
        // 7-card hand
        for &idx in indices.iter().take(7) {
            mask.set(idx);
        }
        masks.push(mask.as_raw());
    }
    masks
}

fn bench_gpu_vs_cpu(c: &mut Criterion) {
    let gpu = pollster::block_on(GPUEvaluator::new());
    if gpu.is_none() {
        println!("Skipping GPU benchmark: No compatible adapter found.");
        return;
    }
    let mut gpu = gpu.unwrap();

    let mut group = c.benchmark_group("eval_throughput");

    for size in [1024, 8192, 65536].iter() {
        let masks = generate_random_masks(*size);

        group.bench_with_input(BenchmarkId::new("CPU", size), size, |b, _| {
            b.iter(|| {
                for m_raw in &masks {
                    let mask = StdDeckCardMask::from_raw(*m_raw);
                    let _ = HoldemEvaluator::evaluate_hand(&mask, &StdDeckCardMask::new()).unwrap();
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("GPU", size), size, |b, _| {
            b.iter(|| {
                let results = gpu.evaluate_batch(black_box(&masks));
                black_box(results)
            })
        });

        group.bench_with_input(BenchmarkId::new("GPU_chunked_8192", size), size, |b, _| {
            b.iter(|| {
                let results = gpu.evaluate_batch_chunked(black_box(&masks), 8192);
                black_box(results)
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_gpu_vs_cpu);
criterion_main!(benches);
