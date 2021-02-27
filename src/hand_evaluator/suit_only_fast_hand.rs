#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct SuitOnlyFastHand(u64);

impl SuitOnlyFastHand {
    pub fn new(val: u64) -> SuitOnlyFastHand {
        if val >= (1 << 13) {
            panic!("Hand too big.");
        }
        SuitOnlyFastHand(val)
    }
    pub fn is_straight(self) -> bool {
        let helper = 0b11111;
        for i in 0..(13 - 4) {
            if (self.0 & (helper << i)).count_ones() == 5 {
                return true;
            }
        }
        return false;
    }
    pub fn count_ones(self) -> u64 {
        self.0.count_ones() as u64
    }
}

impl Into<u64> for SuitOnlyFastHand {
    fn into(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::StdRng;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;

    #[test]
    #[should_panic]
    fn new() {
        let _ = SuitOnlyFastHand::new(0b10_0000_0000_0000);
    }

    #[test]
    fn is_straight() {
        assert_eq!(
            SuitOnlyFastHand::new(0b0_0000_0000_0000).is_straight(),
            false
        );
        assert_eq!(
            SuitOnlyFastHand::new(0b0_1010_1010_1001).is_straight(),
            false
        );
        assert_eq!(
            SuitOnlyFastHand::new(0b0_1011_1000_1000).is_straight(),
            false
        );
        assert_eq!(
            SuitOnlyFastHand::new(0b1_1111_0000_1000).is_straight(),
            true
        );
        assert_eq!(
            SuitOnlyFastHand::new(0b0_1111_1000_1000).is_straight(),
            true
        );
    }

    #[test]
    fn is_straight_automated() {
        let mut rng: StdRng = SeedableRng::seed_from_u64(2104);
        for _ in 0..1_000 {
            let mut value = 0;
            let straight_cards_count = (5..=7)
                .collect::<Vec<usize>>()
                .choose(&mut rng)
                .cloned()
                .unwrap();
            let to_set = (0..13 - straight_cards_count - 1)
                .collect::<Vec<usize>>()
                .choose(&mut rng)
                .cloned()
                .unwrap();
            for i_th_bit in to_set..to_set + straight_cards_count {
                value |= 1 << i_th_bit;
            }
            assert_eq!(SuitOnlyFastHand::new(value).is_straight(), true);
        }
    }
}
