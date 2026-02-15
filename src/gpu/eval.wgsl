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
