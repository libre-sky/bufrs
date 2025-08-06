mod descriptor;
mod error;
mod section_0;
mod section_1;
mod section_2;
mod section_3;
mod section_5;
pub use error::InvalidSection;
pub use section_0::Section0;
pub use section_1::Section1;
pub use section_5::Section5;

use crate::dtype::U24;

pub trait Section
where
    Self: Sized,
    Self::Error: Sized,
{
    type Error;
    fn len(&self) -> usize;
    fn read(buf: &[u8]) -> Result<Self, Self::Error>;

    fn read_length(buf: &[u8]) -> Result<U24, InvalidSection> {
        if buf.len() < 3 {
            Err(InvalidSection::InvalidLen(buf.len()))
        } else {
            Ok(U24::new(buf).unwrap_or_default())
        }
    }

    fn validate_length(&self, buf: &[u8]) -> Result<(), ()> {
        if self.len() > buf.len() {
            Err(())
        } else {
            Ok(())
        }
    }
}
