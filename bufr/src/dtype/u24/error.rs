use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    InvalidByteLength(usize),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
