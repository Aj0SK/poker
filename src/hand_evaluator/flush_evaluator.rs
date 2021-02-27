use super::fast_hand::PokerHandFast;
use super::suit_only_fast_hand::SuitOnlyFastHand;

use std::cmp::Ord;
use std::cmp::Ordering;
use std::convert::Into;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlushEvaluator {
    flush_helper: Vec<u64>,
}

impl FlushEvaluator {
    pub fn new() -> FlushEvaluator {
        FlushEvaluator {
            flush_helper: FlushEvaluator::prepare_flush_table(),
        }
    }

    fn prepare_flush_table() -> Vec<u64> {
        let mut all_hands: Vec<SuitOnlyFastHand> = vec![];
        for hand in 0..(1 << 13) {
            all_hands.push(SuitOnlyFastHand::new(hand));
        }

        all_hands.sort_by(|a, b| {
            let a_is_flush = a.count_ones() >= 5;
            let b_is_flush = b.count_ones() >= 5;
            let a_is_straight = a.is_straight();
            let b_is_straight = b.is_straight();
            if a_is_flush != b_is_flush {
                if a_is_flush {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
            if a_is_straight != b_is_straight {
                if a_is_straight {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }

            a.cmp(b)
        });

        let mut flush_helper = vec![0; 1 << 13];
        for (index, hand) in all_hands.iter().enumerate() {
            let hand_as_number: u64 = (*hand).into();
            flush_helper[hand_as_number as usize] = index as u64;
        }

        flush_helper
    }
    pub fn eval(&self, h: PokerHandFast) -> u64 {
        self.flush_helper[h.flush_val() as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_flush() {
        let evaluator = FlushEvaluator::new();
    }
}
