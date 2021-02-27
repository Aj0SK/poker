use crate::poker_basics::card::*;

#[derive(Debug, Copy, Clone)]
pub struct PokerHandFast(pub u64);

impl PokerHandFast {
    pub fn new(hand: PokerHand) -> PokerHandFast {
        hand.get_fast()
    }

    pub fn new_raw(hand: u64) -> PokerHandFast {
        PokerHandFast(hand)
    }

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
    pub fn count_val(&self, val: u64) -> u64 {
        let mut counter = 0;
        for i in 0..4 {
            if (self.0 & (1 << (13 * i + val))) != 0 {
                counter += 1;
            }
        }
        counter
    }

    pub fn get_non_flush_repr(&self) -> Vec<u64> {
        let mut repr: Vec<u64> = vec![];
        for i in 0..13 {
            repr.push(self.count_val(i));
        }
        repr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_flush_true() {
        let c1 = [
            (Suit::C, 0),
            (Suit::C, 1),
            (Suit::C, 2),
            (Suit::C, 3),
            (Suit::C, 4),
            (Suit::D, 5),
            (Suit::D, 6),
        ];
        let c2 = [
            (Suit::C, 0),
            (Suit::C, 1),
            (Suit::D, 2),
            (Suit::D, 3),
            (Suit::D, 4),
            (Suit::D, 5),
            (Suit::D, 6),
        ];
        assert_eq!(PokerHand::new(c1).get_fast().is_flush(), true);
        assert_eq!(PokerHand::new(c2).get_fast().is_flush(), true);
    }
    #[test]
    fn is_flush_false() {
        let c3 = [
            (Suit::C, 0),
            (Suit::D, 1),
            (Suit::H, 2),
            (Suit::S, 3),
            (Suit::C, 4),
            (Suit::D, 5),
            (Suit::H, 6),
        ];
        assert_eq!(PokerHand::new(c3).get_fast().is_flush(), false);
    }
}
