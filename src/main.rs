pub mod poker_hand;

use crate::poker_hand::{PokerHand, PokerHandFast, PokerHandNonFlush};
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
struct PokerHandEvaluator {
    flush_helper: Vec<u64>,
    non_flush_helper: HashMap<PokerHandNonFlush, u64>,
}

impl PokerHandEvaluator {
    fn is_straight(hand: u64) -> bool {
        let helper = 0b11111;
        for i in 0..9 {
            if (hand & (helper << i)).count_ones() == 5 {
                return true;
            }
        }
        return false;
    }

    fn fun(mut generating: Vec<u64>, sum: u64, generated: &mut Vec<PokerHandNonFlush>) {
        if sum == 0 {
            while generating.len() != 13 {
                generating.push(0);
            }
        }
        if generating.len() == 13 {
            if sum == 0 {
                generated.push(PokerHandNonFlush::new(generating));
            }
            return;
        }

        for i in 0..min(sum + 1, 5) {
            generating.push(i);
            PokerHandEvaluator::fun(generating.clone(), sum - i, generated);
            generating.pop();
        }
    }

    fn generate_non_flush_hands() -> Vec<PokerHandNonFlush> {
        let mut generated = vec![];
        PokerHandEvaluator::fun(vec![], 7, &mut generated);
        if generated.len() != 49_205 {
            println!("Tabulka nesedi.");
        }
        generated
    }

    /*fn compare_four_of_a_kind(a: Vec<u64>, b: Vec<u64>) -> Option<Ordering> {
        let res = Option::None;
        for i in (0..13).rev() {
            if a[i] >= 4 || b[i] >= 4 {
                if a[i] < 4 {
                    return Some(Ordering::Less);
                } else if b[i] < 4 {
                    return Some(Ordering::Greater);
                }
                else {}
            }
        }

        return res;
    }*/

    fn sort_non_flush(hands: &mut Vec<PokerHandNonFlush>) {
        /*let foak = vec![];
        let full_house = vec![];
        let straight = vec![];
        let toak = vec![];
        let two_pairs = vec![];
        let pairs = vec![];
        let high_card = vec![];*/

        /*for hand in hands.iter() {
            if hand.is_foak() {
                foak.push(hand);
            } else if hand.is_full_house() {
                full_house.push(hand);
            } else if hand.is_straight() {
                straight.push(hand);
            } else if hand.is_toak() {
                toak.push(hand);
            } else if hand.is_two_pairs() {
                two_pairs.push(hand);
            } else if hand.is_pairs() {
                pairs.push(hand);
            } else {
                high_card.push(hand);
            }
        }*/
    }

    fn prepare_non_flush_table() -> HashMap<PokerHandNonFlush, u64> {
        let mut helper = PokerHandEvaluator::generate_non_flush_hands();

        PokerHandEvaluator::sort_non_flush(&mut helper);

        let mut out = HashMap::new();
        for (index, hand) in helper.iter().enumerate() {
            out.insert(hand.clone(), index as u64);
        }

        out
    }

    fn prepare_flush_table() -> Vec<u64> {
        let mut all_hands = vec![];
        for hand in 0..(1u64 << 13) {
            all_hands.push(hand);
        }

        all_hands.sort_by(|a, b| {
            let a_is_flush = a.count_ones() >= 5;
            let b_is_flush = b.count_ones() >= 5;
            let a_is_straight = PokerHandEvaluator::is_straight(*a);
            let b_is_straight = PokerHandEvaluator::is_straight(*b);
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

    fn new() -> Self {
        Self {
            flush_helper: PokerHandEvaluator::prepare_flush_table(),
            non_flush_helper: PokerHandEvaluator::prepare_non_flush_table(),
        }
    }

    fn eval_flush(&self, h: PokerHandFast) -> u64 {
        self.flush_helper[h.flush_val() as usize]
    }

    fn eval_non_flush(&self, h: PokerHandFast) -> u64 {
        self.non_flush_helper[&h.get_non_flush_repr()]
    }

    fn eval_fast(&self, h: PokerHandFast) -> u64 {
        if h.is_flush() {
            self.eval_flush(h)
        } else {
            self.eval_non_flush(h)
        }
    }
    fn eval(&self, h: PokerHand) -> u64 {
        self.eval_fast(h.get_fast())
    }
}

fn main() {
    let evaluator = PokerHandEvaluator::new();
    let h1 = PokerHand::new([(1, 1), (1, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 8)]);
    let h2 = PokerHand::new([(1, 1), (1, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)]);

    println!("Je to {0} {1}\n", evaluator.eval(h1), evaluator.eval(h2));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_flush() {
        let c1 = [(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)];
        let c2 = [(1, 1), (1, 2), (1, 3), (0, 4), (0, 5), (1, 6), (1, 7)];
        let c3 = [(0, 1), (1, 2), (2, 3), (3, 4), (0, 5), (1, 6), (2, 7)];
        assert_eq!(PokerHand::new(c1).get_fast().is_flush(), true);
        assert_eq!(PokerHand::new(c2).get_fast().is_flush(), true);
        assert_eq!(PokerHand::new(c3).get_fast().is_flush(), false);
    }
    #[test]
    fn is_straight() {
        assert_eq!(PokerHandEvaluator::is_straight(0b0_0000_0000_0000), false);
        assert_eq!(PokerHandEvaluator::is_straight(0b0_1010_1010_1001), false);
        assert_eq!(PokerHandEvaluator::is_straight(0b0_1011_1000_1000), false);
        assert_eq!(PokerHandEvaluator::is_straight(0b1_1111_0000_1000), true);
        assert_eq!(PokerHandEvaluator::is_straight(0b0_1111_1000_1000), true);
    }

    #[test]
    fn eval_flush() {
        let evaluator = PokerHandEvaluator::new();
        let h1 = PokerHand::new([(1, 1), (1, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 8)]);
        let h2 = PokerHand::new([(1, 1), (1, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)]);

        assert!(evaluator.eval(h1) < evaluator.eval(h2));
    }
}
