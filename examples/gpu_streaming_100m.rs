use poker_eval_rs::deck::StdDeckCardMask;
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
        for &idx in indices.iter().take(7) {
            mask.set(idx);
        }
        masks.push(mask.as_raw());
    }
    masks
}

fn parse_arg_u64(flag: &str, default: u64) -> u64 {
    let args: Vec<String> = std::env::args().collect();
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            if let Ok(v) = args[i + 1].parse::<u64>() {
                return v;
            }
        }
    }
    default
}

fn main() {
    let total_hands = parse_arg_u64("--total", 100_000_000);
    let sample_size = parse_arg_u64("--sample", 262_144) as usize;
    let candidates = [4096usize, 8192, 16384, 32768, 65536, 131072];

    let mut gpu = match pollster::block_on(GPUEvaluator::new()) {
        Some(g) => g,
        None => {
            eprintln!("No compatible GPU adapter found.");
            return;
        }
    };

    let sample = generate_random_masks(sample_size);
    let chunk_size = gpu.autotune_chunk_size(&sample, &candidates);
    println!(
        "Auto-tuned chunk size: {} (sample={} hands)",
        chunk_size, sample_size
    );

    let stats = gpu.stream_evaluate(total_hands, chunk_size, generate_random_masks);
    println!("GPU streaming run complete");
    println!("total_hands={}", stats.total_hands);
    println!("chunk_size={}", stats.chunk_size);
    println!("chunks_processed={}", stats.chunks_processed);
    println!("elapsed_seconds={:.3}", stats.elapsed.as_secs_f64());
    println!("hands_per_second={:.2}", stats.hands_per_second);
    println!("checksum={}", stats.checksum);
}
