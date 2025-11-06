use std::fmt::Debug;

use crate::{
    dtype::U24,
    message::{
        Message,
        section::{InvalidSection, Section},
        table::Table,
    },
    utils::bytes::BitIndex,
};

#[derive(Default, Debug, Clone)]
pub struct Section3 {
    length: U24,
    num_data_subsets: u16,
    data_spec: DataSpec,
}

#[derive(Default, Clone)]
pub struct DataSpec(u8);

impl Debug for DataSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DataSpec{{ is_compressed: {}, is_observed: {}}}",
            self.is_compressed(),
            self.is_observed()
        )
    }
}

impl DataSpec {
    pub fn is_compressed(&self) -> bool {
        BitIndex::Two.mask(self.0)
    }

    pub fn is_observed(&self) -> bool {
        !BitIndex::One.mask(self.0)
    }
}

impl Section for Section3 {
    type Error = InvalidSection;
    fn len(&self) -> usize {
        self.length.clone().into()
    }

    fn read(buf: &[u8], _: &Message) -> Result<Self, Self::Error> {
        let length = Self::read_length(buf)?;
        let l: usize = length.clone().into();
        if l < 7usize {
            return Err(Self::Error::InvalidLen(length.into()));
        }
        let num_data_subsets = u16::from_be_bytes([buf[4], buf[5]]);
        let data_spec = DataSpec(buf[6]);
        Ok(Self {
            length,
            num_data_subsets,
            data_spec,
        })
    }
}

impl Section3 {
    fn parse_descriptors(
        buf: &[u8],
        section_length: usize,
    ) -> Result<Vec<Box<dyn Table>>, <Self as Section>::Error> {
        Ok(vec![])
    }
}
