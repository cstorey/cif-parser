use std::fmt;

use bytes::Bytes;

const SNIPPET_LEN: usize = 240;

#[derive(Debug)]
pub enum CIFParseError {
    Utf8(std::str::Utf8Error),
    InvalidNumber(lexical_core::Error),
    InvalidTime(Bytes),
    InvalidItem,
}

impl fmt::Display for CIFParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CIFParseError::Utf8(ref err) => writeln!(fmt, "UTF conversion: {}", err),
            CIFParseError::InvalidNumber(e) => writeln!(fmt, "Invalid number: {:?}", e),
            CIFParseError::InvalidTime(s) => writeln!(fmt, "Invalid time: {}", as_snippet(s)),
            CIFParseError::InvalidItem => writeln!(fmt, "Invalid item"),
        }
    }
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

fn as_snippet(s: &[u8]) -> String {
    let len = std::cmp::min(s.len(), SNIPPET_LEN);
    format!(
        "{}{}",
        String::from_utf8_lossy(&s[..len]),
        if s.len() < SNIPPET_LEN { "" } else { "…" }
    )
}
