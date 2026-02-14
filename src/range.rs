//! Hand range parsing and manipulation.
//!
//! A `HandRange` represents a set of specific hand combinations (e.g., "AKs" expands
//! into 4 specific suited combinations).

use crate::deck::{Rank, StdDeck, StdDeckCardMask, Suit};
use std::fmt;
use std::str::FromStr;

/// Represents a range of poker hands.
///
/// A range is essentially a collection of specific hand combinations (2-card hole cards).
/// It provides methods to iterate over these combinations for equity calculations.
#[derive(Clone, Default, PartialEq)]
pub struct HandRange {
    /// The specific hand combinations that make up this range, with their weights.
    /// Each mask represents a specific hand (2 cards for Hold'em, 4 for Omaha, etc.).
    pub hands: Vec<(StdDeckCardMask, f64)>,
}

impl HandRange {
    /// Creates a new empty `HandRange`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `HandRange` from a list of specific hand masks with default weight 1.0.
    pub fn from_hand_masks(hands: Vec<StdDeckCardMask>) -> Self {
        Self {
            hands: hands.into_iter().map(|h| (h, 1.0)).collect(),
        }
    }

    /// Returns a slice of the specific hand masks and weights in this range.
    pub fn hands(&self) -> &[(StdDeckCardMask, f64)] {
        &self.hands
    }

    /// Returns the number of specific combinations in this range.
    pub fn len(&self) -> usize {
        self.hands.len()
    }

    /// Returns true if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.hands.is_empty()
    }

    /// Adds a hand mask to the range with a default weight of 1.0.
    /// If the hand already exists, it updates the weight? Or ignores?
    /// For simple range construction, duplicates are usually ignored.
    pub fn push(&mut self, mask: StdDeckCardMask) {
        if !self.hands.iter().any(|(h, _)| *h == mask) {
            self.hands.push((mask, 1.0));
        }
    }

    /// Adds a hand mask with a specific weight.
    pub fn push_weighted(&mut self, mask: StdDeckCardMask, weight: f64) {
        if let Some(pos) = self.hands.iter().position(|(h, _)| *h == mask) {
            self.hands[pos].1 = weight;
        } else {
            self.hands.push((mask, weight));
        }
    }

    /// Merges another range into this one.
    pub fn merge(&mut self, other: HandRange) {
        for (hand, weight) in other.hands {
            self.push_weighted(hand, weight);
        }
    }
}

impl fmt::Debug for HandRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HandRange")
            .field("count", &self.hands.len())
            // We might want to avoid printing all hands if there are many
            .finish()
    }
}

impl FromStr for HandRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut range = HandRange::new();
        // Split by comma
        for part in s.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            range.merge(parse_range_part(part)?);
        }
        Ok(range)
    }
}

fn parse_range_part(s: &str) -> Result<HandRange, String> {
    let mut range = HandRange::new();

    // Check for specific cards first (e.g., "AhKh" or "AsKsQsJs")
    // Should do this before range syntax to catch explicit lists of cards.
    // However, string_to_mask is strict (expects RankSuit pairs), so it won't accidentally match "AKs" or "JJ+".
    if let Ok((mask, count)) = StdDeck::string_to_mask(s) {
        if count > 0 {
            range.push(mask);
            return Ok(range);
        }
    }

    // Check for "plus" notation
    if s.ends_with('+') {
        let base = &s[0..s.len() - 1];
        return parse_plus_notation(base);
    }

    // Check for "dash" notation (range)
    if let Some(idx) = s.find('-') {
        let start = &s[0..idx];
        let end = &s[idx + 1..];
        return parse_dash_notation(start, end);
    }

    // Check for single hand notation: "AK", "AKs", "AKo", "TT"
    parse_single_notation(s)
}

fn parse_single_notation(s: &str) -> Result<HandRange, String> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 2 || chars.len() > 3 {
        return Err(format!("Invalid token length: {}", s));
    }

    let r1 = Rank::from_char(chars[0]).ok_or_else(|| format!("Invalid rank: {}", chars[0]))?;
    let r2 = Rank::from_char(chars[1]).ok_or_else(|| format!("Invalid rank: {}", chars[1]))?;

    let suffix = if chars.len() == 3 {
        Some(chars[2])
    } else {
        None
    };

    let mut range = HandRange::new();

    if r1 == r2 {
        // Pairs: "TT", "AA"
        // Suffix should be none for pairs in strictly single notation, but sometimes people write "TTs" (invalid) or "TTo" (invalid).
        // Actually, "TT" implies all 6 combos.
        if suffix.is_some() {
            return Err(format!("Invalid suffix for pair: {}", s));
        }
        add_pair_combos(&mut range, r1);
    } else {
        // Non-pairs: "AK", "AKs", "AKo"
        // Ensure r1 > r2 for canonical processing, or handle provided order.
        // Usually notation is "AK", not "KA".
        // Let's assume input is high card first. If not, swap?
        let (high, low) = if r1 > r2 { (r1, r2) } else { (r2, r1) };

        match suffix {
            Some('s') => add_suited_combos(&mut range, high, low),
            Some('o') => add_offsuit_combos(&mut range, high, low),
            None => {
                // "AK" -> implies suited AND offsuit
                add_suited_combos(&mut range, high, low);
                add_offsuit_combos(&mut range, high, low);
            }
            Some(c) => return Err(format!("Invalid suffix: {}", c)),
        }
    }

    Ok(range)
}

fn parse_plus_notation(base: &str) -> Result<HandRange, String> {
    // "JJ+" -> JJ, QQ, KK, AA
    // "AJs+" -> AJs, AQs, AKs
    // "AJo+" -> AJo, AQo, AKo
    // "AJ+" -> AJs+, AJo+

    let chars: Vec<char> = base.chars().collect();
    if chars.len() < 2 {
        return Err(format!("Invalid plus base: {}", base));
    }

    let r1 = Rank::from_char(chars[0]).ok_or_else(|| format!("Invalid rank: {}", chars[0]))?;
    let r2 = Rank::from_char(chars[1]).ok_or_else(|| format!("Invalid rank: {}", chars[1]))?;
    let suffix = if chars.len() == 3 {
        Some(chars[2])
    } else {
        None
    };

    let mut range = HandRange::new();

    if r1 == r2 {
        // Pairs: "JJ+" -> JJ..AA
        if suffix.is_some() {
            return Err(format!("Invalid suffix for pair range: {}", base));
        }
        for r_val in r1.as_u8()..13 {
            // 13 is Rank::COUNT
            add_pair_combos(&mut range, Rank::new(r_val));
        }
    } else {
        // "AJs+" -> AJs, AQs, AKs
        // Base is AJs. We keep the high card (A) constant, and iterate the low card from J up to A-1.
        // "K9s+" -> K9s, KTs, KJs, KQs. (Up to K(K-1)s)

        let (high, low) = if r1 > r2 { (r1, r2) } else { (r2, r1) };

        // Iterate kicker from low.as_u8() up to high.as_u8() - 1
        for kicker_val in low.as_u8()..high.as_u8() {
            let kicker = Rank::new(kicker_val);
            match suffix {
                Some('s') => add_suited_combos(&mut range, high, kicker),
                Some('o') => add_offsuit_combos(&mut range, high, kicker),
                None => {
                    add_suited_combos(&mut range, high, kicker);
                    add_offsuit_combos(&mut range, high, kicker);
                }
                Some(c) => return Err(format!("Invalid suffix: {}", c)),
            }
        }
    }

    Ok(range)
}

fn parse_dash_notation(start_str: &str, end_str: &str) -> Result<HandRange, String> {
    // "88-66" -> 88, 77, 66
    // "KJs-K9s" -> KJs, KTs, K9s
    // "KJo-K9o" -> KJo, KTo, K9o

    // Parse start and end to get ranks and type.
    // They must match in type (pair vs non-pair, suited vs offsuit).

    // Helper to extract structure
    let analyze = |s: &str| -> Result<(Rank, Rank, Option<char>), String> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() < 2 {
            return Err("Too short".into());
        }
        let r1 = Rank::from_char(chars[0]).ok_or(format!("Invalid rank {}", chars[0]))?;
        let r2 = Rank::from_char(chars[1]).ok_or(format!("Invalid rank {}", chars[1]))?;
        let suffix = if chars.len() == 3 {
            Some(chars[2])
        } else {
            None
        };
        Ok((r1, r2, suffix))
    };

    let (s_r1, s_r2, s_suff) = analyze(start_str)?;
    let (e_r1, e_r2, e_suff) = analyze(end_str)?;

    if s_suff != e_suff {
        return Err("Range limits must have same suffix format".into());
    }

    let mut range = HandRange::new();

    if s_r1 == s_r2 {
        // Pair range: "88-66"
        if e_r1 != e_r2 {
            return Err("Range mismatch: pair vs non-pair".into());
        }

        // Determine start and end rank. Usually typically "High-Low" e.g "88-66".
        // But support "66-88" too.
        let start_val = s_r1.as_u8();
        let end_val = e_r1.as_u8();

        let (min, max) = if start_val < end_val {
            (start_val, end_val)
        } else {
            (end_val, start_val)
        };

        for r in min..=max {
            add_pair_combos(&mut range, Rank::new(r));
        }
    } else {
        // "KJs-K9s"
        // High cards must match? Usually yes. "KJs-QJs" is weird.
        // Assume high cards match: "KJs-K9s".
        // If high cards don't match, like "AK-KQ", that's usually interpreted as a range of strong hands...
        // But standard notation usually implies keeping one card fixed.
        // Let's enforce fixed high card for now.

        // Canonicalize
        let (s_high, s_low) = if s_r1 > s_r2 {
            (s_r1, s_r2)
        } else {
            (s_r2, s_r1)
        };
        let (e_high, e_low) = if e_r1 > e_r2 {
            (e_r1, e_r2)
        } else {
            (e_r2, e_r1)
        };

        if s_high != e_high {
            return Err("Range bounds must share the same high card".into());
        }

        let start_kicker = s_low.as_u8();
        let end_kicker = e_low.as_u8();
        let (min, max) = if start_kicker < end_kicker {
            (start_kicker, end_kicker)
        } else {
            (end_kicker, start_kicker)
        };

        for k in min..=max {
            let kicker = Rank::new(k);
            if kicker == s_high {
                continue;
            } // cannot have same rank if non-pair
            match s_suff {
                Some('s') => add_suited_combos(&mut range, s_high, kicker),
                Some('o') => add_offsuit_combos(&mut range, s_high, kicker),
                None => {
                    add_suited_combos(&mut range, s_high, kicker);
                    add_offsuit_combos(&mut range, s_high, kicker);
                }
                Some(c) => return Err(format!("Invalid suffix: {}", c)),
            }
        }
    }

    Ok(range)
}

fn add_pair_combos(range: &mut HandRange, rank: Rank) {
    // 6 combos: h d, h c, h s, d c, d s, c s
    let suits = [Suit::HEARTS, Suit::DIAMONDS, Suit::CLUBS, Suit::SPADES];
    for i in 0..4 {
        for j in i + 1..4 {
            let c1 = StdDeck::make_card(rank, suits[i]);
            let c2 = StdDeck::make_card(rank, suits[j]);
            let mut mask = StdDeckCardMask::new();
            mask.set(c1);
            mask.set(c2);
            range.push(mask);
        }
    }
}

fn add_suited_combos(range: &mut HandRange, high: Rank, low: Rank) {
    // 4 combos: h h, d d, c c, s s
    let suits = [Suit::HEARTS, Suit::DIAMONDS, Suit::CLUBS, Suit::SPADES];
    for suit in suits {
        let c1 = StdDeck::make_card(high, suit);
        let c2 = StdDeck::make_card(low, suit);
        let mut mask = StdDeckCardMask::new();
        mask.set(c1);
        mask.set(c2);
        range.push(mask);
    }
}

fn add_offsuit_combos(range: &mut HandRange, high: Rank, low: Rank) {
    // 12 combos: h d, h c, h s, d h, d c... (pairs of different suits)
    let suits = [Suit::HEARTS, Suit::DIAMONDS, Suit::CLUBS, Suit::SPADES];
    for i in 0..4 {
        for j in 0..4 {
            if i == j {
                continue;
            }
            let c1 = StdDeck::make_card(high, suits[i]);
            let c2 = StdDeck::make_card(low, suits[j]);
            let mut mask = StdDeckCardMask::new();
            mask.set(c1);
            mask.set(c2);
            range.push(mask);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::StdDeck;

    #[test]
    fn test_hand_range_basic() {
        let mut range = HandRange::new();
        assert!(range.is_empty());

        let (mask, _) = StdDeck::string_to_mask("AsKs").unwrap();
        range.push(mask);

        assert!(!range.is_empty());
        assert_eq!(range.len(), 1);
        assert_eq!(range.hands()[0].0, mask);
    }

    #[test]
    fn test_parse_specific() {
        let range = "AhKh".parse::<HandRange>().unwrap();
        assert_eq!(range.len(), 1);
        assert_eq!(range.hands()[0].0.mask_to_string(), "Kh Ah"); // string_to_mask sort order might vary
    }

    #[test]
    fn test_parse_pair() {
        let range = "TT".parse::<HandRange>().unwrap();
        assert_eq!(range.len(), 6);
    }

    #[test]
    fn test_parse_suited() {
        let range = "AKs".parse::<HandRange>().unwrap();
        assert_eq!(range.len(), 4);
    }

    #[test]
    fn test_parse_offsuit() {
        let range = "AKo".parse::<HandRange>().unwrap();
        assert_eq!(range.len(), 12);
    }

    #[test]
    fn test_parse_generic() {
        let range = "AK".parse::<HandRange>().unwrap();
        assert_eq!(range.len(), 16); // 4 suited + 12 offsuit
    }

    #[test]
    fn test_parse_pair_plus() {
        let range = "JJ+".parse::<HandRange>().unwrap();
        // JJ (6), QQ (6), KK (6), AA (6) = 24
        assert_eq!(range.len(), 24);
    }

    #[test]
    fn test_parse_pair_dash() {
        let range = "88-66".parse::<HandRange>().unwrap();
        // 88, 77, 66 -> 3 * 6 = 18
        assert_eq!(range.len(), 18);
    }

    #[test]
    fn test_parse_suited_plus() {
        let range = "AJs+".parse::<HandRange>().unwrap();
        // AJs, AQs, AKs -> 3 * 4 = 12
        assert_eq!(range.len(), 12);
    }

    #[test]
    fn test_parse_suited_dash() {
        let range = "KJs-K9s".parse::<HandRange>().unwrap();
        // KJs, KTs, K9s -> 3 * 4 = 12
        assert_eq!(range.len(), 12);
    }

    #[test]
    fn test_parse_combined() {
        let range = "JJ+, AKs".parse::<HandRange>().unwrap();
        // JJ+ (24) + AKs (4) = 28
        assert_eq!(range.len(), 28);
    }

    #[test]
    fn test_parse_omaha_hand() {
        let range = "AsKsQsJs".parse::<HandRange>().unwrap();
        assert_eq!(range.len(), 1);
        let (mask, _) = range.hands()[0];
        assert_eq!(mask.num_cards(), 4);
    }
}
