/// Helper for working with bufr bits.
/// BUFR specification indexes bits from most significant to least significant,
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum BitIndex {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Into<u8> for BitIndex {
    fn into(self) -> u8 {
        match self {
            BitIndex::One => 0b1000_0000,
            BitIndex::Two => 0b0100_0000,
            BitIndex::Three => 0b0010_0000,
            BitIndex::Four => 0b0001_0000,
            BitIndex::Five => 0b0000_1000,
            BitIndex::Six => 0b0000_0100,
            BitIndex::Seven => 0b0000_0010,
            BitIndex::Eight => 0b0000_0001,
        }
    }
}
impl Into<u8> for &BitIndex {
    fn into(self) -> u8 {
        (*self).into()
    }
}

impl BitIndex {
    pub(crate) fn mask(&self, byte: u8) -> bool {
        let v: u8 = self.into();
        v & byte != 0
    }
}

#[cfg(test)]
mod test {
    use crate::utils::bytes::BitIndex;

    #[test]
    fn bits_and_bobs() {
        let num: u8 = 0b1010_1111;

        assert!(BitIndex::One.mask(num));
        assert!(!BitIndex::Two.mask(num));
    }
}
