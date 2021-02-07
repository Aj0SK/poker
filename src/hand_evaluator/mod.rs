use crate::poker_hand::{PokerHand, PokerHandFast, PokerHandNonFlush};
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub struct PokerHandEvaluator {
    flush_helper: Vec<u64>,
    non_flush_helper: HashMap<PokerHandNonFlush, u64>,
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
        fun(generating.clone(), sum - i, generated);
        generating.pop();
    }
}

impl PokerHandEvaluator {
    pub fn new() -> Self {
        Self {
            flush_helper: PokerHandEvaluator::prepare_flush_table(),
            non_flush_helper: PokerHandEvaluator::prepare_non_flush_table(),
        }
    }

    fn is_straight(hand: u64) -> bool {
        let helper = 0b11111;
        for i in 0..9 {
            if (hand & (helper << i)).count_ones() == 5 {
                return true;
            }
        }
        return false;
    }

    fn generate_non_flush_hands() -> Vec<PokerHandNonFlush> {
        let mut generated = vec![];
        fun(vec![], 7, &mut generated);
        if generated.len() != 49_205 {
            println!("Tabulka nesedi.");
        }
        generated
    }

    fn sort_non_flush(hands: &mut Vec<PokerHandNonFlush>) {
        let mut foak_vec = vec![];
        let mut full_house_vec = vec![];
        let mut straight_vec = vec![];
        let mut toak_vec = vec![];
        let mut two_pairs_vec = vec![];
        let mut pairs_vec = vec![];
        let mut high_card_vec = vec![];

        for hand in hands.iter() {
            let foak = hand.foak();
            if foak.is_some() {
                foak_vec.push(foak.unwrap());
                continue;
            }
            let full_house = hand.full_house();
            if full_house.is_some() {
                full_house_vec.push(full_house.unwrap());
                continue;
            }
            let straight = hand.straight();
            if straight.is_some() {
                straight_vec.push(straight.unwrap());
                continue;
            }
            let toak = hand.toak();
            if toak.is_some() {
                toak_vec.push(toak.unwrap());
                continue;
            }
            let two_pairs = hand.two_pairs();
            if two_pairs.is_some() {
                two_pairs_vec.push(two_pairs.unwrap());
                continue;
            }
            let pairs = hand.pair();
            if pairs.is_some() {
                pairs_vec.push(pairs.unwrap());
                continue;
            }
            let high_card = hand.high_card();
            if high_card.is_some() {
                high_card_vec.push(high_card.unwrap());
                continue;
            }
        }

        foak_vec.sort();
        full_house_vec.sort();
        straight_vec.sort();
        toak_vec.sort();
        two_pairs_vec.sort();
        pairs_vec.sort();
        high_card_vec.sort();

        for i in 0..10 {
            println!("Je tu aj {:?}", full_house_vec[i]);
        }
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
    pub fn eval(&self, h: PokerHand) -> u64 {
        self.eval_fast(h.get_fast())
    }
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
