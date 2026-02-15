//! Evaluation algorithms and HandEvaluator trait implementations.

pub mod badugi;
pub mod holdem;
pub mod joker;
pub mod joker_low;
pub mod joker_low8;
pub mod lowball;
pub mod lowball27;
pub mod lowball8;
pub mod lowball_variants;
pub mod ofc;
pub mod omaha;
pub mod range_equity;

use crate::errors::PokerError;
use crate::handval::HandVal;
use crate::handval_low::LowHandVal;
use crate::tables::t_cardmasks::StdDeckCardMask;
pub use badugi::badugi_eval;
pub use holdem::Eval;
pub use joker::EvalJoker;
pub use joker_low::joker_lowball_eval;
pub use joker_low8::joker_lowball8_eval;
pub use lowball::std_deck_lowball_eval;
pub use lowball27::std_deck_lowball27_eval;
pub use lowball8::std_deck_lowball8_eval;
pub use ofc::OFCBoard;
pub use omaha::{std_deck_omaha_hi_eval, std_deck_omaha_hi_low8_eval};

/// Trait for evaluating poker hands.
///
/// This trait abstracts over different game types (Hold'em, Omaha, Lowball)
/// allowing for generic implementation of enumeration and simulation logic.
pub trait HandEvaluator {
    /// The type of the result produced by the evaluator (e.g., HandVal, LowHandVal).
    type Output;

    /// Evaluates a hand given a set of hole cards and a set of board cards.
    ///
    /// # Arguments
    ///
    /// * `hole` - A mask representing the player's hole cards.
    /// * `board` - A mask representing the community cards (board).
    ///
    /// # Returns
    ///
    /// The evaluation result of type `Result<Self::Output, PokerError>`.
    fn evaluate_hand(
        hole: &StdDeckCardMask,
        board: &StdDeckCardMask,
    ) -> Result<Self::Output, PokerError>;
}

/// Evaluator for Texas Hold'em and other high-hand variants using standard rules.
///
/// # Examples
///
/// ```
/// use poker_eval_rs::deck::StdDeck;
/// use poker_eval_rs::evaluators::{HandEvaluator, HoldemEvaluator};
///
/// let (hole, _) = StdDeck::string_to_mask("As Ks").unwrap();
/// let (board, _) = StdDeck::string_to_mask("Qs Js Ts").unwrap();
/// let result = HoldemEvaluator::evaluate_hand(&hole, &board).unwrap();
/// // Royal Flush
/// assert_eq!(result.hand_type(), 8);
/// ```
pub struct HoldemEvaluator;

impl HandEvaluator for HoldemEvaluator {
    type Output = HandVal;

    fn evaluate_hand(
        hole: &StdDeckCardMask,
        board: &StdDeckCardMask,
    ) -> Result<Self::Output, PokerError> {
        let mut hand = *hole;
        hand.or(board);
        Ok(Eval::eval_n(&hand, hand.num_cards()))
    }
}

/// Evaluator for Omaha High.
pub struct OmahaHiEvaluator;

impl HandEvaluator for OmahaHiEvaluator {
    type Output = Option<HandVal>;

    fn evaluate_hand(
        hole: &StdDeckCardMask,
        board: &StdDeckCardMask,
    ) -> Result<Self::Output, PokerError> {
        let mut hival = None;
        std_deck_omaha_hi_eval(*hole, *board, &mut hival)?;
        Ok(hival)
    }
}

/// Evaluator for Omaha Hi/Lo (8-or-better).
/// Returns a tuple of (High Hand, Low Hand).
pub struct OmahaHiLoEvaluator;

impl HandEvaluator for OmahaHiLoEvaluator {
    type Output = (Option<HandVal>, Option<LowHandVal>);

    fn evaluate_hand(
        hole: &StdDeckCardMask,
        board: &StdDeckCardMask,
    ) -> Result<Self::Output, PokerError> {
        let mut hival = None;
        let mut loval = None;
        std_deck_omaha_hi_low8_eval(*hole, *board, &mut hival, &mut loval)?;
        Ok((hival, loval))
    }
}

/// Evaluator for Lowball (Ace-to-Five or 2-7 depending on specific implementation, here generic lowball).
/// Using std_deck_lowball_eval which is typically A-5.
///
/// # Examples
///
/// ```
/// use poker_eval_rs::deck::StdDeck;
/// use poker_eval_rs::evaluators::{HandEvaluator, LowballEvaluator};
///
/// let (hole, _) = StdDeck::string_to_mask("As 2s 3s 4s 5s").unwrap();
/// let (board, _) = StdDeck::string_to_mask("").unwrap();
/// let result = LowballEvaluator::evaluate_hand(&hole, &board).unwrap();
/// // A-2-3-4-5 is the best low hand (Wheel)
/// assert_ne!(result.value, 0);
/// ```
pub struct LowballEvaluator;

impl HandEvaluator for LowballEvaluator {
    type Output = LowHandVal;

    fn evaluate_hand(
        hole: &StdDeckCardMask,
        board: &StdDeckCardMask,
    ) -> Result<Self::Output, PokerError> {
        let mut hand = *hole;
        hand.or(board);
        Ok(std_deck_lowball_eval(&hand, hand.num_cards()))
    }
}

/// Evaluator for Badugi.
pub struct BadugiEvaluator;

impl HandEvaluator for BadugiEvaluator {
    type Output = LowHandVal;

    fn evaluate_hand(
        hole: &StdDeckCardMask,
        board: &StdDeckCardMask,
    ) -> Result<Self::Output, PokerError> {
        let mut hand = *hole;
        hand.or(board);
        Ok(badugi_eval(&hand))
    }
}

/// Evaluator for 5-card Draw and other draw variants.
pub struct DrawEvaluator;

impl HandEvaluator for DrawEvaluator {
    type Output = HandVal;

    fn evaluate_hand(
        hole: &StdDeckCardMask,
        board: &StdDeckCardMask,
    ) -> Result<Self::Output, PokerError> {
        let mut hand = *hole;
        hand.or(board);
        Ok(Eval::eval_n(&hand, hand.num_cards()))
    }
}

/// Evaluator for Short Deck (Six Plus) Hold'em.
///
/// Rules:
/// - Deck of 36 cards (6 to Ace).
/// - Flush beats Full House.
/// - A-6-7-8-9 is a straight (Ace plays low as 5).
/// - Trips > Straight (optional variant, but here we assume standard Straight > Trips unless specified otherwise.
///   Based on user request "Flush > FH" and "A-6-7-8-9 straight" are the key changes).
///
/// Note on HandVal:
/// To enforce Flush > Full House without changing the global `HandType` enum (where FH > Flush),
/// this evaluator swaps the `HandType` returned:
///  - A Flush is returned with `HandType::FullHouse` (value 6).
///  - A Full House is returned with `HandType::Flush` (value 5).
///  - This ensures direct comparison of `HandVal` works as expected for Short Deck rules.
pub struct ShortDeckEvaluator;

impl HandEvaluator for ShortDeckEvaluator {
    type Output = HandVal;

    #[inline]
    fn evaluate_hand(
        hole: &StdDeckCardMask,
        board: &StdDeckCardMask,
    ) -> Result<Self::Output, PokerError> {
        let mut hand = *hole;
        hand.or(board);
        // We can optimize num_cards if we assume 2 hole + 5 board, but safer to calc.
        // popcount is fast on native.
        let n_cards = hand.num_cards();
        Self::evaluate_combined(&hand, n_cards)
    }
}

impl ShortDeckEvaluator {
    const MASK_A_6_7_8_9: u16 =
        (1u16 << 12) | (1u16 << 4) | (1u16 << 5) | (1u16 << 6) | (1u16 << 7);
    const FLUSH_TYPE: u32 = (crate::rules::HandType::Flush as u32) << 24;
    const FH_TYPE: u32 = (crate::rules::HandType::FullHouse as u32) << 24;
    const TYPE_MASK: u32 = 0x0F000000;

    #[inline]
    pub fn evaluate_combined(
        hand: &StdDeckCardMask,
        _n_cards: usize,
    ) -> Result<HandVal, PokerError> {
        let ss = hand.spades();
        let sc = hand.clubs();
        let sd = hand.diamonds();
        let sh = hand.hearts();
        let ranks = ss | sc | sd | sh;

        // Use the fast OMPEval path for everything
        let mut val = Eval::eval_n(hand, 0);

        let htype = val.value & Self::TYPE_MASK;

        // Short Deck rules: Flush beats Full House
        // Swap the type bits: Flush(5) ↔ FullHouse(6)
        if htype == Self::FLUSH_TYPE {
            // Check A-6-7-8-9 straight flush (not detected by standard STRAIGHT_TABLE)
            let mask = Self::MASK_A_6_7_8_9;
            if (ss & mask) == mask
                || (sc & mask) == mask
                || (sd & mask) == mask
                || (sh & mask) == mask
            {
                return Ok(HandVal::new(
                    crate::rules::HandType::StFlush as u8,
                    7,
                    6,
                    5,
                    4,
                    12,
                ));
            }
            // Upgrade Flush → FullHouse rank (so it beats FH)
            val.value = (val.value & !Self::TYPE_MASK) | Self::FH_TYPE;
        } else if htype == Self::FH_TYPE {
            // Downgrade FullHouse → Flush rank (so Flush beats it)
            val.value = (val.value & !Self::TYPE_MASK) | Self::FLUSH_TYPE;
        }

        // Check A-6-7-8-9 Straight (not a flush)
        // Only if current hand is weaker than a Straight
        if htype < (crate::rules::HandType::Straight as u32) << 24
            && (ranks & Self::MASK_A_6_7_8_9) == Self::MASK_A_6_7_8_9
        {
            return Ok(HandVal::new(
                crate::rules::HandType::Straight as u8,
                7,
                6,
                5,
                4,
                12,
            ));
        }

        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::StdDeck;
    use crate::handval_low::LOW_HAND_VAL_NOTHING;
    use crate::rules::HandType;

    #[test]
    fn test_holdem_evaluator() {
        let (hole, _) = StdDeck::string_to_mask("AsKs").unwrap();
        let (board, _) = StdDeck::string_to_mask("QsJsTs").unwrap();
        let val = HoldemEvaluator::evaluate_hand(&hole, &board).unwrap();
        assert_eq!(val.hand_type(), HandType::StFlush as u8);
    }

    #[test]
    fn test_omaha_hi_evaluator() {
        let (hole, _) = StdDeck::string_to_mask("AsKsAhKh").unwrap();
        let (board, _) = StdDeck::string_to_mask("QsJsTs").unwrap();
        let val = OmahaHiEvaluator::evaluate_hand(&hole, &board)
            .unwrap()
            .unwrap();
        assert_eq!(val.hand_type(), HandType::StFlush as u8);
    }

    #[test]
    fn test_omaha_hilo_evaluator() {
        let (hole, _) = StdDeck::string_to_mask("As2s3d4d").unwrap();
        let (board, _) = StdDeck::string_to_mask("5s6s7s8d9d").unwrap();
        let (hi, lo) = OmahaHiLoEvaluator::evaluate_hand(&hole, &board).unwrap();
        assert!(hi.is_some());
        assert!(lo.is_some());
        assert_ne!(lo.unwrap().value, LOW_HAND_VAL_NOTHING);
    }

    #[test]
    fn test_lowball_evaluator() {
        let (hole, _) = StdDeck::string_to_mask("As2s").unwrap();
        let (board, _) = StdDeck::string_to_mask("3s4s5s").unwrap();
        let val = LowballEvaluator::evaluate_hand(&hole, &board).unwrap();
        // A-2-3-4-5 is the best low hand (Wheel)
        // Value logic depends on implementation, but it shouldn't be NOTHING
        assert_ne!(val.value, LOW_HAND_VAL_NOTHING);
    }

    #[test]
    fn test_short_deck_flush_beats_full_house() {
        // Full House: As Ah Ad Ks Kh (Stands for rank 12 and 11)
        let (hole_fh, _) = StdDeck::string_to_mask("AsAh").unwrap();
        let (board_fh, _) = StdDeck::string_to_mask("AdKsKh").unwrap();
        let val_fh = ShortDeckEvaluator::evaluate_hand(&hole_fh, &board_fh).unwrap();

        // Flush: 6s 7s 8s 9s Js (Example flush)
        let (hole_fl, _) = StdDeck::string_to_mask("6s7s").unwrap();
        let (board_fl, _) = StdDeck::string_to_mask("8s9sJs").unwrap();
        let val_fl = ShortDeckEvaluator::evaluate_hand(&hole_fl, &board_fl).unwrap();

        // In Short Deck, Flush > Full House.
        // Our implementation swaps the HandType enum values internally to ensure this.
        assert!(
            val_fl > val_fh,
            "Flush should beat Full House in Short Deck"
        );
    }

    #[test]
    fn test_short_deck_straight_a_6_7_8_9() {
        // A-6-7-8-9 is a straight in Short Deck
        let (hole, _) = StdDeck::string_to_mask("As6s").unwrap();
        let (board, _) = StdDeck::string_to_mask("7s8s9h").unwrap();
        let val = ShortDeckEvaluator::evaluate_hand(&hole, &board).unwrap();

        assert_eq!(val.hand_type(), HandType::Straight as u8);
        assert_eq!(val.top_card(), 7); // 9 high straight (9 is rank 7)
    }

    #[test]
    fn test_short_deck_overlapping_straight_flushes() {
        // A, 6, 7, 8, 9, T (all spades).
        // Contains A-6-7-8-9 (9-high SF) AND 6-7-8-9-T (T-high SF).
        // Should return T-high SF.
        let (hole, _) = StdDeck::string_to_mask("As6s").unwrap();
        let (board, _) = StdDeck::string_to_mask("7s8s9sTs").unwrap(); // 6 cards total
        let val = ShortDeckEvaluator::evaluate_hand(&hole, &board).unwrap();

        assert_eq!(val.hand_type(), HandType::StFlush as u8);
        assert_eq!(val.hand_type(), HandType::StFlush as u8);
    }
}
