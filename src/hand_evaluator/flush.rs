use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialOrd;

use super::ranks::*;
use crate::poker_basics::card::PokerCard;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokerHandNonFlush(pub Vec<u64>);

impl PokerHandNonFlush {
    pub fn new(hand: Vec<u64>) -> Self {
        Self(hand)
    }

    fn find_first_greater(v: &Vec<u64>, left: u64, right: u64, val: u64) -> Option<u64> {
        let mut best_index: Option<u64> = None;
        for i in left..right {
            if v[i as usize] > val {
                best_index = Some(i);
            }
        }
        best_index
    }

    pub fn foak(&self) -> Option<FourOfAKind> {
        let i = PokerHandNonFlush::find_first_greater(&self.0, 0, 13, 3);
        if i.is_none() {
            return None;
        }
        let i = i.unwrap();
        let mut arr_without_foak = self.0.clone();
        arr_without_foak[i as usize] -= 4;
        let j = PokerHandNonFlush::find_first_greater(&arr_without_foak, 0, 13, 0);
        if j.is_none() {
            return None;
        }
        let j = j.unwrap();
        return Some(FourOfAKind {
            hand: self.clone(),
            value: i as u64,
            high_card: j as u64,
        });
    }

    pub fn straight(&self) -> Option<Straight> {
        let mut maybe_index: Option<i64> = None;
        for i in (-1 as i64)..(7 as i64) {
            if self.0[((i + 13) % 13) as usize] >= 1
                && self.0[((i + 1) % 13) as usize] >= 1
                && self.0[((i + 2) % 13) as usize] >= 1
                && self.0[((i + 3) % 13) as usize] >= 1
                && self.0[((i + 4) % 13) as usize] >= 1
            {
                maybe_index = Some(i);
            }
        }

        let index = maybe_index;
        if index.is_none() {
            return None;
        }
        let index = index.unwrap();
        let mut arr_without_straight = self.0.clone();
        arr_without_straight[((index + 13) % 13) as usize] -= 1;
        arr_without_straight[((index + 1) % 13) as usize] -= 1;
        arr_without_straight[((index + 2) % 13) as usize] -= 1;
        arr_without_straight[((index + 3) % 13) as usize] -= 1;
        arr_without_straight[((index + 4) % 13) as usize] -= 1;

        let k = PokerHandNonFlush::find_first_greater(&arr_without_straight, 0, 13, 0);
        if k.is_none() {
            return None;
        }
        let k = k.unwrap();
        return Some(Straight {
            hand: self.clone(),
            value_end: (index + 5) as u64,
            high_card: k as u64,
        });
    }

    pub fn full_house(&self) -> Option<FullHouse> {
        let i = PokerHandNonFlush::find_first_greater(&self.0, 0, 13, 2);
        if i.is_none() {
            return None;
        }
        let i = i.unwrap();
        let mut arr_without_three = self.0.clone();
        arr_without_three[i as usize] -= 3;
        let j = PokerHandNonFlush::find_first_greater(&arr_without_three, 0, 13, 1);
        if j.is_none() {
            return None;
        }
        let j = j.unwrap();
        let mut arr_without_full_house = arr_without_three.clone();
        arr_without_full_house[j as usize] -= 2;
        let k = PokerHandNonFlush::find_first_greater(&arr_without_full_house, 0, 13, 0);
        if k.is_none() {
            return None;
        }
        let k = k.unwrap();
        return Some(FullHouse {
            hand: self.clone(),
            value_three: i as u64,
            value_pair: j as u64,
            high_card: k as u64,
        });
    }

    pub fn toak(&self) -> Option<ThreeOfAKind> {
        let i = PokerHandNonFlush::find_first_greater(&self.0, 0, 13, 2);
        if i.is_none() {
            return None;
        }
        let i = i.unwrap();
        let mut arr_without_three = self.0.clone();
        arr_without_three[i as usize] -= 3;
        let j = PokerHandNonFlush::find_first_greater(&arr_without_three, 0, 13, 0);
        if j.is_none() {
            return None;
        }
        let j = j.unwrap();
        return Some(ThreeOfAKind {
            hand: self.clone(),
            value: i as u64,
            high_card: j as u64,
        });
    }

    pub fn two_pairs(&self) -> Option<TwoPairs> {
        let i = PokerHandNonFlush::find_first_greater(&self.0, 0, 13, 1);
        if i.is_none() {
            return None;
        }
        let i = i.unwrap();
        let mut arr_without_pair = self.0.clone();
        arr_without_pair[i as usize] -= 2;
        let j = PokerHandNonFlush::find_first_greater(&arr_without_pair, 0, 13, 1);
        if j.is_none() {
            return None;
        }
        let j = j.unwrap();
        let mut arr_without_two_pairs = arr_without_pair.clone();
        arr_without_two_pairs[j as usize] -= 2;
        let k = PokerHandNonFlush::find_first_greater(&arr_without_two_pairs, 0, 13, 0);
        if k.is_none() {
            return None;
        }
        let k = k.unwrap();
        return Some(TwoPairs {
            hand: self.clone(),
            value1: i as u64,
            value2: j as u64,
            high_card: k as u64,
        });
    }

    pub fn pair(&self) -> Option<Pair> {
        let i = PokerHandNonFlush::find_first_greater(&self.0, 0, 13, 1);
        if i.is_none() {
            return None;
        }
        let i = i.unwrap();
        let mut arr_without_pair = self.0.clone();
        arr_without_pair[i as usize] -= 2;
        let k = PokerHandNonFlush::find_first_greater(&arr_without_pair, 0, 13, 0);
        if k.is_none() {
            return None;
        }
        let k = k.unwrap();
        return Some(Pair {
            hand: self.clone(),
            value: i as u64,
            high_card: k as u64,
        });
    }

    pub fn high_card(&self) -> Option<HighCard> {
        let i = PokerHandNonFlush::find_first_greater(&self.0, 0, 13, 0);
        if i.is_none() {
            return None;
        }
        let i = i.unwrap();
        return Some(HighCard {
            hand: self.clone(),
            value: i as u64,
        });
    }
}
