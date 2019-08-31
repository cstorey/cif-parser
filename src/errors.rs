use std::fmt;

use log::*;
use nom::error::*;

const SNIPPET_LEN: usize = 240;

#[derive(Debug)]
pub enum CIFParseError<'a> {
    NomVerbose(VerboseError<&'a [u8]>),
    Utf8(std::str::Utf8Error),
    MandatoryFieldMissing(&'static str, &'a [u8]),
    InvalidNumber(lexical_core::Error),
    InvalidTime(&'a [u8]),
}

impl<'a> CIFParseError<'a> {
    pub(crate) fn into_unrecoverable<E>(e: E) -> nom::Err<Self>
    where
        Self: From<E>,
    {
        let e: Self = e.into();
        nom::Err::Failure(e)
    }
}

impl fmt::Display for CIFParseError<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CIFParseError::NomVerbose(ref err) => {
                writeln!(fmt, "NomError: ")?;
                for &(ref s, ref kind) in err.errors.iter() {
                    writeln!(fmt, "Err: {:?}: {}", kind, as_snippet(s))?;
                }
                Ok(())
            }
            &CIFParseError::Utf8(ref err) => writeln!(fmt, "UTF conversion: {}", err),
            &CIFParseError::MandatoryFieldMissing(field_name, s) => writeln!(
                fmt,
                "Mandatory field {} missing at: {}",
                field_name,
                as_snippet(s)
            ),
            &CIFParseError::InvalidNumber(e) => writeln!(fmt, "Invalid number: {:?}", e),
            &CIFParseError::InvalidTime(s) => writeln!(fmt, "Invalid time: {}", as_snippet(s)),
        }
    }
}

impl<'a> nom::error::ParseError<&'a [u8]> for CIFParseError<'a> {
    fn from_error_kind(i: &'a [u8], kind: nom::error::ErrorKind) -> Self {
        let vb = VerboseError::from_error_kind(i, kind);
        CIFParseError::NomVerbose(vb)
    }

    fn append(i: &'a [u8], kind: nom::error::ErrorKind, other: Self) -> Self {
        match other {
            CIFParseError::NomVerbose(vb) => {
                let vb = VerboseError::append(i, kind, vb);
                CIFParseError::NomVerbose(vb)
            }
            e @ CIFParseError::Utf8(_) => {
                warn!("Dropping UTF error: {}", e);
                Self::from_error_kind(i, kind)
            }
            CIFParseError::MandatoryFieldMissing(_, _) => {
                unimplemented!("CIFParseError::append: MandatoryFieldMissing")
            }
            CIFParseError::InvalidNumber(e) => {
                unimplemented!("CIFParseError::append: InvalidNumber: {:?}", e)
            }
            CIFParseError::InvalidTime(e) => {
                unimplemented!("CIFParseError::append: InvalidTime: {:?}", e)
            }
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
