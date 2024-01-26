use crate::deck_joker::{JOKER_DECK_RANK_2, JOKER_DECK_RANK_ACE, JOKER_DECK_RANK_COUNT};
use crate::handval::HandVal;
use crate::rules_joker::JokerRulesHandType;
use crate::t_jokercardmasks::JokerDeckCardMask;
use crate::t_jokerstraight::JOKER_STRAIGHT_TABLE;
use crate::t_nbits::NBITS_TABLE;
use crate::t_topcard::TOP_CARD_TABLE;
use crate::t_topfivecards::TOP_FIVE_CARDS_TABLE;
use crate::Eval;

pub struct EvalJoker;

fn top_unset(ranks: u32) -> u32 {
    for j in (JOKER_DECK_RANK_2..=JOKER_DECK_RANK_ACE).rev() {
        if ranks & (1 << j) == 0 {
            return j as u32; // Cast j to u32
        }
    }
    JOKER_DECK_RANK_2 as u32 // Cast JOKER_DECK_RANK_2 to u32
}

fn flush_val(ranks: u32) -> HandVal {
    HandVal::new(
        JokerRulesHandType::Flush as u8,
        top_unset(ranks) as u8, // Cast to u8
        0,
        0,
        0,
        0,
    )
}

impl EvalJoker {
    pub fn extract_top_five_cards(suit_mask: u16) -> (u8, u8, u8, u8, u8) {
        let mut cards = Vec::new();
        let mask = suit_mask;

        // Parcourir les bits du masque pour trouver les cartes
        for i in (0..13).rev() {
            if mask & (1 << i) != 0 {
                cards.push(i as u8);
                if cards.len() == 5 {
                    break;
                }
            }
        }

        // Assurez-vous que la liste contient exactement 5 éléments
        while cards.len() < 5 {
            cards.push(0);
        }

        // Renvoie les valeurs des cinq cartes les plus hautes
        (cards[0], cards[1], cards[2], cards[3], cards[4])
    }

    // Helper function to extract top N kickers from a suit mask
    pub fn extract_top_kickers(mask: u64, n: usize) -> [u8; 3] {
        let mut kickers = [0; 3];
        let mut count = 0;

        for i in (0..JOKER_DECK_RANK_COUNT).rev() {
            if mask & (1 << i) != 0 && count < n {
                kickers[count] = i as u8;
                count += 1;
            }
        }

        kickers
    }

    pub fn eval_n(cards: JokerDeckCardMask, n_cards: usize) -> HandVal {
        if !cards.is_joker_set() {
            // Convert to standard deck and use standard evaluation
            let s_cards = cards.to_std();
            return Eval::eval_n(&s_cards, n_cards);
        }

        // If we reach here, we know we have a joker
        let mut ss = cards.spades();
        //println!("Spades: {:b}", ss);
        let mut sc = cards.clubs();
        //println!("Clubs: {:b}", sc);
        let mut sd = cards.diamonds();
        //println!("Diamonds: {:b}", sd);
        let mut sh = cards.hearts();
        //println!("Hearts: {:b}", sh);
        let ranks = ss | sc | sd | sh;
        //println!("Combined ranks: {:b}", ranks);
        let n_ranks = NBITS_TABLE[ranks as usize];
        let mut retval = HandVal::new(0, 0, 0, 0, 0, 0);

        // Check for straight, flush, or straight flush
        if NBITS_TABLE[ss as usize] >= 4 {
            let straight = JOKER_STRAIGHT_TABLE[ss as usize];
            retval = if straight != 0 {
                HandVal::new(JokerRulesHandType::StFlush as u8, straight, 0, 0, 0, 0)
            } else {
                flush_val(ss as u32)
            };
        } else if NBITS_TABLE[sc as usize] >= 4 {
            let straight = JOKER_STRAIGHT_TABLE[ss as usize];
            retval = if straight != 0 {
                HandVal::new(JokerRulesHandType::StFlush as u8, straight, 0, 0, 0, 0)
            } else {
                flush_val(sc as u32)
            };
        } else if NBITS_TABLE[sd as usize] >= 4 {
            let straight = JOKER_STRAIGHT_TABLE[ss as usize];
            retval = if straight != 0 {
                HandVal::new(JokerRulesHandType::StFlush as u8, straight, 0, 0, 0, 0)
            } else {
                flush_val(sd as u32)
            };
        } else if NBITS_TABLE[sh as usize] >= 4 {
            let straight = JOKER_STRAIGHT_TABLE[ss as usize];
            retval = if straight != 0 {
                HandVal::new(JokerRulesHandType::StFlush as u8, straight, 0, 0, 0, 0)
            } else {
                flush_val(sh as u32)
            };
        } else {
            let straight = JOKER_STRAIGHT_TABLE[ranks as usize];
            if straight != 0 {
                retval = HandVal::new(JokerRulesHandType::Straight as u8, straight, 0, 0, 0, 0);
            }
        }

        // Add an Ace if it's not present in any suit
        let jrank = 1 << JOKER_DECK_RANK_ACE as u32;
        if ss & jrank == 0 {
            ss |= jrank;
        } else if sc & jrank == 0 {
            sc |= jrank;
        } else if sd & jrank == 0 {
            sd |= jrank;
        } else if sh & jrank == 0 {
            sh |= jrank;
        } else {
            // All aces are set, so we have quints
            return HandVal::new(
                JokerRulesHandType::Quints as u8,
                JOKER_DECK_RANK_ACE as u8,
                0,
                0,
                0,
                0,
            );
        }

        let ranks = ss | sc | sd | sh;
        let n_ranks = NBITS_TABLE[ranks as usize];
        let n_dups = n_cards - n_ranks as usize;

        match n_dups {
            0 => {
                // If a hand value has already been determined (e.g., for a straight or flush)
                // C'est une main sans paire
                let (top, second, third, fourth, fifth) =
                    Self::extract_top_five_cards(ranks as u16);

                let hand_val = HandVal::new(
                    JokerRulesHandType::NoPair as u8,
                    top,
                    second,
                    third,
                    fourth,
                    fifth,
                );
                //println!("Retourne HandVal pour NoPair: {:?}", hand_val);
                return hand_val;
            }
            1 => {
                // Calculate the hand value for one pair
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                let pair_top_card = TOP_CARD_TABLE[two_mask as usize]; // Assuming implementation of top_card_table
                let t = ranks ^ two_mask;
                let kickers = Self::extract_top_kickers(t, 3); // Assuming implementation of extract_top_kickers

                HandVal::new(
                    JokerRulesHandType::OnePair as u8,
                    pair_top_card as u8,
                    kickers[0],
                    kickers[1],
                    kickers[2],
                    0,
                )
            }
            2 => {
                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);

                if two_mask != 0 {
                    // Two Pair
                    let top_pair = TOP_CARD_TABLE[two_mask as usize];
                    let t = ranks ^ two_mask;
                    let second_pair = TOP_CARD_TABLE[t as usize];
                    let kicker =
                        TOP_CARD_TABLE[(ranks ^ (1 << top_pair) ^ (1 << second_pair)) as usize];

                    HandVal::new(
                        JokerRulesHandType::TwoPair as u8,
                        top_pair as u8,
                        second_pair as u8,
                        kicker as u8,
                        0,
                        0,
                    )
                } else {
                    // Trips
                    let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));
                    let trip_card = TOP_CARD_TABLE[three_mask as usize];
                    let remaining_mask = ranks ^ three_mask;
                    let second = TOP_CARD_TABLE[remaining_mask as usize];
                    let third = TOP_CARD_TABLE[(remaining_mask ^ (1 << second)) as usize];

                    HandVal::new(
                        JokerRulesHandType::Trips as u8,
                        trip_card as u8,
                        second as u8,
                        third as u8,
                        0,
                        0,
                    )
                }
            }
            _ => {
                let four_mask = sh & sd & sc & ss;
                if four_mask != 0 {
                    let tc = TOP_CARD_TABLE[four_mask as usize];
                    return HandVal::new(
                        JokerRulesHandType::Quads as u8,
                        tc as u8,
                        TOP_CARD_TABLE[(ranks ^ (1 << tc)) as usize] as u8,
                        0,
                        0,
                        0,
                    );
                }

                let two_mask = ranks ^ (sc ^ sd ^ sh ^ ss);
                let three_mask = ((sc & sd) | (sh & ss)) & ((sc & sh) | (sd & ss));

                if NBITS_TABLE[two_mask as usize] as usize != n_dups {
                    // Full house
                    let tc = TOP_CARD_TABLE[three_mask as usize];
                    let remaining = (two_mask | three_mask) ^ (1 << tc);
                    HandVal::new(
                        JokerRulesHandType::FullHouse as u8,
                        tc as u8,
                        TOP_CARD_TABLE[remaining as usize] as u8,
                        0,
                        0,
                        0,
                    )
                } else {
                    // Handle other cases where retval is not yet set
                    // You can add logic here to handle cases like two pair, straight, flush, etc.
                    // For now, returning a default value
                    HandVal::new(0, 0, 0, 0, 0, 0)
                }
            }
        }
    }
}
