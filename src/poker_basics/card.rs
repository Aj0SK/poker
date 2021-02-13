extern crate rand;
use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand::Rng;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    S = 0, //Spades
    H = 1, //Hearts
    D = 2, //Diamonds
    C = 3, //Clubs
}

#[derive(Debug, Copy, Clone)]
pub struct PokerCard {
    suit: Suit,
    value: u64,
} // suit and value from 0 to 13

#[derive(Debug, Copy, Clone)]
pub struct PokerHand {
    cards: [PokerCard; 7],
}

#[derive(Debug, Copy, Clone)]
pub struct PokerHandFast(pub u64);

impl Distribution<PokerCard> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PokerCard {
        PokerCard {
            suit: rng.gen(),
            value: rng.gen_range(0..=13),
        }
    }
}

impl Distribution<PokerHand> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PokerHand {
        let all_cards: Vec<PokerCard> = (0..52)
            .map(|x| PokerCard {
                suit: Suit::try_from(x / 13).unwrap(),
                value: x % 13,
            })
            .collect();
        let random_cards: Vec<PokerCard> = all_cards.choose_multiple(rng, 7).cloned().collect();
        let cards: [PokerCard; 7] = random_cards.try_into().unwrap();
        PokerHand { cards }
    }
}

impl Distribution<Suit> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        match rng.gen_range(0..=4) {
            0 => Suit::S,
            1 => Suit::H,
            2 => Suit::D,
            _ => Suit::C,
        }
    }
}

impl TryFrom<u64> for Suit {
    type Error = ();
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        match v {
            x if x == Suit::S as u64 => Ok(Suit::S),
            x if x == Suit::H as u64 => Ok(Suit::H),
            x if x == Suit::D as u64 => Ok(Suit::D),
            x if x == Suit::C as u64 => Ok(Suit::C),
            _ => Err(()),
        }
    }
}

impl PokerCard {
    fn new(suit: Suit, value: u64) -> Self {
        Self { suit, value }
    }
}

impl PokerHand {
    pub fn new(arr: [(Suit, u64); 7]) -> Self {
        for (_, value) in arr.iter() {
            if value >= &13 {
                panic!("Card initialized with too big number!");
            }
        }
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
    pub fn new_from_cards(cards: [PokerCard; 7]) -> Self {
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
