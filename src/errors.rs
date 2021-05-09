use std::{borrow::Cow, fmt};

const SNIPPET_LEN: usize = 240;

#[derive(Debug)]
pub enum CIFParseError<'a> {
    Utf8(std::str::Utf8Error),
    MandatoryFieldMissing(&'static str, Cow<'a, [u8]>),
    InvalidNumber(lexical_core::Error),
    InvalidTime(Cow<'a, [u8]>),
    InvalidItem,
}

impl fmt::Display for CIFParseError<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CIFParseError::Utf8(ref err) => writeln!(fmt, "UTF conversion: {}", err),
            CIFParseError::MandatoryFieldMissing(field_name, s) => writeln!(
                fmt,
                "Mandatory field {} missing at: {}",
                field_name,
                as_snippet(s)
            ),
            CIFParseError::InvalidNumber(e) => writeln!(fmt, "Invalid number: {:?}", e),
            CIFParseError::InvalidTime(s) => writeln!(fmt, "Invalid time: {}", as_snippet(s)),
            CIFParseError::InvalidItem => writeln!(fmt, "Invalid item"),
        }
    }
}

impl std::convert::From<std::str::Utf8Error> for CIFParseError<'_> {
    fn from(e: std::str::Utf8Error) -> Self {
        CIFParseError::Utf8(e)
    }
}
impl std::convert::From<lexical_core::Error> for CIFParseError<'_> {
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
