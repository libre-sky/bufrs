mod error;
pub use error::Error;
use std::fmt::Display;

/// U24 is a uint consisting of 3 bytes
#[derive(Debug, Clone, Copy, Default)]
pub struct U24([u8; 3]);

impl Display for U24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: u32 = (*self).into();
        val.fmt(f)
    }
}

impl Into<u32> for U24 {
    fn into(self) -> u32 {
        ((self.0[0] as u32) << 16) + ((self.0[1] as u32) << 8) + (self.0[2] as u32)
    }
}

impl Into<usize> for U24 {
    fn into(self) -> usize {
        let v: u32 = self.into();
        v as usize
    }
}

impl U24 {
    pub fn new(b: &[u8]) -> Result<Self, Error> {
        if b.len() < 3 {
            Err(Error::InvalidByteLength(b.len()))
        } else {
            Ok(U24::from_be_bytes(b[..3].try_into().unwrap()))
        }
    }

    /// creates a new U24 from bytes, ordered with most-significant byte first
    pub fn from_be_bytes(b: [u8; 3]) -> Self {
        Self(b)
    }

    /// creates a new U24 from bytes, ordered with least-significant byte first
    pub fn from_le_bytes(b: [u8; 3]) -> Self {
        let mut b_rev = b;
        b_rev.reverse();
        Self(b_rev)
    }
}

#[cfg(test)]
mod test {
    use crate::dtype::{U24, u24::Error};

    #[test]
    fn new() {
        let v0 = U24::new(&[1, 15, 3]).unwrap();
        assert_eq!(
            <U24 as Into<u32>>::into(v0),
            (1u32 << 16) + (15u32 << 8) + 3u32
        );

        assert_eq!(Error::InvalidByteLength(1), U24::new(&[123]).unwrap_err());
    }
}
