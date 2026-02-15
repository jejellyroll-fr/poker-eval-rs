//! Open Face Chinese (OFC) Poker engine.
//!
//! OFC is a 13-card poker variant where players arrange cards into three rows:
//! - Top row (3 cards)
//! - Middle row (5 cards)
//! - Bottom row (5 cards)
//!
//! Rules:
//! - Bottom Hand must be stronger than or equal to the Middle Hand.
//! - Middle Hand must be stronger than or equal to the Top Hand.
//! - If these conditions are not met, the hand is a "Foul".

use crate::deck::std_deck::StdDeckCardMask;
use crate::evaluators::Eval;
use crate::handval::HandVal;

/// Represents an OFC board for a single player.
#[derive(Debug, Clone, Default)]
pub struct OFCBoard {
    pub top: StdDeckCardMask,
    pub middle: StdDeckCardMask,
    pub bottom: StdDeckCardMask,
}

/// Results of an OFC evaluation.
#[derive(Debug, Clone, Default)]
pub struct OFCResult {
    pub is_foul: bool,
    pub top_val: HandVal,
    pub middle_val: HandVal,
    pub bottom_val: HandVal,
    pub royalties: i32,
    pub fantasyland: bool,
}

impl OFCBoard {
    /// Evaluates the OFC board.
    pub fn evaluate(&self) -> OFCResult {
        // Top row (3 cards) needs a specialized evaluator because Eval::eval_n expects 5-7 cards.
        let top_val = self.eval_3_cards(&self.top);
        let middle_val = Eval::eval_n(&self.middle, 5);
        let bottom_val = Eval::eval_n(&self.bottom, 5);

        // Foul logic: Bottom >= Middle >= Top
        let is_foul = !(bottom_val.value >= middle_val.value && middle_val.value >= top_val.value);

        let mut result = OFCResult {
            is_foul,
            top_val,
            middle_val,
            bottom_val,
            royalties: 0,
            fantasyland: false,
        };

        if !is_foul {
            result.royalties = self.calculate_royalties(&top_val, &middle_val, &bottom_val);
            // Fantasyland: QQ or better on top
            let top_type = top_val.hand_type();
            let top_rank = top_val.top_card();
            if top_type > 1 || (top_type == 1 && top_rank >= 10) {
                result.fantasyland = true;
            }
        }

        result
    }

    /// Helper to evaluate 3 cards for OFC Top Row.
    fn eval_3_cards(&self, mask: &StdDeckCardMask) -> HandVal {
        let ss = mask.spades();
        let sc = mask.clubs();
        let sd = mask.diamonds();
        let sh = mask.hearts();
        let ranks = ss | sc | sd | sh;

        // Trips: Bit in 3+ suits
        let i_sc_sd = sc & sd;
        let i_sh_ss = sh & ss;
        let trips_mask = (i_sc_sd & (sh | ss)) | (i_sh_ss & (sc | sd));
        if trips_mask != 0 {
            let rank = 15 - (trips_mask.leading_zeros() as u8);
            return HandVal::new(3, rank, 0, 0, 0, 0); // Trips
        }

        // Pair: Bit in exactly 2 suits
        let pair_mask = (ss & sc) | (ss & sd) | (ss & sh) | (sc & sd) | (sc & sh) | (sd & sh);
        if pair_mask != 0 {
            let pair_rank = 15 - (pair_mask.leading_zeros() as u8);
            let kicker_mask = ranks ^ pair_mask;
            let kicker_rank = if kicker_mask != 0 {
                15 - (kicker_mask.leading_zeros() as u8)
            } else {
                0
            };
            return HandVal::new(1, pair_rank, kicker_rank, 0, 0, 0); // OnePair
        }

        // High Card
        let mut r = ranks;
        let r1 = if r != 0 {
            15 - (r.leading_zeros() as u8)
        } else {
            0
        };
        r ^= if r != 0 { 1 << r1 } else { 0 };
        let r2 = if r != 0 {
            15 - (r.leading_zeros() as u8)
        } else {
            0
        };
        r ^= if r != 0 { 1 << r2 } else { 0 };
        let r3 = if r != 0 {
            15 - (r.leading_zeros() as u8)
        } else {
            0
        };

        HandVal::new(0, r1, r2, r3, 0, 0)
    }

    /// Calculates royalties for a non-fouled hand.
    fn calculate_royalties(&self, top: &HandVal, mid: &HandVal, bot: &HandVal) -> i32 {
        let mut total = 0;

        // Top Row (3 cards)
        let top_type = top.hand_type();
        let top_rank = top.top_card();
        if top_type == 3 {
            // Trips: 222=10, ..., AAA=22
            total += 10 + (top_rank as i32);
        } else if top_type == 1 {
            // Pair: 66=1, ..., AA=9
            if top_rank >= 4 {
                total += (top_rank as i32) - 3;
            }
        }

        // Middle Row (5 cards)
        let mid_type = mid.hand_type();
        match mid_type {
            3 => total += 2,  // Trips
            4 => total += 4,  // Straight
            5 => total += 8,  // Flush
            6 => total += 12, // Full House
            7 => total += 20, // Quads
            8 => {
                // Straight Flush
                // Check if it's the 10th straight flush (Royal Flush)
                let monotonic_val = mid.value & 0xFFFFFF;
                if monotonic_val == 10 {
                    total += 50; // Royal
                } else {
                    total += 30; // StFlush
                }
            }
            _ => {}
        }

        // Bottom Row (5 cards)
        let bot_type = bot.hand_type();
        match bot_type {
            4 => total += 2,  // Straight
            5 => total += 4,  // Flush
            6 => total += 6,  // Full House
            7 => total += 10, // Quads
            8 => {
                // Straight Flush
                let monotonic_val = bot.value & 0xFFFFFF;
                if monotonic_val == 10 {
                    total += 25; // Royal
                } else {
                    total += 15; // StFlush
                }
            }
            _ => {}
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::StdDeck;

    #[test]
    fn test_ofc_basic_valid() {
        let board = OFCBoard {
            bottom: StdDeck::string_to_mask("As Ks Qs Js 9s").unwrap().0,
            middle: StdDeck::string_to_mask("8h 7c 6s 5d 4h").unwrap().0,
            top: StdDeck::string_to_mask("Jc Jd 2h").unwrap().0,
        };

        let result = board.evaluate();
        assert!(!result.is_foul);
        // Bottom Flush = 4 royalties
        // Middle Straight = 4 royalties
        // Top Pair of Jacks (rank 9) = 9-3 = 6 royalties
        // Total = 14
        assert_eq!(result.royalties, 14);
        assert!(!result.fantasyland); // JJ is not enough (balanced variant usually QQ)
    }

    #[test]
    fn test_ofc_foul() {
        let board = OFCBoard {
            bottom: StdDeck::string_to_mask("As 2s 3s 4s 6d").unwrap().0,
            middle: StdDeck::string_to_mask("Ks Kd 3s 4s 5s").unwrap().0,
            top: StdDeck::string_to_mask("Qh 2h 3h").unwrap().0,
        };

        let result = board.evaluate();
        assert!(result.is_foul);
        assert_eq!(result.royalties, 0);
    }

    #[test]
    fn test_ofc_fantasyland() {
        let board = OFCBoard {
            bottom: StdDeck::string_to_mask("As Ks Qs Js Ts").unwrap().0,
            middle: StdDeck::string_to_mask("Ah Ad As Kh Kd").unwrap().0,
            top: StdDeck::string_to_mask("Qc Qd 2h").unwrap().0,
        };

        let result = board.evaluate();
        assert!(!result.is_foul);
        assert!(result.fantasyland);
        // Bottom Royal = 25
        // Middle Full House = 12
        // Top QQ (rank 10) = 10-3 = 7
        // Total = 44
        assert_eq!(result.royalties, 44);
    }
}
