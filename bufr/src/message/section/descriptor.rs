pub struct Descriptor([u8; 2]);
pub struct DescriptorImpl {
    pub descriptor_type: Type,
    pub table_class: Class,
    pub element: u8,
}

pub enum Type {
    Element,
    Replication,
    Operator,
    Sequence,
}

pub struct Class(u8);

impl Class {
    pub fn new(v: u8) -> Self {
        Self(v ^ 0b11_1111)
    }
}

impl Into<u8> for Type {
    fn into(self) -> u8 {
        let out = match self {
            Self::Element => 0,
            Self::Replication => 1,
            Self::Operator => 2,
            Self::Sequence => 3,
        };
        out << 6
    }
}

impl From<u8> for Type {
    fn from(value: u8) -> Self {
        let value = (value & 0b1100_0000) >> 6;
        match value {
            0 => Self::Element,
            1 => Self::Replication,
            2 => Self::Operator,
            3 => Self::Sequence,
            _ => panic!("invalid value"),
        }
    }
}

impl Into<u16> for Descriptor {
    fn into(self) -> u16 {
        u16::from_be_bytes(self.0)
    }
}
