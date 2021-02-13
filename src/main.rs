pub mod hand_evaluator;
pub mod poker_basics;

use crate::hand_evaluator::fast_hand::PokerHandFast;
use crate::hand_evaluator::PokerHandEvaluator;
use crate::poker_basics::card::{PokerCard, PokerHand, Suit};

// based on https://www.pokerlistings.com/which-hand-wins-calculator

fn main() {
    let size = 1_000;
    let mut hands: Vec<PokerHand> = (0..size).map(|_| rand::random()).collect();
    let eval: PokerHandEvaluator = PokerHandEvaluator::new();
    hands.sort_by(|a, b| {
        let a_num = eval.eval(*a);
        let b_num = eval.eval(*b);
        a_num.cmp(&b_num)
    });

    for (i, hand) in hands.iter().enumerate() {
        println!("{} {}\n", i, hand);
    }
}
