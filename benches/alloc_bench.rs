use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poker_eval_rs::deck::StdDeck;
use poker_eval_rs::evaluators::lowball::{extract_top_five_cards_lowball, get_trips};
use poker_eval_rs::evaluators::omaha::std_deck_omaha_hi_low8_eval;
use poker_eval_rs::evaluators::Eval;

fn bench_eval_hotpaths(c: &mut Criterion) {
    let mut group = c.benchmark_group("hotpaths");

    // Benchmark Eval::extract_top_five_cards via eval_n (since it's private/internal)
    // We use a Flush hand to force it to go effectively through the extraction logic
    let (flush_mask, _) = StdDeck::string_to_mask("AsKsQsJs9s").unwrap();
    group.bench_function("eval_n_flush", |b| {
        b.iter(|| Eval::eval_n(black_box(&flush_mask), black_box(5)))
    });

    // Benchmark eval_low.rs functions (they are public)
    let ranks_full = 0b1111111111111; // All ranks present
    let dups_pair = 0b100; // Pair of 3s (rank 2)

    group.bench_function("low_extract_top_five", |b| {
        b.iter(|| extract_top_five_cards_lowball(black_box(ranks_full)))
    });

    group.bench_function("low_get_trips", |b| {
        b.iter(|| get_trips(black_box(dups_pair), black_box(ranks_full)))
    });

    // Benchmark Omaha Hi/Lo Eval
    let (hole, _) = StdDeck::string_to_mask("As2s3d4c").unwrap();
    let (board, _) = StdDeck::string_to_mask("5h6h7h8d9c").unwrap();
    group.bench_function("omaha_hi_low_eval", |b| {
        b.iter(|| {
            let mut hival = None;
            let mut loval = None;
            std_deck_omaha_hi_low8_eval(black_box(hole), black_box(board), &mut hival, &mut loval)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_eval_hotpaths);
criterion_main!(benches);
