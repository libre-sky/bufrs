use crate::{
    dtype::U24,
    message::section::{InvalidSection, Section},
};

#[derive(Debug, Clone)]
pub struct Section2 {
    length: U24,
}

impl Section for Section2 {
    type Error = InvalidSection;
    fn len(&self) -> usize {
        self.length.clone().into()
    }

    fn read(buf: &[u8]) -> Result<Self, Self::Error> {
        if buf.len() < 3 {
            Err(Self::Error::InvalidLen(buf.len()))
        } else {
            Ok(Self {
                length: U24::new(buf).unwrap(),
            })
        }
    }
}
