use crate::poker_basics::card::{PokerCard, PokerHand, PokerHandFast};
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;

pub mod ranks;
use ranks::*;

pub mod flush;
pub mod flush_evaluator;
pub mod non_flush_evaluator;

use flush::PokerHandNonFlush;
use flush_evaluator::*;
use non_flush_evaluator::*;

#[derive(Clone, PartialEq, Eq)]
pub struct PokerHandEvaluator {
    flush_evaluator: FlushEvaluator,
    non_flush_helper: NonFlushEvaluator,
}

impl PokerHandEvaluator {
    pub fn new() -> Self {
        Self {
            flush_evaluator: FlushEvaluator::new(),
            non_flush_helper: NonFlushEvaluator::new(),
        }
    }

    fn eval_non_flush(&self, h: PokerHandFast) -> u64 {
        self.non_flush_helper.evaluate(h)
    }

    pub fn eval_fast(&self, h: PokerHandFast) -> u64 {
        if h.is_flush() {
            self.flush_evaluator.eval(h)
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
    use crate::poker_basics::card::*;
    #[test]
    fn is_flush() {
        let c1 = [
            (Suit::Clubs, 1),
            (Suit::Clubs, 2),
            (Suit::Clubs, 3),
            (Suit::Clubs, 4),
            (Suit::Clubs, 5),
            (Suit::Diamonds, 6),
            (Suit::Diamonds, 7),
        ];
        let c2 = [
            (Suit::Clubs, 1),
            (Suit::Clubs, 2),
            (Suit::Diamonds, 3),
            (Suit::Diamonds, 4),
            (Suit::Diamonds, 5),
            (Suit::Diamonds, 6),
            (Suit::Diamonds, 7),
        ];
        let c3 = [
            (Suit::Clubs, 1),
            (Suit::Diamonds, 2),
            (Suit::Hearts, 3),
            (Suit::Spades, 4),
            (Suit::Clubs, 5),
            (Suit::Diamonds, 6),
            (Suit::Hearts, 7),
        ];
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
    fn eval_non_flush() {
        let evaluator = PokerHandEvaluator::new();
        //let c1 = [(Suit::Clubs, 1)]
        //assert!(evaluator.eval(PokerHand::new(c1)) < evaluator.eval(PokerHand::new(c1)));
    }
}
