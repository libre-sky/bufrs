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
}
