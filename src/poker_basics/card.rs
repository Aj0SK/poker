extern crate rand;
use crate::hand_evaluator::fast_hand::PokerHandFast;
use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand::Rng;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    S = 0, //Spades
    H = 1, //Hearts
    D = 2, //Diamonds
    C = 3, //Clubs
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PokerCard {
    suit: Suit,
    value: u64,
} // suit and value from 0 to 12

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PokerHand {
    cards: [PokerCard; 7],
}

//////////////////////////////////////// impl ////////////////////////////////////////

impl PokerCard {
    fn new(suit: Suit, value: u64) -> Self {
        if value > 12 {
            panic!("Card value higher than 12.");
        }
        Self { suit, value }
    }
}

impl PokerHand {
    pub fn new(arr: [(Suit, u64); 7]) -> Self {
        let mut cards: Vec<PokerCard> = Vec::new();
        for (suit, value) in arr.iter() {
            cards.push(PokerCard::new(*suit, *value));
        }
        PokerHand::from_cards(cards.into_iter())
    }
    pub fn from_cards(input: impl Iterator<Item = PokerCard>) -> Self {
        let cards: Vec<PokerCard> = input.collect();
        if cards.len() != 7 {
            panic!("Number of cards is not equal 7!");
        }
        PokerHand {
            cards: cards.try_into().unwrap(),
        }
    }

    pub fn get_fast(&self) -> PokerHandFast {
        let mut repr: u64 = 0;
        for PokerCard { suit, value } in self.cards.iter() {
            repr |= 1 << (13 * (*suit as u64) + value);
        }
        PokerHandFast::new_raw(repr)
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

//////////////////////////////////////// Display ////////////////////////////////////////

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Suit::S => write!(f, "♠"),
            Suit::H => write!(f, "♥"),
            Suit::D => write!(f, "♦"),
            Suit::C => write!(f, "♣"),
        }
    }
}

impl fmt::Display for PokerCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.value)
    }
}

impl fmt::Display for PokerHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}, {}, {}, {}]",
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4],
            self.cards[5],
            self.cards[6]
        )
    }
}

//////////////////////////////////////// Distribution ////////////////////////////////////////

impl Distribution<PokerCard> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PokerCard {
        PokerCard {
            suit: rng.gen(),
            value: rng.gen_range(0..=12),
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

////////////////////////////////////////// Tests ///////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::StdRng;
    use rand::SeedableRng;

    #[test]
    fn get_fast() {
        let mut all_cards = Vec::new();

        for i in 0..4 {
            for j in 0..13 {
                all_cards.push(PokerCard::new(Suit::try_from(i).unwrap(), j));
            }
        }

        let mut rng: StdRng = SeedableRng::seed_from_u64(2104);
        for _ in 0..100 {
            let random_cards: Vec<PokerCard> =
                all_cards.choose_multiple(&mut rng, 7).cloned().collect();
            let mut result = 0;
            for card in random_cards.iter() {
                result |= 1 << all_cards.iter().position(|&r| r == *card).unwrap();
            }
            assert_eq!(
                PokerHand::from_cards(random_cards.into_iter()).get_fast().0,
                result
            );
        }
    }
}
