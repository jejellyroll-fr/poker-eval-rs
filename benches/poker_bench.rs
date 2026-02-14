//! Benchmarks for poker-eval-rs using Criterion
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poker_eval_rs::deck::{StdDeck, StdDeckCardMask};
use poker_eval_rs::enumdefs::{EnumResult, Game, SampleType, ENUM_MAXPLAYERS};
use poker_eval_rs::enumerate::evaluation::enum_exhaustive;
use poker_eval_rs::evaluators::Eval;

/// Benchmark hand evaluation for different hand sizes
fn bench_eval_n(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval_n");

    // 5-card hand
    let (mask5, _) = StdDeck::string_to_mask("AsKsQsJsTs").unwrap();
    group.bench_function("5_cards_royal_flush", |b| {
        b.iter(|| Eval::eval_n(black_box(&mask5), black_box(5)))
    });

    // 7-card hand (common in Hold'em)
    let (mask7, _) = StdDeck::string_to_mask("AsKsQsJsTs9s8s").unwrap();
    group.bench_function("7_cards_straight_flush", |b| {
        b.iter(|| Eval::eval_n(black_box(&mask7), black_box(7)))
    });

    // Different hand types
    let (pair, _) = StdDeck::string_to_mask("AsAd2h3c4s").unwrap();
    group.bench_function("5_cards_pair", |b| {
        b.iter(|| Eval::eval_n(black_box(&pair), black_box(5)))
    });

    let (two_pair, _) = StdDeck::string_to_mask("AsAdKhKc2s").unwrap();
    group.bench_function("5_cards_two_pair", |b| {
        b.iter(|| Eval::eval_n(black_box(&two_pair), black_box(5)))
    });

    let (flush, _) = StdDeck::string_to_mask("As9s7s5s3s").unwrap();
    group.bench_function("5_cards_flush", |b| {
        b.iter(|| Eval::eval_n(black_box(&flush), black_box(5)))
    });

    group.finish();
}

/// Benchmark card string to mask conversion
fn bench_string_to_mask(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_to_mask");

    group.bench_function("2_cards", |b| {
        b.iter(|| StdDeck::string_to_mask(black_box("AsKd")))
    });

    group.bench_function("5_cards", |b| {
        b.iter(|| StdDeck::string_to_mask(black_box("AsKsQsJsTs")))
    });

    group.bench_function("7_cards", |b| {
        b.iter(|| StdDeck::string_to_mask(black_box("AsKsQsJsTs9s8s")))
    });

    group.finish();
}

/// Benchmark equity calculation
fn bench_equity(c: &mut Criterion) {
    let mut group = c.benchmark_group("equity");
    group.sample_size(10); // Reduce sample size for slower benchmarks

    // Parse hands once
    let (aa, _) = StdDeck::string_to_mask("AsAd").unwrap();
    let (kk, _) = StdDeck::string_to_mask("KsKd").unwrap();

    // Pre-flop AA vs KK (most combinations)
    group.bench_function("aa_vs_kk_preflop", |b| {
        b.iter(|| {
            let mut result = create_empty_result(2);
            let pockets = vec![aa.clone(), kk.clone()];
            let _ = enum_exhaustive(
                Game::Holdem,
                &pockets,
                StdDeckCardMask::new(),
                StdDeckCardMask::new(),
                2,
                0,
                false,
                &mut result,
            );
            result.nsamples
        })
    });

    // With board (fewer combinations)
    let (board3, _) = StdDeck::string_to_mask("Th9h8h").unwrap();
    group.bench_function("aa_vs_kk_flop", |b| {
        b.iter(|| {
            let mut result = create_empty_result(2);
            let pockets = vec![aa.clone(), kk.clone()];
            let _ = enum_exhaustive(
                Game::Holdem,
                &pockets,
                board3.clone(),
                StdDeckCardMask::new(),
                2,
                3,
                false,
                &mut result,
            );
            result.nsamples
        })
    });

    group.finish();
}

/// Benchmark SIMD 8-hand evaluation
#[cfg(all(feature = "simd", target_arch = "x86_64", feature = "large-table", not(feature = "compact-table")))]
fn bench_eval_simd(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval_simd");

    let (h1, _) = StdDeck::string_to_mask("AsKsQsJsTs").unwrap();
    let (h2, _) = StdDeck::string_to_mask("AsAhAdAcKs").unwrap();
    let (h3, _) = StdDeck::string_to_mask("AsAhAdKsKc").unwrap();
    let (h4, _) = StdDeck::string_to_mask("2s3s4s5s9s").unwrap();
    let (h5, _) = StdDeck::string_to_mask("9s8h7d6c5s").unwrap();
    let (h6, _) = StdDeck::string_to_mask("AsAhAdKcJs").unwrap();
    let (h7, _) = StdDeck::string_to_mask("AsAhKdKcJs").unwrap();
    let (h8, _) = StdDeck::string_to_mask("AsKdQcJs9h").unwrap();

    let masks = [h1, h2, h3, h4, h5, h6, h7, h8];

    group.bench_function("eval_8_hands_avx2", |b| {
        b.iter(|| unsafe { Eval::eval_8_hands(black_box(&masks)) })
    });

    group.finish();
}

/// Create an empty EnumResult for benchmarking
fn create_empty_result(nplayers: usize) -> EnumResult {
    EnumResult {
        game: Game::Holdem,
        sample_type: SampleType::Exhaustive,
        nsamples: 0,
        nplayers: nplayers as u32,
        nwinhi: [0; ENUM_MAXPLAYERS],
        ntiehi: [0; ENUM_MAXPLAYERS],
        nlosehi: [0; ENUM_MAXPLAYERS],
        nwinlo: [0; ENUM_MAXPLAYERS],
        ntielo: [0; ENUM_MAXPLAYERS],
        nloselo: [0; ENUM_MAXPLAYERS],
        nscoop: [0; ENUM_MAXPLAYERS],
        nsharehi: [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
        nsharelo: [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
        nshare: {
            let mut ns = Box::new([[[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS]);
            *ns = [[[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS];
            ns
        },
        ev: [0.0; ENUM_MAXPLAYERS],
        ordering: None,
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64", feature = "large-table", not(feature = "compact-table")))]
criterion_group!(
    benches,
    bench_eval_n,
    bench_string_to_mask,
    bench_equity,
    bench_eval_simd
);
#[cfg(not(all(feature = "simd", target_arch = "x86_64", feature = "large-table", not(feature = "compact-table"))))]
criterion_group!(benches, bench_eval_n, bench_string_to_mask, bench_equity);
criterion_main!(benches);
