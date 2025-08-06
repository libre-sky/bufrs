use crate::{
    dtype::U24,
    message::section::{InvalidSection, Section},
    utils::bytes::BitIndex,
};

#[derive(Default, Debug, Clone)]
pub struct Section3 {
    length: U24,
    num_data_subsets: u16,
    data_spec: DataSpec,
}

#[derive(Default, Debug, Clone)]
pub struct DataSpec(u8);

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

    fn read(buf: &[u8]) -> Result<Self, Self::Error> {
        let length = Self::read_length(buf)?;
        let l: usize = length.clone().into();
        if l < 7usize {
            return Err(Self::Error::InvalidLen(length.into()));
        }
        let num_data_subsets = u16::from_be_bytes(buf[4..6].try_into().unwrap());
        let data_spec = DataSpec(buf[6]);
        Ok(Self {
            length,
            num_data_subsets,
            data_spec,
        })
    }
}
