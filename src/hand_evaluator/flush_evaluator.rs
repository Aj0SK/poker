use super::fast_hand::PokerHandFast;

use std::cmp::Ord;
use std::cmp::Ordering;

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
        let mut all_hands = vec![];
        for hand in 0..(1u64 << 13) {
            all_hands.push(hand);
        }

        all_hands.sort_by(|a, b| {
            let a_is_flush = a.count_ones() >= 5;
            let b_is_flush = b.count_ones() >= 5;
            let a_is_straight = FlushEvaluator::is_straight(*a);
            let b_is_straight = FlushEvaluator::is_straight(*b);
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
            flush_helper[*hand as usize] = index as u64;
        }

        flush_helper
    }

    fn is_straight(hand: u64) -> bool {
        let helper = 0b1_1111;
        for i in 0..9 {
            if (hand & (helper << i)).count_ones() == 5 {
                return true;
            }
        }
        return false;
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
        assert_eq!(FlushEvaluator::is_straight(0b0_0000_0000_0000), false);
        assert_eq!(FlushEvaluator::is_straight(0b0_1010_1010_1001), false);
        assert_eq!(FlushEvaluator::is_straight(0b0_1011_1000_1000), false);
        assert_eq!(FlushEvaluator::is_straight(0b1_1111_0000_1000), true);
        assert_eq!(FlushEvaluator::is_straight(0b0_1111_1000_1000), true);
    }
}
