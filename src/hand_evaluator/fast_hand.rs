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

////////////////////////////////////////// Tests ///////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::StdRng;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;

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
    fn is_flush_true_automated() {
        let mut rng: StdRng = SeedableRng::seed_from_u64(2104);
        for _ in 0..100 {
            let flush_suit = rand::random::<Suit>();
            let flush_cards_count = (5..=7)
                .collect::<Vec<usize>>()
                .choose(&mut rng)
                .cloned()
                .unwrap();
            let flush_values = (0..13)
                .collect::<Vec<u64>>()
                .choose_multiple(&mut rng, flush_cards_count)
                .cloned()
                .collect::<Vec<u64>>();

            let mut cards: Vec<PokerCard> = Vec::new();
            for value in flush_values.iter() {
                cards.push(PokerCard::new(flush_suit, *value));
            }

            while cards.len() != 7 {
                let card = rand::random::<PokerCard>();
                if card.get_suit() != flush_suit {
                    cards.push(card);
                }
            }
            let hand = PokerHand::from_cards(cards.into_iter());
            assert_eq!(hand.get_fast().is_flush(), true);
            assert_eq!(
                hand.get_fast().flush_val().count_ones() as u64,
                flush_cards_count as u64
            );
        }
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

    #[test]
    fn is_flush_false_automated() {
        let mut tested = 0;
        while tested != 100 {
            let hand = rand::random::<PokerHand>();
            let mut suit_counts = [0, 0, 0, 0];
            for card in hand.cards.iter() {
                suit_counts[card.get_suit() as usize] += 1;
            }
            if suit_counts[0] >= 5
                || suit_counts[1] >= 5
                || suit_counts[2] >= 5
                || suit_counts[3] >= 5
            {
                continue;
            }
            assert_eq!(hand.get_fast().is_flush(), false);
            tested += 1;
        }
    }
}
