/// U24 is a uint consisting of 3 bytes
#[derive(Debug, Clone, Default)]
pub struct U24([u8; 3]);

#[derive(Debug, Clone)]
pub enum Error {
    InvalidByteLength(usize),
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
