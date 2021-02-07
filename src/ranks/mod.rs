use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialOrd;

use crate::poker_hand::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FourOfAKind {
    pub hand: PokerHandNonFlush,
    pub value: u64,
    pub high_card: u64,
}

impl Ord for FourOfAKind {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value != other.value {
            self.value.cmp(&other.value)
        } else {
            self.high_card.cmp(&other.high_card)
        }
    }
}

impl PartialOrd for FourOfAKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Straight {
    pub hand: PokerHandNonFlush,
    pub value_end: u64,
    pub high_card: u64,
}

impl Ord for Straight {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value_end != other.value_end {
            self.value_end.cmp(&other.value_end)
        } else {
            self.high_card.cmp(&other.high_card)
        }
    }
}

impl PartialOrd for Straight {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FullHouse {
    pub hand: PokerHandNonFlush,
    pub value_three: u64,
    pub value_pair: u64,
    pub high_card: u64,
}

impl Ord for FullHouse {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value_three != other.value_three {
            self.value_three.cmp(&other.value_three)
        } else if self.value_pair != other.value_pair {
            self.value_pair.cmp(&other.value_pair)
        } else {
            self.high_card.cmp(&other.high_card)
        }
    }
}

impl PartialOrd for FullHouse {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThreeOfAKind {
    pub hand: PokerHandNonFlush,
    pub value: u64,
    pub high_card: u64,
}

impl Ord for ThreeOfAKind {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value != other.value {
            self.value.cmp(&other.value)
        } else {
            self.high_card.cmp(&other.high_card)
        }
    }
}

impl PartialOrd for ThreeOfAKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TwoPairs {
    pub hand: PokerHandNonFlush,
    pub value1: u64,
    pub value2: u64,
    pub high_card: u64,
}

impl Ord for TwoPairs {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value1 != other.value1 {
            self.value1.cmp(&other.value1)
        } else if self.value2 != other.value2 {
            self.value2.cmp(&other.value2)
        } else {
            self.high_card.cmp(&other.high_card)
        }
    }
}

impl PartialOrd for TwoPairs {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pair {
    pub hand: PokerHandNonFlush,
    pub value: u64,
    pub high_card: u64,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value != other.value {
            self.value.cmp(&other.value)
        } else {
            self.high_card.cmp(&other.high_card)
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HighCard {
    pub hand: PokerHandNonFlush,
    pub value: u64,
}

impl Ord for HighCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for HighCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
