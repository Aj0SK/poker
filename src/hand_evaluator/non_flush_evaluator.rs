use crate::poker_basics::card::PokerHandFast;
use std::cmp::min;
use std::collections::HashMap;

use super::flush::PokerHandNonFlush;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonFlushEvaluator {
    non_flush_helper: HashMap<PokerHandNonFlush, u64>,
}

impl NonFlushEvaluator {
    pub fn new() -> NonFlushEvaluator {
        Self {
            non_flush_helper: NonFlushEvaluator::prepare_non_flush_table(),
        }
    }

    fn prepare_non_flush_table() -> HashMap<PokerHandNonFlush, u64> {
        let mut helper = NonFlushEvaluator::generate_non_flush_hands();

        NonFlushEvaluator::sort_non_flush(&mut helper);

        let mut out = HashMap::new();
        for (index, hand) in helper.iter().enumerate() {
            out.insert(hand.clone(), index as u64);
        }

        out
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

        let mut current_index: usize = 0;

        for h in high_card_vec.iter() {
            hands[current_index] = h.hand.clone();
            current_index += 1;
        }
        for h in pairs_vec.iter() {
            hands[current_index] = h.hand.clone();
            current_index += 1;
        }
        for h in two_pairs_vec.iter() {
            hands[current_index] = h.hand.clone();
            current_index += 1;
        }
        for h in toak_vec.iter() {
            hands[current_index] = h.hand.clone();
            current_index += 1;
        }
        for h in straight_vec.iter() {
            hands[current_index] = h.hand.clone();
            current_index += 1;
        }
        for h in full_house_vec.iter() {
            hands[current_index] = h.hand.clone();
            current_index += 1;
        }
        for h in foak_vec.iter() {
            hands[current_index] = h.hand.clone();
            current_index += 1;
        }
    }

    pub fn evaluate(&self, h: PokerHandFast) -> u64 {
        self.non_flush_helper[&PokerHandNonFlush(h.get_non_flush_repr())]
    }
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
