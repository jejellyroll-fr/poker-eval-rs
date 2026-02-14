use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// OMPEval-compatible rank multipliers.
/// These are carefully chosen so that every unique rank distribution
/// (0-4 cards per rank, 0-7 total cards) maps to a unique key.
const RANKS: [u32; 13] = [
    0x2000, 0x8001, 0x11000, 0x3a000, 0x91000, 0x176005, 0x366000, 0x41a013, 0x47802e, 0x479068,
    0x48c0e4, 0x48f211, 0x494493,
];

/// Flush ranks: powers of 2 (since flushes have at most 1 card per rank per suit).
const FLUSH_RANKS: [u32; 13] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];

/// Row shift for the perfect hash offset table.
const PERF_HASH_ROW_SHIFT: u32 = 12;

/// MAX_KEY = 4*RANKS[12] + 3*RANKS[11]
const MAX_KEY: u32 = 4 * RANKS[12] + 3 * RANKS[11];

#[allow(unused_assignments)]
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("rank_eval_table_generated.rs");
    let mut f = File::create(&dest_path).unwrap();

    // ========================================================================
    // 1. Build the ORIG_LOOKUP table (sparse, indexed by raw key)
    // ========================================================================
    let orig_size = (MAX_KEY + 1) as usize;
    let mut orig_lookup = vec![0u32; orig_size];
    let mut flush_lookup = vec![0u32; 8192];

    // Populate all hand types in order of increasing strength.
    // hand_value is a monotonically increasing counter — higher = stronger hand.
    let mut hand_value: u32 = 0;

    // Hand type category offsets. We store values in HandVal format:
    // (hand_type << 24) | (monotonic_rank_within_category)
    // hand_type: 0=NoPair, 1=OnePair, 2=TwoPair, 3=Trips, 4=Straight, 5=Flush, 6=FullHouse, 7=Quads, 8=StFlush

    // --- 0. High Card ---
    hand_value = populate_lookup(
        &mut orig_lookup,
        &mut None,
        0u64,
        0,
        hand_value,
        13,
        0,
        0,
        0,
        false,
    );

    // --- 1. OnePair ---
    hand_value = 1 << 24;
    for r in 0..13u32 {
        hand_value = populate_lookup(
            &mut orig_lookup,
            &mut None,
            2u64 << (4 * r),
            2,
            hand_value,
            13,
            0,
            0,
            0,
            false,
        );
    }

    // --- 2. TwoPair ---
    hand_value = 2 << 24;
    for r1 in 0..13u32 {
        for r2 in 0..r1 {
            hand_value = populate_lookup(
                &mut orig_lookup,
                &mut None,
                (2u64 << (4 * r1)) + (2u64 << (4 * r2)),
                4,
                hand_value,
                13,
                r2,
                0,
                0,
                false,
            );
        }
    }

    // --- 3. Trips ---
    hand_value = 3 << 24;
    for r in 0..13u32 {
        hand_value = populate_lookup(
            &mut orig_lookup,
            &mut None,
            3u64 << (4 * r),
            3,
            hand_value,
            13,
            0,
            r,
            0,
            false,
        );
    }

    // --- 4. Straight ---
    hand_value = 4 << 24;
    // Wheel (A-2-3-4-5)
    hand_value = populate_lookup(
        &mut orig_lookup,
        &mut None,
        0x1000000001111u64,
        5,
        hand_value,
        13,
        13,
        13,
        3,
        false,
    );
    for r in 4..13u32 {
        hand_value = populate_lookup(
            &mut orig_lookup,
            &mut None,
            0x11111u64 << (4 * (r - 4)),
            5,
            hand_value,
            13,
            13,
            13,
            r,
            false,
        );
    }

    // --- 5. Flush ---
    hand_value = 5 << 24;
    hand_value = populate_lookup(
        &mut flush_lookup,
        &mut None,
        0u64,
        0,
        hand_value,
        13,
        0,
        0,
        0,
        true,
    );

    // --- 6. FullHouse ---
    hand_value = 6 << 24;
    for r1 in 0..13u32 {
        for r2 in 0..13u32 {
            if r2 != r1 {
                hand_value = populate_lookup(
                    &mut orig_lookup,
                    &mut None,
                    (3u64 << (4 * r1)) + (2u64 << (4 * r2)),
                    5,
                    hand_value,
                    13,
                    r2,
                    r1,
                    13,
                    false,
                );
            }
        }
    }

    // --- 7. Quads ---
    hand_value = 7 << 24;
    for r in 0..13u32 {
        hand_value = populate_lookup(
            &mut orig_lookup,
            &mut None,
            4u64 << (4 * r),
            4,
            hand_value,
            13,
            13,
            13,
            13,
            false,
        );
    }

    // --- 8. StFlush ---
    hand_value = 8 << 24;
    // Wheel straight flush
    hand_value = populate_lookup(
        &mut flush_lookup,
        &mut None,
        0x1000000001111u64,
        5,
        hand_value,
        13,
        0,
        0,
        3,
        true,
    );
    for r in 4..13u32 {
        hand_value = populate_lookup(
            &mut flush_lookup,
            &mut None,
            0x11111u64 << (4 * (r - 4)),
            5,
            hand_value,
            13,
            0,
            0,
            r,
            true,
        );
    }

    // Post-populate FLUSH_LOOKUP for 6-card and 7-card suit masks.
    // For any mask with >5 bits set, find the best 5-card subset.
    for mask in 0u16..8192 {
        let bits = mask.count_ones();
        if bits >= 6 {
            let mut best = 0u32;
            // Iterate over all 5-card subsets of this mask
            // Use recursive approach: remove one bit at a time
            for i in 0..13u16 {
                if (mask & (1 << i)) == 0 {
                    continue;
                }
                let m6 = mask ^ (1 << i); // remove bit i
                if bits == 6 {
                    // m6 is a 5-card mask
                    let v = flush_lookup[m6 as usize];
                    if v > best {
                        best = v;
                    }
                } else {
                    // bits == 7, need to remove one more
                    for j in (i + 1)..13u16 {
                        if (m6 & (1 << j)) == 0 {
                            continue;
                        }
                        let m5 = m6 ^ (1 << j);
                        let v = flush_lookup[m5 as usize];
                        if v > best {
                            best = v;
                        }
                    }
                }
            }
            flush_lookup[mask as usize] = best;
            if mask == 4592 {
                println!("cargo:warning=FLUSH_LOOKUP[4592] (As6s7s8s9sTs) = {} (best from 5-card subsets)", best);
                // Print all 5-card subsets
                for i in 0..13u16 {
                    if (mask & (1 << i)) == 0 {
                        continue;
                    }
                    let m5 = mask ^ (1 << i);
                    println!(
                        "cargo:warning=  Removing bit {}: mask {} -> FLUSH_LOOKUP[{}] = {}",
                        i, m5, m5, flush_lookup[m5 as usize]
                    );
                }
            }
        }
    }

    println!("cargo:warning=Total hand values used: {}", hand_value);

    // ========================================================================
    // 2. Calculate Perfect Hash Row Offsets
    //    Divides the sparse orig_lookup into rows of 4096 entries.
    //    For each row, finds an offset so entries don't collide with others.
    // ========================================================================
    let n_rows = (MAX_KEY >> PERF_HASH_ROW_SHIFT) as usize + 1;
    let column_mask = (1u32 << PERF_HASH_ROW_SHIFT) - 1;

    // Collect non-zero entries per row
    let mut rows: Vec<(usize, Vec<usize>)> = Vec::new();
    for (i, &val) in orig_lookup.iter().enumerate().take(orig_size) {
        if val != 0 {
            let row_idx = i >> PERF_HASH_ROW_SHIFT;
            while rows.len() <= row_idx {
                rows.push((rows.len(), Vec::new()));
            }
            rows[row_idx].1.push(i);
        }
    }
    // Pad rows to n_rows
    while rows.len() < n_rows {
        rows.push((rows.len(), Vec::new()));
    }

    // Sort rows by density (densest first)
    rows.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    let mut row_offsets = vec![0u32; n_rows];
    // Start with a reasonable initial size for the compact lookup
    let mut compact_lookup = vec![0u32; 100_000];

    let mut max_idx: usize = 0;
    let is_large = env::var("CARGO_FEATURE_LARGE_TABLE").is_ok();

    if is_large {
        // LARGE TABLE: Simple sparse array indexed by raw key.
        // Size: ~9.6 MB (4.8M entries * 2 bytes).
        // Advantage: Single memory access (NOFLUSH_LOOKUP[key]).
        println!("cargo:warning=Generating LARGE (sparse) lookup table (~9.6 MB)");
        for i in 0..orig_size {
            if orig_lookup[i] != 0 {
                if i >= compact_lookup.len() {
                    compact_lookup.resize(i + 1, 0);
                }
                compact_lookup[i] = orig_lookup[i];
                if i > max_idx {
                    max_idx = i;
                }
            }
        }
    } else {
        // COMPACT TABLE: Row-offset based perfect hash.
        // Size: ~170 KB + 32 KB offsets.
        // Advantage: Fits in L1/L2 cache.
        println!("cargo:warning=Generating COMPACT perfect-hash lookup table (~200 KB)");
        for (orig_row_idx, entries) in rows.iter() {
            if entries.is_empty() {
                continue;
            }

            let mut offset: usize = 0;
            'find_offset: loop {
                let mut ok = true;
                for &key in entries {
                    let col = (key as u32 & column_mask) as usize;
                    let new_idx = col + offset;
                    // Grow compact_lookup if needed
                    if new_idx >= compact_lookup.len() {
                        compact_lookup.resize(new_idx + 4096, 0);
                    }
                    let existing = compact_lookup[new_idx];
                    if existing != 0 && existing != orig_lookup[key] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    break 'find_offset;
                }
                offset += 1;
            }

            // Record the offset (relative to row start). Use wrapping_sub like C++ unsigned semantics.
            let row_start = (*orig_row_idx as u32) << PERF_HASH_ROW_SHIFT;
            row_offsets[*orig_row_idx] = (offset as u32).wrapping_sub(row_start);

            for &key in entries {
                let col = (key as u32 & column_mask) as usize;
                let new_idx = col + offset;
                if new_idx >= compact_lookup.len() {
                    compact_lookup.resize(new_idx + 4096, 0);
                }
                compact_lookup[new_idx] = orig_lookup[key];
                if new_idx > max_idx {
                    max_idx = new_idx;
                }
            }
        }
    }

    let lookup_size = max_idx + 1;
    compact_lookup.truncate(lookup_size);

    if is_large {
        println!(
            "cargo:warning=Large LOOKUP table size: {} entries ({} KB)",
            lookup_size,
            lookup_size * 2 / 1024
        );
    } else {
        println!(
            "cargo:warning=Compact LOOKUP table size: {} entries ({} KB)",
            lookup_size,
            lookup_size * 2 / 1024
        );
        println!(
            "cargo:warning=ROW_OFFSETS table: {} entries ({} KB)",
            n_rows,
            n_rows * 4 / 1024
        );
    }
    println!("cargo:warning=FLUSH_LOOKUP table: 8192 entries (16 KB)");

    // ========================================================================
    // 3. Precompute SUIT_HASH_TABLE: for each 13-bit suit mask, the sum of RANKS[r]
    //    This allows O(1) computation: sum = SUIT_HASH_TABLE[ss] + ... + SUIT_HASH_TABLE[sh]
    // ========================================================================
    let mut suit_hash_table = vec![0u32; 8192];
    for mask in 0u32..8192 {
        let mut h = 0u32;
        for i in 0..13u32 {
            if (mask & (1 << i)) != 0 {
                h = h.wrapping_add(RANKS[i as usize]);
            }
        }
        suit_hash_table[mask as usize] = h;
    }

    // ========================================================================
    // 4. Emit all tables
    // ========================================================================
    writeln!(f, "/// OMPEval-compatible rank multipliers.").unwrap();
    writeln!(f, "pub const RANKS: [u32; 13] = {:?};", RANKS).unwrap();
    writeln!(f).unwrap();

    writeln!(f, "/// Flush rank multipliers (powers of 2).").unwrap();
    writeln!(f, "pub const FLUSH_RANKS: [u32; 13] = {:?};", FLUSH_RANKS).unwrap();
    writeln!(f).unwrap();

    writeln!(
        f,
        "pub const PERF_HASH_ROW_SHIFT: u32 = {};",
        PERF_HASH_ROW_SHIFT
    )
    .unwrap();
    if is_large {
        writeln!(f, "pub const PERF_HASH_ROW_OFFSETS: [u32; 0] = [];").unwrap();
    } else {
        writeln!(f, "/// Perfect hash row offsets.").unwrap();
        writeln!(f, "pub static PERF_HASH_ROW_OFFSETS: [u32; {}] = [", n_rows).unwrap();
        for (i, &v) in row_offsets.iter().enumerate() {
            if i % 8 == 0 {
                write!(f, "    ").unwrap();
            }
            write!(f, "0x{:08x}, ", v).unwrap();
            if i % 8 == 7 {
                writeln!(f).unwrap();
            }
        }
        writeln!(f, "];").unwrap();
    }
    writeln!(f).unwrap();

    writeln!(f, "/// Pre-computed rank key for each 13-bit suit mask.").unwrap();
    writeln!(
        f,
        "/// Usage: key = SUIT_HASH[ss] + SUIT_HASH[sc] + SUIT_HASH[sd] + SUIT_HASH[sh]"
    )
    .unwrap();
    writeln!(
        f,
        "pub static SUIT_HASH: [u32; 8192] = {:?};",
        suit_hash_table
    )
    .unwrap();
    writeln!(f).unwrap();

    writeln!(f, "/// Non-flush hand lookup table.").unwrap();
    if is_large {
        writeln!(f, "/// Indexed directly by key: NOFLUSH_LOOKUP[key]").unwrap();
    } else {
        writeln!(
            f,
            "/// Indexed via perf_hash(key) = key + ROW_OFFSETS[key >> SHIFT]"
        )
        .unwrap();
    }
    writeln!(
        f,
        "/// Values are in HandVal u32 format: (hand_type << 24) | monotonic_rank"
    )
    .unwrap();
    writeln!(f, "pub static NOFLUSH_LOOKUP: [u32; {}] = [", lookup_size).unwrap();
    for (i, &v) in compact_lookup.iter().enumerate() {
        if i % 16 == 0 {
            write!(f, "    ").unwrap();
        }
        write!(f, "{}, ", v).unwrap();
        if i % 16 == 15 {
            writeln!(f).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
    writeln!(f).unwrap();

    writeln!(
        f,
        "/// Flush hand lookup table (indexed by 13-bit bitmask)."
    )
    .unwrap();
    writeln!(
        f,
        "/// Values are in HandVal u32 format: (hand_type << 24) | monotonic_rank"
    )
    .unwrap();
    writeln!(
        f,
        "pub static FLUSH_LOOKUP: [u32; 8192] = {:?};",
        flush_lookup
    )
    .unwrap();
    writeln!(f).unwrap();

    // Keep legacy table for backward compat during transition
    writeln!(f, "/// Legacy 13-bit rank lookup (kept for compatibility).").unwrap();
    writeln!(
        f,
        "pub static RANK_5_EVAL_TABLE: [u32; 8192] = [0u32; 8192];"
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

/// Recursively enumerates kicker combinations and writes hand values to the lookup table.
///
/// This mirrors OMPEval's `populateLookup` function exactly:
/// - `ranks`: 4-bit per rank counters packed into a u64
/// - `ncards`: total cards in the hand so far
/// - `hand_value`: current monotonic hand strength counter
/// - `end_rank`: highest rank allowed for new kickers (prevents duplicates)
/// - `max_pair`, `max_trips`, `max_straight`: constraints to prevent hand improvement
/// - `flush`: if true, write to flush table (max 1 card per rank)
#[allow(clippy::too_many_arguments)]
fn populate_lookup(
    lookup: &mut Vec<u32>,
    _orig: &mut Option<&mut Vec<u32>>,
    ranks: u64,
    ncards: u32,
    mut hand_value: u32,
    end_rank: u32,
    max_pair: u32,
    max_trips: u32,
    max_straight: u32,
    flush: bool,
) -> u32 {
    // Increment hand value for every valid 5-card combination (or smaller if min_cards < 5)
    // We use MIN_CARDS = 0 like OMPEval for flexibility
    if ncards == 5 {
        hand_value += 1;
    }

    // Write hand value to lookup when we have required number of cards
    if !flush || ncards >= 5 {
        if flush {
            let key = get_key(ranks, true);
            if key < lookup.len() as u32 {
                lookup[key as usize] = hand_value;
            }
        } else {
            let key = get_key(ranks, false);
            if (key as usize) < lookup.len() {
                lookup[key as usize] = hand_value;
            }
        }

        if ncards == 7 {
            return hand_value;
        }
    }

    // Iterate next card rank
    for r in 0..end_rank {
        let new_ranks = ranks + (1u64 << (4 * r));
        let rank_count = ((new_ranks >> (r * 4)) & 0xf) as u32;

        // Check that hand doesn't improve beyond its category
        if rank_count == 2 && r >= max_pair {
            continue;
        }
        if rank_count == 3 && r >= max_trips {
            continue;
        }
        if rank_count >= 4 && !flush {
            continue;
        } // Don't allow quads (or more) as kickers
        if flush && rank_count >= 2 {
            continue;
        } // Flush: max 1 card per rank
        if get_biggest_straight(new_ranks) > max_straight {
            continue;
        }

        hand_value = populate_lookup(
            lookup,
            _orig,
            new_ranks,
            ncards + 1,
            hand_value,
            r + 1,
            max_pair,
            max_trips,
            max_straight,
            flush,
        );
    }

    hand_value
}

/// Calculate lookup table key from rank counts.
fn get_key(ranks: u64, flush: bool) -> u32 {
    let mut key = 0u32;
    for r in 0..13u32 {
        let count = ((ranks >> (r * 4)) & 0xf) as u32;
        if flush {
            key += count * FLUSH_RANKS[r as usize];
        } else {
            key += count * RANKS[r as usize];
        }
    }
    key
}

/// Returns index of the highest straight card or 0 when no straight.
fn get_biggest_straight(ranks: u64) -> u32 {
    // Build a presence mask (1 bit per rank if count > 0)
    let rank_mask = (0x1111111111111u64 & ranks)
        | ((0x2222222222222u64 & ranks) >> 1)
        | ((0x4444444444444u64 & ranks) >> 2);

    for i in (0..9u32).rev() {
        if ((rank_mask >> (4 * i)) & 0x11111u64) == 0x11111u64 {
            return i + 4;
        }
    }
    // Check wheel (A-2-3-4-5)
    if (rank_mask & 0x1000000001111u64) == 0x1000000001111u64 {
        return 3;
    }
    0
}
