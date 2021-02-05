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

    fn new() -> Self {
        let mut flush = vec![];
        for hand in 0..(1u64 << 13u64) {
            flush.push(hand);
        }

        flush.sort_by(|a, b| {
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
        for (index, hand) in flush.iter().enumerate() {
            flush_helper[*hand as usize] = index as u64;
        }

        Self { flush_helper }
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
        assert_eq!(PokerHandEvaluator::is_straight(0b1_1111_0000_1000), true);
        assert_eq!(PokerHandEvaluator::is_straight(0b0_1111_1000_1000), true);
        assert_eq!(PokerHandEvaluator::is_straight(0b0_1011_1000_1000), false);
    }
}
