use bytes::Bytes;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CIFParseError {
    #[error("UTF conversion: {}", 0)]
    Utf8(std::str::Utf8Error),
    #[error("Invalid number: {}", 0)]
    InvalidNumber(lexical_core::Error),
    #[error("Invalid time: {:?}", 0)]
    InvalidTime(Bytes),
    #[error("Invalid item")]
    InvalidItem,
}

impl std::convert::From<std::str::Utf8Error> for CIFParseError {
    fn from(e: std::str::Utf8Error) -> Self {
        CIFParseError::Utf8(e)
    }
}
impl std::convert::From<lexical_core::Error> for CIFParseError {
    fn from(e: lexical_core::Error) -> Self {
        CIFParseError::InvalidNumber(e)
    }
}
