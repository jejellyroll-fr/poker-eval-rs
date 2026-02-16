@group(0) @binding(0) var<storage, read> noflush_lookup: array<u32>;
@group(0) @binding(1) var<storage, read> flush_lookup: array<u32>;
@group(0) @binding(2) var<storage, read> row_offsets: array<u32>;
@group(0) @binding(3) var<storage, read> suit_hash: array<u32>;

@group(1) @binding(0) var<storage, read> inputs: array<u32>; // [low, high] pairs (u64 masks)
@group(1) @binding(1) var<storage, read_write> results: array<u32>;

const PERF_HASH_ROW_SHIFT: u32 = 12u;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    if (idx >= arrayLength(&results)) { return; }

    // Each input is a u64 card mask, split into two u32s
    let low = inputs[idx * 2u];
    let high = inputs[idx * 2u + 1u];
    
    // Extract suit masks (13 bits per suit, 16-bit aligned)
    // Suit 3 (Spades): bits 0..12
    // Suit 2 (Clubs): bits 16..28
    // Suit 1 (Diamonds): bits 32..44 (high 0..12)
    // Suit 0 (Hearts): bits 48..60 (high 16..28)
    let s3 = low & 0x1FFFu;
    let s2 = (low >> 16u) & 0x1FFFu;
    let s1 = high & 0x1FFFu;
    let s0 = (high >> 16u) & 0x1FFFu;

    // 1. Flush Detection
    let f0 = flush_lookup[s0];
    if (f0 != 0u) { results[idx] = f0; return; }
    let f1 = flush_lookup[s1];
    if (f1 != 0u) { results[idx] = f1; return; }
    let f2 = flush_lookup[s2];
    if (f2 != 0u) { results[idx] = f2; return; }
    let f3 = flush_lookup[s3];
    if (f3 != 0u) { results[idx] = f3; return; }

    // 2. Non-Flush Evaluation (Perfect Hash)
    let key = suit_hash[s0] + suit_hash[s1] + suit_hash[s2] + suit_hash[s3];
    let row = key >> PERF_HASH_ROW_SHIFT;
    let offset = row_offsets[row];
    
    // In our build.rs: row_offsets[row] = offset.wrapping_sub(row_start)
    // and result index in Rust is key + offset.
    let final_idx = key + offset;
    results[idx] = noflush_lookup[final_idx];
}

@group(2) @binding(0) var<storage, read> range_a: array<u32>;
@group(2) @binding(1) var<storage, read> range_b: array<u32>;
@group(2) @binding(2) var<storage, read_write> matrix_results: array<u32>;
@group(2) @binding(3) var<storage, read> params: array<u32>; // [len_a, len_b]

fn evaluate_mask(low: u32, high: u32) -> u32 {
    let s3 = low & 0x1FFFu;
    let s2 = (low >> 16u) & 0x1FFFu;
    let s1 = high & 0x1FFFu;
    let s0 = (high >> 16u) & 0x1FFFu;

    let f0 = flush_lookup[s0];
    if (f0 != 0u) { return f0; }
    let f1 = flush_lookup[s1];
    if (f1 != 0u) { return f1; }
    let f2 = flush_lookup[s2];
    if (f2 != 0u) { return f2; }
    let f3 = flush_lookup[s3];
    if (f3 != 0u) { return f3; }

    let key = suit_hash[s0] + suit_hash[s1] + suit_hash[s2] + suit_hash[s3];
    let row = key >> PERF_HASH_ROW_SHIFT;
    return noflush_lookup[key + row_offsets[row]];
}

@compute @workgroup_size(16, 16)
fn range_vs_range(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = global_id.x;
    let j = global_id.y;
    let len_a = params[0];
    let len_b = params[1];

    if (i >= len_a || j >= len_b) { return; }

    let low_a = range_a[i * 2u];
    let high_a = range_a[i * 2u + 1u];
    let low_b = range_b[j * 2u];
    let high_b = range_b[j * 2u + 1u];

    let val_a = evaluate_mask(low_a, high_a);
    let val_b = evaluate_mask(low_b, high_b);

    let res_idx = i * len_b + j;
    if (val_a > val_b) {
        matrix_results[res_idx] = 1u; // A wins
    } else if (val_b > val_a) {
        matrix_results[res_idx] = 2u; // B wins
    } else {
        matrix_results[res_idx] = 3u; // Tie
    }
}
