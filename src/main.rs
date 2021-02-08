pub mod hand_evaluator;
pub mod poker_basics;

use crate::hand_evaluator::PokerHandEvaluator;
use crate::poker_basics::card::{PokerHand, Suit};

// based on https://www.pokerlistings.com/which-hand-wins-calculator

fn main() {
    let evaluator = PokerHandEvaluator::new();
    let h1 = PokerHand::new([
        (Suit::Spades, 1),
        (Suit::Spades, 2),
        (Suit::Spades, 5),
        (Suit::Spades, 6),
        (Suit::Diamonds, 5),
        (Suit::Diamonds, 6),
        (Suit::Diamonds, 8),
    ]);
    let h2 = PokerHand::new([
        (Suit::Spades, 1),
        (Suit::Clubs, 2),
        (Suit::Spades, 5),
        (Suit::Clubs, 7),
        (Suit::Diamonds, 5),
        (Suit::Diamonds, 6),
        (Suit::Diamonds, 8),
    ]);

    println!("Je to {0} {1}\n", evaluator.eval(h1), evaluator.eval(h2));
}
