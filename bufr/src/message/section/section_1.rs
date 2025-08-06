use chrono::{DateTime, TimeZone, Utc};

use crate::{
    dtype::U24,
    message::section::{InvalidSection, Section},
    utils::bytes::BitIndex,
};

#[derive(Debug, Default, Clone)]
pub struct Section1 {
    pub length: U24,
    pub master_table: u8,
    pub origin_center_id: u16,
    pub sub_center_id: u16,
    pub sequence_number: u8,
    pub contains_optional_section: bool,
    pub data_category: u8,
    pub international_data_sub_category: u8,
    pub local_sub_category: u8,
    pub master_table_version: u8,
    pub local_table_version: u8,
    pub timestamp: Datetime,
}

#[derive(Debug, Clone, Default)]
pub struct Datetime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl Into<DateTime<Utc>> for Datetime {
    fn into(self) -> DateTime<Utc> {
        chrono::Utc
            .with_ymd_and_hms(
                self.year as i32,
                self.month as u32,
                self.day as u32,
                self.hour as u32,
                self.minute as u32,
                self.second as u32,
            )
            .unwrap()
    }
}

impl Datetime {
    pub fn new(b: [u8; 7]) -> Self {
        Self {
            year: u16::from_be_bytes([b[0], b[1]]),
            month: b[2],
            day: b[3],
            hour: b[4],
            minute: b[5],
            second: b[6],
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContainsOptionalSection {
    No,
    Yes,
}

impl Into<u8> for ContainsOptionalSection {
    fn into(self) -> u8 {
        match self {
            Self::No => 0,
            Self::Yes => 1,
        }
    }
}

impl Into<bool> for ContainsOptionalSection {
    fn into(self) -> bool {
        match self {
            Self::No => false,
            Self::Yes => true,
        }
    }
}
impl From<bool> for ContainsOptionalSection {
    fn from(value: bool) -> Self {
        match value {
            false => Self::No,
            true => Self::Yes,
        }
    }
}

impl From<u8> for ContainsOptionalSection {
    fn from(value: u8) -> Self {
        let v = BitIndex::One.mask(value);
        v.into()
    }
}

impl Section1 {
    fn read_len(b: &[u8]) -> Result<U24, <Self as Section>::Error> {
        match U24::new(b) {
            Ok(v) => {
                let vv: usize = v.clone().into();
                if vv > b.len() {
                    Err(<Self as Section>::Error::InvalidLen(b.len()))
                } else {
                    Ok(v)
                }
            }
            Err(_) => Err(<Self as Section>::Error::InvalidLen(b.len())),
        }
    }
}

impl Section for Section1 {
    type Error = InvalidSection;
    fn len(&self) -> usize {
        self.length.clone().into()
    }
    fn read(buf: &[u8]) -> Result<Self, Self::Error> {
        let length = Self::read_len(buf)?;
        let l: usize = length.clone().into();
        if l < 23usize {
            return Err(Self::Error::InvalidLen(length.into()));
        }
        let master_table = buf[3];
        let origin_center_id = u16::from_be_bytes(buf[4..6].try_into().unwrap());
        let sub_center_id = u16::from_be_bytes(buf[6..8].try_into().unwrap());
        let sequence_number = buf[8];
        let contains_optional_section: bool = ContainsOptionalSection::from(buf[9]).into();
        let data_category = buf[10];
        let international_data_sub_category = buf[11];
        let local_sub_category = buf[12];
        let master_table_version = buf[13];
        let local_table_version = buf[14];
        let timestamp = Datetime::new(buf[15..22].try_into().unwrap());
        Ok(Self {
            length,
            master_table,
            origin_center_id,
            sub_center_id,
            sequence_number,
            contains_optional_section,
            data_category,
            international_data_sub_category,
            local_sub_category,
            master_table_version,
            local_table_version,
            timestamp,
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        let x = vec![1, 2, 3, 4];
        println!("{:?}", &x[1..3]);
    }
}
