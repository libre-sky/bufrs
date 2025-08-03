mod section_0;
mod section_1;
mod section_5;
pub use section_0::Section0;
pub use section_1::Section1;
pub use section_5::Section5;

pub trait Section
where
    Self: Sized,
    Self::Error: Sized,
{
    type Error;
    fn len(&self) -> usize;
    fn read(buf: &[u8]) -> Result<Self, Self::Error>;

    fn validate_length(&self, buf: &[u8]) -> Result<(), ()> {
        if self.len() > buf.len() {
            Err(())
        } else {
            Ok(())
        }
    }
}
