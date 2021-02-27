use super::fast_hand::PokerHandFast;

use std::cmp::Ord;
use std::cmp::Ordering;
use std::convert::Into;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlushEvaluator {
    flush_helper: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct SuitOnlyFastHand(u64);

impl Into<u64> for SuitOnlyFastHand {
    fn into(self) -> u64 {
        self.0
    }
}

impl SuitOnlyFastHand {
    pub fn is_straight(self) -> bool {
        let helper = 0b11111;
        for i in 0..(13 - 4) {
            if (self.0 & (helper << i)).count_ones() == 5 {
                return true;
            }
        }
        return false;
    }
    pub fn count_ones(self) -> u64 {
        self.0.count_ones() as u64
    }
}

impl FlushEvaluator {
    pub fn new() -> FlushEvaluator {
        FlushEvaluator {
            flush_helper: FlushEvaluator::prepare_flush_table(),
        }
    }

    fn prepare_flush_table() -> Vec<u64> {
        let mut all_hands: Vec<SuitOnlyFastHand> = vec![];
        for hand in 0..(1u64 << 13) {
            all_hands.push(SuitOnlyFastHand(hand));
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
            let hand_index: u64 = (*hand).into();
            flush_helper[hand_index as usize] = index as u64;
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
    fn is_straight() {
        assert_eq!(SuitOnlyFastHand(0b0_0000_0000_0000).is_straight(), false);
        assert_eq!(SuitOnlyFastHand(0b0_1010_1010_1001).is_straight(), false);
        assert_eq!(SuitOnlyFastHand(0b0_1011_1000_1000).is_straight(), false);
        assert_eq!(SuitOnlyFastHand(0b1_1111_0000_1000).is_straight(), true);
        assert_eq!(SuitOnlyFastHand(0b0_1111_1000_1000).is_straight(), true);
    }
}
