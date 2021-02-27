pub mod hand_evaluator;
pub mod poker_basics;

use crate::hand_evaluator::PokerHandEvaluator;
use crate::poker_basics::card::PokerHand;

// based on https://www.pokerlistings.com/which-hand-wins-calculator

use std::io;

fn main() {
    let size = 1_000_000;
    let mut hands: Vec<PokerHand> = (0..size).map(|_| rand::random()).collect();
    let eval: PokerHandEvaluator = PokerHandEvaluator::new();
    hands.sort_by(|a, b| {
        let a_num = eval.eval(*a);
        let b_num = eval.eval(*b);
        a_num.cmp(&b_num)
    });

    /*for (i, hand) in hands.iter().enumerate() {
        println!("{} {}\n", i, hand);
    }*/

    for i in 0..100 {
        let mut n = String::new();
        let hand1: PokerHand = rand::random();
        let hand2: PokerHand = rand::random();
        println!("{}\n{}", hand1, hand2);
        io::stdin()
            .read_line(&mut n)
            .expect("failed to read input.");
        let user_out: u64 = n.trim().parse().expect("invalid input");

        let value1 = eval.eval(hand1);
        let value2 = eval.eval(hand2);

        let evaluator = if value1 < value2 {
            1
        } else if value1 > value2 {
            2
        } else {
            3
        };

        if user_out == evaluator {
            println!("Juchuu!")
        } else {
            println!("Niee!");
        }
    }
}
