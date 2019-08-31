use std::fmt;

use log::*;
use nom::error::*;
use smallvec;

const SNIPPET_LEN: usize = 240;

#[derive(Debug)]
pub enum CIFParseError<'a> {
    NomVerbose(VerboseError<&'a [u8]>),
    Utf8(std::str::Utf8Error),
    MandatoryFieldMissing(&'a [u8]),
}

impl fmt::Display for CIFParseError<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CIFParseError::NomVerbose(ref err) => {
                writeln!(fmt, "NomError: ")?;
                for &(ref s, ref kind) in err.errors.iter() {
                    let len = std::cmp::min(s.len(), SNIPPET_LEN);
                    writeln!(
                        fmt,
                        "Err: {:?}: {:?}{}",
                        kind,
                        String::from_utf8_lossy(&s[..len]),
                        if s.len() < SNIPPET_LEN { "" } else { "…" }
                    )?;
                }
                Ok(())
            }
            &CIFParseError::Utf8(ref err) => writeln!(fmt, "UTF conversion: {}", err),
            &CIFParseError::MandatoryFieldMissing(s) => {
                let len = std::cmp::min(s.len(), SNIPPET_LEN);

                writeln!(
                    fmt,
                    "Mandatory field missing at: {:?}{}",
                    String::from_utf8_lossy(&s[..len]),
                    if s.len() < SNIPPET_LEN { "" } else { "…" }
                )
            }
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
            CIFParseError::MandatoryFieldMissing(_) => {
                unimplemented!("CIFParseError::append: MandatoryFieldMissing")
            }
        }
    }
}

impl<A: smallvec::Array<Item = u8>> std::convert::From<smallstr::FromUtf8Error<A>>
    for CIFParseError<'_>
{
    fn from(e: smallstr::FromUtf8Error<A>) -> Self {
        CIFParseError::Utf8(e.utf8_error())
    }
}
