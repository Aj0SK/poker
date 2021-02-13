pub mod hand_evaluator;
pub mod poker_basics;

use crate::hand_evaluator::PokerHandEvaluator;
use crate::poker_basics::card::{PokerCard, PokerHand, Suit};

// based on https://www.pokerlistings.com/which-hand-wins-calculator

fn main() {
    for i in (0..1_000) {
        let hand: PokerHand = rand::random();
        let is_flush = hand.get_fast().is_flush();
        if is_flush {
            println!("Hand is {:?}\n", hand);
        }
    }
}
