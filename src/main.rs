pub mod hand_evaluator;
pub mod poker_hand;
pub mod ranks;

use crate::hand_evaluator::PokerHandEvaluator;
use crate::poker_hand::{PokerHand, PokerHandFast, PokerHandNonFlush};

fn main() {
    let evaluator = PokerHandEvaluator::new();
    let h1 = PokerHand::new([(1, 1), (1, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 8)]);
    let h2 = PokerHand::new([(1, 1), (1, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)]);

    println!("Je to {0} {1}\n", evaluator.eval(h1), evaluator.eval(h2));
}
