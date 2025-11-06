use crate::{
    dtype::U24,
    message::{
        Message,
        section::{InvalidSection, Section, Section3},
        table::Table,
    },
};

#[derive(Default, Debug)]
pub struct Section4 {
    length: U24,
    data: Vec<Box<dyn Table>>,
}

impl Section4 {
    fn parse_binary_data(b: &[u8]) -> Result<Vec<Box<dyn Table>>, <Self as Section>::Error> {
        Err(InvalidSection::InvalidLen(0))
    }
}

impl Section for Section4 {
    type Error = InvalidSection;
    fn len(&self) -> usize {
        self.length.into()
    }

    fn read(buf: &[u8], _: &Message) -> Result<Self, Self::Error> {
        let length = Self::read_length(buf)?;
        let mut s: Section4 = Default::default();
        s.length = length;
        s.data = Self::parse_binary_data(&buf[4..])?;

        Ok(s)
    }
}
