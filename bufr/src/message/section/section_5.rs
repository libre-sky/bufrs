use crate::message::section::Section;

/// Section5: Section 5 is the footer for a BUFR message.
/// It validates that the end of the message is 7777
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidSection {
    InvalidContent([u8; 4]),
    InvalidLen(usize),
}

impl Section5 {
    fn len() -> usize {
        4
    }
    fn validate_content(buf: &[u8]) -> Result<(), <Self as Section>::Error> {
        if buf == b"7777" {
            Ok(())
        } else {
            Err(<Self as Section>::Error::InvalidContent(
                buf[..4].try_into().unwrap(),
            ))
        }
    }
}

impl Section for Section5 {
    type Error = InvalidSection;
    fn len(&self) -> usize {
        Self::len()
    }

    fn read(buf: &[u8]) -> Result<Self, Self::Error> {
        let s = Section5;
        match s.validate_length(buf) {
            Ok(_) => {
                Self::validate_content(buf)?;
                Ok(s)
            }
            Err(_) => Err(Self::Error::InvalidLen(buf.len())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::message::section::{Section, Section5, section_5::InvalidSection};

    #[test]
    fn valid_content() {
        let buf = &b"7777"[..];
        Section5::read(buf).unwrap();
    }
    #[test]
    fn invalid_content() {
        let buf = &b"6969"[..];
        let err = Section5::read(buf).unwrap_err();
        assert_eq!(
            err,
            InvalidSection::InvalidContent(buf[..4].try_into().unwrap())
        );
    }
    #[test]
    fn invalid_length() {
        let buf = &b"696"[..];
        let err = Section5::read(buf).unwrap_err();
        assert_eq!(err, InvalidSection::InvalidLen(buf.len()));
    }
}
