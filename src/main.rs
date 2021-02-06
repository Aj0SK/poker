use std::cmp::min;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
struct PokerCard {
    color: u64,
    value: u64,
}

impl PokerCard {
    fn new(color: u64, value: u64) -> Self {
        Self { color, value }
    }
}

#[derive(Debug, Copy, Clone)]
struct PokerHand {
    cards: [PokerCard; 7],
}

#[derive(Debug, Copy, Clone)]
struct PokerHandFast(u64);

impl PokerHand {
    fn new(arr: [(u64, u64); 7]) -> Self {
        let cards: [PokerCard; 7] = [
            PokerCard::new(arr[0].0, arr[0].1),
            PokerCard::new(arr[1].0, arr[1].1),
            PokerCard::new(arr[2].0, arr[2].1),
            PokerCard::new(arr[3].0, arr[3].1),
            PokerCard::new(arr[4].0, arr[4].1),
            PokerCard::new(arr[5].0, arr[5].1),
            PokerCard::new(arr[6].0, arr[6].1),
        ];
        PokerHand { cards }
    }
    fn get_representation(&self) -> PokerHandFast {
        let mut repr: u64 = 0;
        for i in self.cards.iter() {
            match i {
                PokerCard { color, value } => {
                    repr |= 1 << (13 * color + value);
                }
            }
        }
        PokerHandFast(repr)
    }
}

impl PokerHandFast {
    pub fn is_flush(&self) -> bool {
        self.flush_val() != 0
    }
    pub fn flush_val(&self) -> u64 {
        for i in 0..4 {
            let val: u64 = (self.0 >> (13 * i)) & (0b1_1111_1111_1111);
            if val.count_ones() >= 5 {
                return val;
            }
        }
        0
    }
}

struct PokerHandEvaluator {
    flush_helper: Vec<u64>,
    non_flush_helper: Vec<Vec<u64>>,
}

impl PokerHandEvaluator {
    fn is_straight(hand: u64) -> bool {
        let helper = 0b11111;
        for i in 0..9 {
            if (hand & (helper << i)).count_ones() == 5 {
                return true;
            }
        }
        return false;
    }

    fn fun(mut generating: Vec<u64>, sum: u64, generated: &mut Vec<Vec<u64>>) {
        if sum == 0 {
            while generating.len() != 13 {
                generating.push(0);
            }
        }
        if generating.len() == 13 {
            if sum == 0 {
                generated.push(generating);
            }
            return;
        }

        for i in 0..min(sum + 1, 5) {
            generating.push(i);
            PokerHandEvaluator::fun(generating.clone(), sum - i, generated);
            generating.pop();
        }
    }

    fn generate_non_flush_hands() -> Vec<Vec<u64>> {
        let mut generated = vec![];
        PokerHandEvaluator::fun(vec![], 7, &mut generated);
        if generated.len() != 49_205 {
            println!("Tabulka nesedi.");
        }
        generated
    }

    fn prepare_non_flush_table() -> Vec<Vec<u64>> {
        PokerHandEvaluator::generate_non_flush_hands()
    }

    fn prepare_flush_table() -> Vec<u64> {
        let mut all_hands = vec![];
        for hand in 0..(1u64 << 13u64) {
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

    fn new() -> Self {
        Self {
            flush_helper: PokerHandEvaluator::prepare_flush_table(),
            non_flush_helper: PokerHandEvaluator::prepare_non_flush_table(),
        }
    }

    fn eval_flush(&self, h: PokerHandFast) -> u64 {
        self.flush_helper[h.flush_val() as usize]
    }
    fn eval(&self, h: PokerHandFast) -> u64 {
        if h.is_flush() {
            println!("Flush!");
            self.eval_flush(h)
        } else {
            println!("Not flush!");
            0
        }
    }
}

fn main() {
    let c = [(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)];
    let h = PokerHand::new(c);
    println!("{:#064b}", h.get_representation().0);

    let eval = PokerHandEvaluator::new();
    eval.eval(h.get_representation());

    let mut perms = vec![0; 13];
    let mut counter = 0u64;
    let mut total_counter = 0;
    /*loop {
        total_counter += 1;
        perms[0] += 1;
        for i in 0..12 {
            if perms[i] == 5 {
                perms[i] = 0;
                perms[i + 1] += 1;
            }
        }
        if perms[12] == 5 {
            break;
        }
        let sum: u64 = perms.iter().sum();
        if sum == 7 {
            counter += 1;
        }
        println!("Counter is {} out of {}", counter, total_counter);
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_flush() {
        let c1 = [(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)];
        let c2 = [(1, 1), (1, 2), (1, 3), (0, 4), (0, 5), (1, 6), (1, 7)];
        assert_eq!(PokerHand::new(c1).get_representation().is_flush(), true);
        assert_eq!(PokerHand::new(c2).get_representation().is_flush(), true);
    }
    #[test]
    fn is_straight() {
        assert_eq!(PokerHandEvaluator::is_straight(0b0_0000_0000_0000), false);
        assert_eq!(PokerHandEvaluator::is_straight(0b0_1010_1010_1001), false);
        assert_eq!(PokerHandEvaluator::is_straight(0b0_1011_1000_1000), false);
        assert_eq!(PokerHandEvaluator::is_straight(0b1_1111_0000_1000), true);
        assert_eq!(PokerHandEvaluator::is_straight(0b0_1111_1000_1000), true);
    }
}
