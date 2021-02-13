use crate::hand_evaluator::fast_hand::*;
use crate::poker_basics::card::PokerHand;

pub mod fast_hand;
pub mod flush;
pub mod flush_evaluator;
pub mod non_flush_evaluator;
pub mod ranks;

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

    pub fn eval_fast(&self, h: PokerHandFast) -> u64 {
        if h.is_flush() {
            self.flush_evaluator.eval(h)
        } else {
            self.non_flush_helper.evaluate(h)
        }
    }
    pub fn eval(&self, h: PokerHand) -> u64 {
        self.eval_fast(PokerHandFast::new(h))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_non_flush() {
        let evaluator = PokerHandEvaluator::new();
        //let c1 = [(Suit::Clubs, 1)]
        //assert!(evaluator.eval(PokerHand::new(c1)) < evaluator.eval(PokerHand::new(c1)));
    }
}
