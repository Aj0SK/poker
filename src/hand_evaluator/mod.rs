pub mod fast_hand;
pub mod flush;
mod flush_evaluator;
mod non_flush_evaluator;
mod ranks;

use flush_evaluator::FlushEvaluator;
use non_flush_evaluator::NonFlushEvaluator;

use crate::hand_evaluator::fast_hand::*;
use crate::poker_basics::card::PokerHand;

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
            1_000_000_000_000 + self.flush_evaluator.eval(h)
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
