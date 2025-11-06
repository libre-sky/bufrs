use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidSection {
    /// When the first 4 bytes of the message are not "BUFR"
    NotBufr([u8; 4]),
    /// Length is not 8 bytes
    InvalidLen(usize),
    /// Version is not currently supported
    UnsupportedVersion(u8),
    /// Optional Specification value is not `0` or `1`
    InvalidOptionalSpec(u8),
    InvalidContent([u8; 4]),
    ParseError(String),
}
// TODO: make more verbose/descriptive
impl Display for InvalidSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
