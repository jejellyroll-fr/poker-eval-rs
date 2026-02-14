#![allow(dead_code)]
// Required imports
use crate::handval::HandVal;

// Constants for player limits
pub const ENUM_ORDERING_MAXPLAYERS: usize = 7;
pub const ENUM_ORDERING_MAXPLAYERS_HILO: usize = 5;

use serde::{Deserialize, Serialize};

// Enum for the final hand ordering modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnumOrderingMode {
    None = 0,
    Hi,
    Lo,
    Hilo,
}

// Structure for tracking hand orderings
#[derive(Debug, Serialize, Deserialize)]
pub struct EnumOrdering {
    pub mode: EnumOrderingMode,
    pub nplayers: usize,
    pub nentries: usize,
    pub hist: Vec<u32>,
}

// Bit table for player ranks
static ENUM_NBITS: [i32; ENUM_ORDERING_MAXPLAYERS + 1] = [0, 1, 2, 2, 3, 3, 3, 3];

// Function to compute hand ranks
pub fn enum_ordering_rank(
    hands: &mut [HandVal],
    noqual: HandVal,
    nplayers: usize,
    ranks: &mut [i32],
    reverse: bool,
) {
    // Create an intermediate structure for sorting
    let mut elems: Vec<(usize, HandVal)> = hands
        .iter()
        .enumerate()
        .map(|(index, handval)| (index, *handval))
        .collect();

    // Sort hands by value, in ascending or descending order
    if reverse {
        elems.sort_by(|a, b| b.1.value.cmp(&a.1.value)); // Descending order if reverse is true
    } else {
        elems.sort_by(|a, b| a.1.value.cmp(&b.1.value)); // Ascending order otherwise
    }

    // Assign ranks based on sorting
    let mut currank = 0;
    let mut lastval = elems[0].1.value;
    for &(index, ref handval) in &elems {
        if handval.value != lastval {
            currank += 1;
            lastval = handval.value;
        }
        if handval.value == noqual.value {
            ranks[index] = nplayers as i32; // Rank for no qualifier
        } else {
            ranks[index] = currank;
        }
    }
}

// Function to encode ranks into a single integer
pub fn enum_ordering_encode(nplayers: usize, ranks: &[i32]) -> i32 {
    let mut encoding = 0;
    let nbits = ENUM_NBITS[nplayers];
    for &rank in ranks.iter() {
        encoding = (encoding << nbits) | rank;
    }
    encoding
}

// Function to encode high/low ranks into a single integer
pub fn enum_ordering_encode_hilo(nplayers: usize, hiranks: &[i32], loranks: &[i32]) -> i32 {
    let mut encoding = 0;
    let nbits = ENUM_NBITS[nplayers];
    for &rank in hiranks.iter() {
        encoding = (encoding << nbits) | rank;
    }
    for &rank in loranks.iter() {
        encoding = (encoding << nbits) | rank;
    }
    encoding
}

//
// Function to decode a player's rank from the encoding
pub fn enum_ordering_decode_k(encoding: i32, nplayers: usize, k: usize) -> i32 {
    let nbits = ENUM_NBITS[nplayers];
    let shift = (nplayers - k - 1) * (nbits as usize);
    (encoding >> shift) & ((1 << nbits) - 1)
}

pub fn enum_ordering_decode_hilo_k_hi(encoding: i32, nplayers: usize, k: usize) -> i32 {
    let nbits_per_rank = ENUM_NBITS[nplayers] as usize;
    let total_bits = nbits_per_rank * nplayers * 2; // Multiplied by 2 for high and low ranks
    let high_bits_offset = total_bits / 2; // Half the bits for high ranks

    // Compute the shift for player k's high rank
    let shift = high_bits_offset - (k + 1) * nbits_per_rank;

    // Extract player k's high rank
    (encoding >> shift) & ((1 << nbits_per_rank) - 1)
}

pub fn enum_ordering_decode_hilo_k_lo(encoding: i32, nplayers: usize, k: usize) -> i32 {
    let nbits_per_rank = ENUM_NBITS[nplayers] as usize;
    // No additional shift needed for low ranks, as they immediately follow high ranks
    let shift = (nplayers - k - 1) * nbits_per_rank;

    // Extract player k's low rank
    (encoding >> shift) & ((1 << nbits_per_rank) - 1)
}

// Function to compute the number of histogram entries
pub fn enum_ordering_nentries(nplayers: usize) -> i32 {
    if nplayers > ENUM_ORDERING_MAXPLAYERS || ENUM_NBITS[nplayers] < 0 {
        -1
    } else {
        1 << (nplayers * (ENUM_NBITS[nplayers] as usize))
    }
}

// Function to compute the number of histogram entries for high/low games
pub fn enum_ordering_nentries_hilo(nplayers: usize) -> i32 {
    if nplayers > ENUM_ORDERING_MAXPLAYERS_HILO || ENUM_NBITS[nplayers] < 0 {
        -1
    } else {
        1 << (2 * nplayers * (ENUM_NBITS[nplayers] as usize))
    }
}

// Function to increment a specific histogram entry
pub fn enum_ordering_increment(ordering: &mut EnumOrdering, ranks: &[i32]) {
    let encoding = enum_ordering_encode(ordering.nplayers, ranks);
    ordering.hist[encoding as usize] += 1;
}

// Function to increment a specific histogram entry for high/low games
pub fn enum_ordering_increment_hilo(ordering: &mut EnumOrdering, hiranks: &[i32], loranks: &[i32]) {
    let encoding = enum_ordering_encode_hilo(ordering.nplayers, hiranks, loranks);
    ordering.hist[encoding as usize] += 1;
}

// Implementation of EnumOrdering
impl EnumOrdering {
    pub fn new(mode: EnumOrderingMode, nplayers: usize) -> Self {
        let nentries = match mode {
            EnumOrderingMode::Hilo => enum_ordering_nentries_hilo(nplayers) as usize,
            _ => enum_ordering_nentries(nplayers) as usize,
        };
        EnumOrdering {
            mode,
            nplayers,
            nentries,
            hist: vec![0; nentries],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test to verify the creation of an EnumOrdering object
    #[test]
    fn test_enum_ordering_new() {
        let ordering = EnumOrdering::new(EnumOrderingMode::Hi, 5);
        assert_eq!(ordering.mode, EnumOrderingMode::Hi);
        assert_eq!(ordering.nplayers, 5);
    }

    // Test for the enum_ordering_rank function
    #[test]
    fn test_enum_ordering_rank() {
        let mut hands = vec![
            HandVal { value: 3 }, // Hand 1
            HandVal { value: 5 }, // Hand 2
            HandVal { value: 2 }, // Hand 3
        ];
        let noqual = HandVal { value: 0 };
        let nplayers = 3;
        let mut ranks = vec![0; nplayers];

        enum_ordering_rank(&mut hands, noqual, nplayers, &mut ranks, false);

        assert_eq!(ranks, vec![1, 2, 0]); // Expected ranks after sorting
    }

    // Test for enum_ordering_encode
    #[test]
    fn test_enum_ordering_encode() {
        let ranks = vec![1, 2, 0];
        let nplayers = 3;
        let encoded = enum_ordering_encode(nplayers, &ranks);

        assert_eq!(encoded, 24); // Expected encoded value
    }

    // Test for enum_ordering_decode_k
    #[test]
    fn test_enum_ordering_decode_k() {
        let encoded = 9; // Encoding of [1, 2, 0]
        let nplayers = 3;
        let rank = enum_ordering_decode_k(encoded, nplayers, 1); // Decoding the 2nd rank

        assert_eq!(rank, 2); // Expected rank
    }

    #[test]
    fn test_enum_ordering_decode_k_2_players() {
        // For 2 players, ENUM_NBITS[2] = 2 bits per player
        // Encoding ranks [1, 0] = (1 << 2) | 0 = 4 = 0b0100
        let encoding: i32 = 0b0100; // 4 in decimal
        let nplayers: usize = 2;

        // Test with k = 0 (first player's rank)
        let k0_result = enum_ordering_decode_k(encoding, nplayers, 0);
        assert_eq!(k0_result, 1); // Expected: 1 (bits [3:2] = 01)

        // Test with k = 1 (second player's rank)
        let k1_result = enum_ordering_decode_k(encoding, nplayers, 1);
        assert_eq!(k1_result, 0); // Expected: 0 (bits [1:0] = 00)
    }
}
