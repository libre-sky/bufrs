mod section_0;

pub use section_0::Section0;

pub trait Section: Sized {
    type Error;
    fn len(&self) -> usize;
    fn read(buf: &[u8]) -> Result<Self, Self::Error>;
}
