use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::gpu::GPUEvaluator;
use pollster;

fn bench_gpu_rvr(c: &mut Criterion) {
    let mut gpu = pollster::block_on(GPUEvaluator::new()).expect("GPU required for bench");

    let mut group = c.benchmark_group("GPU Range-vs-Range");

    for size in [100, 500, 1000].iter() {
        let (mask, _) = StdDeck::string_to_mask("AsAd5h2d3c").unwrap();
        let range_a = vec![mask.as_raw(); *size];
        let range_b = vec![mask.as_raw(); *size];

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                gpu.evaluate_range_vs_range(&range_a, &range_b);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_gpu_rvr);
criterion_main!(benches);
