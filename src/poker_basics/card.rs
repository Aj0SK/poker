#[derive(Debug, Copy, Clone)]
pub struct PokerCard {
    suit: Suit,
    value: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct PokerHand {
    cards: [PokerCard; 7],
}

#[derive(Debug, Copy, Clone)]
pub struct PokerHandFast(pub u64);

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl PokerCard {
    fn new(suit: Suit, value: u64) -> Self {
        Self { suit, value }
    }
}

impl PokerHand {
    pub fn new(arr: [(Suit, u64); 7]) -> Self {
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
    pub fn get_fast(&self) -> PokerHandFast {
        let mut repr: u64 = 0;
        for i in self.cards.iter() {
            match i {
                PokerCard { suit, value } => {
                    repr |= 1 << (13 * (*suit as u64) + value);
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
    fn is_flush() {
        let c1 = [
            (Suit::Clubs, 1),
            (Suit::Clubs, 2),
            (Suit::Clubs, 3),
            (Suit::Clubs, 4),
            (Suit::Clubs, 5),
            (Suit::Diamonds, 6),
            (Suit::Diamonds, 7),
        ];
        let c2 = [
            (Suit::Clubs, 1),
            (Suit::Clubs, 2),
            (Suit::Diamonds, 3),
            (Suit::Diamonds, 4),
            (Suit::Diamonds, 5),
            (Suit::Diamonds, 6),
            (Suit::Diamonds, 7),
        ];
        let c3 = [
            (Suit::Clubs, 1),
            (Suit::Diamonds, 2),
            (Suit::Hearts, 3),
            (Suit::Spades, 4),
            (Suit::Clubs, 5),
            (Suit::Diamonds, 6),
            (Suit::Hearts, 7),
        ];
        assert_eq!(PokerHand::new(c1).get_fast().is_flush(), true);
        assert_eq!(PokerHand::new(c2).get_fast().is_flush(), true);
        assert_eq!(PokerHand::new(c3).get_fast().is_flush(), false);
    }
}
