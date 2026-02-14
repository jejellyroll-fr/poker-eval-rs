//! Board texture analysis and outs calculation.
//!
//! Provides tools to analyze the board state (flush draws, straight draws, paired board)
//! and calculate outs for specific hands.

use crate::deck::{StdDeckCardMask, STD_DECK_N_CARDS, STD_DECK_RANK_COUNT};
use crate::evaluators::Eval;
use crate::rules::HandType;

/// Represents the texture of a board (flop, turn, or river).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardTexture {
    /// True if all cards are of different suits.
    pub is_rainbow: bool,
    /// True if exactly 2 suits are present on the board.
    pub is_two_tone: bool,
    /// True if all cards are of the same suit.
    pub is_monotone: bool,
    /// True if at least one pair is present on the board.
    pub is_paired: bool,
    /// True if three of a kind is present on the board.
    pub is_trips: bool,
    /// True if four of a kind is present on the board (unlikely but possible on turn/river).
    pub is_quads: bool,
    /// True if a full house is present on the board.
    pub is_full_house: bool,
    /// True if 3 or more cards are connected (possible straight).
    pub has_straight_draw: bool,
    /// True if 3 or more cards are of the same suit (flush draw or made flush).
    pub has_flush_draw: bool,
}

impl BoardTexture {
    /// Analyzes the given board mask and returns its texture.
    ///
    /// # Examples
    ///
    /// ```
    /// use poker_eval_rs::board::BoardTexture;
    /// use poker_eval_rs::deck::StdDeck;
    ///
    /// let (board, _) = StdDeck::string_to_mask("As Ks Qs").unwrap();
    /// let texture = BoardTexture::analyze(&board);
    /// assert!(texture.is_monotone);
    /// assert!(texture.has_flush_draw); // 3 to a flush
    /// ```
    pub fn analyze(board: &StdDeckCardMask) -> Self {
        let mut texture = BoardTexture {
            is_rainbow: false,
            is_two_tone: false,
            is_monotone: false,
            is_paired: false,
            is_trips: false,
            is_quads: false,
            is_full_house: false,
            has_straight_draw: false,
            has_flush_draw: false,
        };

        let n = board.num_cards();
        if n < 3 {
            // Not enough cards for meaningful texture analysis?
            // Or just return default false.
            return texture;
        }

        // Analyze Suits
        let mut suit_counts = [0; 4];
        let mut rank_counts = [0; STD_DECK_RANK_COUNT];
        let mut ranks_present = Vec::with_capacity(n);

        for i in 0..STD_DECK_N_CARDS {
            if board.card_is_set(i) {
                let card_idx = i;
                let rank = card_idx % STD_DECK_RANK_COUNT;
                let suit = card_idx / STD_DECK_RANK_COUNT;

                suit_counts[suit] += 1;
                rank_counts[rank] += 1;
                ranks_present.push(rank);
            }
        }

        // Suits logic
        let mut suits_present = 0;
        let mut max_suit_count = 0;
        for &count in &suit_counts {
            if count > 0 {
                suits_present += 1;
                if count > max_suit_count {
                    max_suit_count = count;
                }
            }
        }

        // Rainbow means each card has a different suit (max 4 suits available)
        // Flop (3 cards): 3 distinct suits. Turn (4 cards): 4 distinct suits.
        // River (5 cards): cannot be rainbow (only 4 suits exist).
        if suits_present == n && n <= 4 {
            texture.is_rainbow = true;
        }

        if suits_present == 2 {
            texture.is_two_tone = true;
        } else if suits_present == 1 {
            texture.is_monotone = true;
        }

        if max_suit_count >= 3 {
            texture.has_flush_draw = true; // 3 to a flush is a draw (or made flush)
        }

        // Ranks logic (Paired, Trips, etc.)
        let mut pair_count = 0;
        let mut three_count = 0;
        let mut four_count = 0;

        for &count in &rank_counts {
            if count == 2 {
                pair_count += 1;
            } else if count == 3 {
                three_count += 1;
            } else if count == 4 {
                four_count += 1;
            }
        }

        if pair_count > 0 {
            texture.is_paired = true;
        }
        if three_count > 0 {
            texture.is_trips = true;
        }
        if four_count > 0 {
            texture.is_quads = true;
        }
        if three_count > 0 && pair_count > 0 {
            texture.is_full_house = true;
        }

        // Straight logic (Connectivity)
        // Check for 3 connected cards (allowing for A-low and A-high)
        // Helper to check connectivity in sorted unique ranks
        ranks_present.sort_unstable();

        // Remove duplicates for straight check
        let mut unique_ranks = ranks_present.clone();
        unique_ranks.dedup();

        if unique_ranks.len() >= 3 {
            // Check for 3 consecutive ranks
            // Standard check
            let mut consecutive = 1;
            for i in 0..unique_ranks.len() - 1 {
                if unique_ranks[i + 1] == unique_ranks[i] + 1 {
                    consecutive += 1;
                    if consecutive >= 3 {
                        texture.has_straight_draw = true;
                        break;
                    }
                } else {
                    consecutive = 1;
                }
            }

            // Ace special case (A-2-3)
            // A is rank 12 (if 0-12). 2 is rank 0.
            // My Rank: 2=0, 3=1, ..., A=12?
            // std_deck.rs: Rank(0) = '2', Rank(12) = 'A'.
            // So A=12, 2=0, 3=1.
            // If we have A, 2, 3 -> 12, 0, 1.
            // unique_ranks sorted: 0, 1, ..., 12.
            // Check if we have 0, 1, 12 present?
            if !texture.has_straight_draw {
                // Check wheel draw elements: A(12), 2(0), 3(1), 4(2), 5(3)
                // If we have 3 of these?
                // No, specifically connected.
                // Wraparound logic?
                // A, 2, 3 is connected.
                let has_ace = unique_ranks.contains(&12);
                let has_two = unique_ranks.contains(&0);
                let has_three = unique_ranks.contains(&1);

                if has_ace && has_two && has_three {
                    texture.has_straight_draw = true;
                }
            }
        }

        texture
    }
}

/// Result of an outs calculation, grouping improving cards by the hand type they achieve.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutsResult {
    /// Maps each HandType to a list of cards that achieve it.
    /// Index is HandType as usize (0..=8).
    /// values are masks of single cards.
    pub outs_by_type: Vec<Vec<StdDeckCardMask>>,
}

impl OutsResult {
    pub fn new() -> Self {
        Self {
            outs_by_type: vec![Vec::new(); 9], // 9 hand types in standard poker
        }
    }
}

impl Default for OutsResult {
    fn default() -> Self {
        Self::new()
    }
}

impl OutsResult {
    pub fn add(&mut self, hand_type: crate::rules::HandType, card: StdDeckCardMask) {
        let idx = hand_type as usize;
        if idx < self.outs_by_type.len() {
            self.outs_by_type[idx].push(card);
        }
    }

    pub fn count(&self, hand_type: crate::rules::HandType) -> usize {
        let idx = hand_type as usize;
        if idx < self.outs_by_type.len() {
            self.outs_by_type[idx].len()
        } else {
            0
        }
    }
}

/// Calculates outs for a given pocket and board.
///
/// Returns an `OutsResult` containing cards that improve the hand to a higher `HandType`
/// than currently held.
///
/// # Examples
///
/// ```
/// use poker_eval_rs::board::calculate_outs;
/// use poker_eval_rs::deck::StdDeck;
/// use poker_eval_rs::rules::HandType;
///
/// // Pocket: 2s 3s. Board: As Ks 9d. (Flush draw)
/// let (pocket, _) = StdDeck::string_to_mask("2s3s").unwrap();
/// let (board, _) = StdDeck::string_to_mask("AsKs9d").unwrap();
///
/// let outs = calculate_outs(&pocket, &board);
/// // 9 spades remaining in deck to make a Flush
/// assert_eq!(outs.count(HandType::Flush), 9);
/// ```
pub fn calculate_outs(pocket: &StdDeckCardMask, board: &StdDeckCardMask) -> OutsResult {
    let mut results = OutsResult::new();

    // Evaluate current hand
    let current_hand = *pocket | *board;
    // We assume 5, 6, or 7 card evaluation support.
    // Standard Hold'em: 2 pocket + 3/4 board.
    // If board has 5 cards, there are no outs coming (unless river -> 6th card? No).
    // Usually we calculate outs on Flop (2+3=5 cards) -> Turn (6th)
    // Or Turn (2+4=6 cards) -> River (7th).

    let n_cards = current_hand.num_cards();
    if n_cards >= 7 {
        return results; // No more cards to come
    }

    let current_val = Eval::eval_n(&current_hand, n_cards);
    let current_type = current_val.hand_type();

    // Iterate all remaining cards
    let dead = current_hand; // pocket | board

    // We iterate 0..52. If not in dead, check it.
    for i in 0..STD_DECK_N_CARDS {
        if !dead.card_is_set(i) {
            let card_mask = StdDeckCardMask::from_card_index(i);
            let next_hand = current_hand | card_mask;
            let next_val = Eval::eval_n(&next_hand, n_cards + 1);
            let next_type = HandType::from_usize(next_val.hand_type() as usize).unwrap();

            if next_type.as_usize() > (current_type as usize) {
                results.add(next_type, card_mask);
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::StdDeck;
    use crate::rules::HandType;

    #[test]
    fn test_analyze_board_rainbow() {
        // As 5d 9c (3 suits)
        let (board, _) = StdDeck::string_to_mask("As5d9c").unwrap();
        let texture = BoardTexture::analyze(&board);
        assert!(texture.is_rainbow);
        assert!(!texture.is_two_tone);
        assert!(!texture.is_monotone);
        assert!(!texture.is_paired);
    }

    #[test]
    fn test_analyze_board_monotone() {
        // As 5s 9s
        let (board, _) = StdDeck::string_to_mask("As5s9s").unwrap();
        let texture = BoardTexture::analyze(&board);
        assert!(texture.is_monotone);
        assert!(texture.has_flush_draw); // 3 to a flush
    }

    #[test]
    fn test_analyze_board_paired() {
        // As Ad 9c
        let (board, _) = StdDeck::string_to_mask("AsAd9c").unwrap();
        let texture = BoardTexture::analyze(&board);
        assert!(texture.is_paired);
    }

    #[test]
    fn test_analyze_board_full_texture_check() {
        // Two Tone, Trips: As Ad Ac 5s 2s
        // Suits: s=3 (A,5,2), d=1 (A), c=1 (A). Two tone? No, 3 suits present?
        // Wait, As, Ad, Ac, 5s, 2s.
        // Suits: s (A,5,2) -> 3. d (A) -> 1. c (A) -> 1.
        // Distinct suits: s, d, c -> 3.
        // Not Two Tone (needs exactly 2 suits present).
        // Max suit count: 3 (s). -> Flush draw.
        // Ranks: A (3), 5 (1), 2 (1). -> Trips.

        let (board, _) = StdDeck::string_to_mask("AsAdAc5s2s").unwrap();
        let texture = BoardTexture::analyze(&board);

        // Suits: 3 suits present. Not two tone. Not monotone. Not rainbow (5 cards, 3 suits != 5).
        assert!(!texture.is_two_tone);
        assert!(!texture.is_monotone);
        assert!(!texture.is_rainbow);

        assert!(texture.has_flush_draw); // 3 spades
        assert!(texture.is_trips);
        assert!(!texture.is_quads);
        assert!(!texture.is_full_house);
        assert!(!texture.is_paired); // Trips implies pair logic check? analyze() sets paired if count==2.
                                     // Logic: if count==3, three_count++. if count==2, pair_count++.
                                     // Here A=3. pair_count=0. So is_paired=false (correct, strict "pair" vs "trips").
    }

    #[test]
    fn test_analyze_board_full_house() {
        // As Ad Ac Ks Kd
        // A=3, K=2.
        let (board, _) = StdDeck::string_to_mask("AsAdAcKsKd").unwrap();
        let texture = BoardTexture::analyze(&board);

        assert!(texture.is_full_house);
        assert!(texture.is_trips);
        assert!(texture.is_paired);
        assert!(!texture.is_quads);
    }

    #[test]
    fn test_analyze_board_quads() {
        // As Ad Ac Ah Ks
        let (board, _) = StdDeck::string_to_mask("AsAdAcAhKs").unwrap();
        let texture = BoardTexture::analyze(&board);

        assert!(texture.is_quads);
        assert!(!texture.is_trips); // Count==4, not 3.
    }

    #[test]
    fn test_analyze_board_straight_draw() {
        // 9s 8d 7c 2h 2d
        // 7, 8, 9 connected.
        let (board, _) = StdDeck::string_to_mask("9s8d7c2h2d").unwrap();
        let texture = BoardTexture::analyze(&board);

        assert!(texture.has_straight_draw);
    }

    #[test]
    fn test_analyze_board_ace_low_straight_draw() {
        // As 2d 3c 9h 9s
        // A, 2, 3 connected?
        let (board, _) = StdDeck::string_to_mask("As2d3c9h9s").unwrap();
        let texture = BoardTexture::analyze(&board);

        assert!(texture.has_straight_draw);
    }

    #[test]
    fn test_analyze_board_two_tone() {
        // As Ks (s), Qd Jd (d), Ts (s)
        // Suits: s, d. Exactly 2.
        let (board, _) = StdDeck::string_to_mask("AsKsQdJdTs").unwrap();
        let texture = BoardTexture::analyze(&board);

        assert!(texture.is_two_tone);
    }

    #[test]
    fn test_calculate_outs_flush_draw() {
        // Pocket: 2s 3s
        // Board: As Ks 9d
        // Current: Pair of nothing? High card A.
        // Outs to Flush: any Spade.
        // 13 spades total. We have 2+2=4 spades visible (2s, 3s, As, Ks).
        // Wait, pocket 2s 3s. Board As Ks 9d.
        // Visible: 2s, 3s, As, Ks. 4 spades.
        // Remaining spades: 13 - 4 = 9.
        // Any remaining spade makes a Flush.

        let (pocket, _) = StdDeck::string_to_mask("2s3s").unwrap();
        let (board, _) = StdDeck::string_to_mask("AsKs9d").unwrap();

        let outs = calculate_outs(&pocket, &board);

        // Count flush outs
        let flush_outs = outs.count(HandType::Flush);
        assert_eq!(flush_outs, 9);
    }

    #[test]
    fn test_calculate_outs_straight() {
        // Pocket: 9c Tc (Open ended straight draw: 8 or K)
        // Board: Jd Qd 2s
        // Hand so far: 9, T, J, Q. Needs 8 or K.
        // 4 eights, 4 kings. Total 8 outs.
        // Note: Kd makes straight. 8d makes straight.
        // Also check flush draw? No, different suits.

        let (pocket, _) = StdDeck::string_to_mask("9cTc").unwrap();
        let (board, _) = StdDeck::string_to_mask("JdQd2s").unwrap();

        let outs = calculate_outs(&pocket, &board);

        let straight_outs = outs.count(HandType::Straight);
        assert_eq!(straight_outs, 8);
    }
}
