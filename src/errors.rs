use std::fmt;

use failure::Fail;
use log::*;
use nom::error::*;
use smallvec;

const SNIPPET_LEN: usize = 240;

#[derive(Debug, Fail)]
pub enum CIFParseError {
    NomVerbose(VerboseError<String>),
    Utf8(std::str::Utf8Error),
    MandatoryFieldMissing,
}

impl fmt::Display for CIFParseError {
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
                        &s[..len],
                        if s.len() < SNIPPET_LEN { "" } else { "…" }
                    )?;
                }
                Ok(())
            }
            &CIFParseError::Utf8(ref err) => writeln!(fmt, "UTF conversion: {}", err),
            &CIFParseError::MandatoryFieldMissing => writeln!(fmt, "Mandatory field missing"),
        }
    }
}

impl nom::error::ParseError<&[u8]> for CIFParseError {
    fn from_error_kind(i: &[u8], kind: nom::error::ErrorKind) -> Self {
        let len = std::cmp::min(i.len(), SNIPPET_LEN + 1);
        let s = String::from_utf8_lossy(&i[..len]);

        let vb = VerboseError::from_error_kind(s.into(), kind);
        CIFParseError::NomVerbose(vb)
    }

    fn append(i: &[u8], kind: nom::error::ErrorKind, other: Self) -> Self {
        match other {
            CIFParseError::NomVerbose(vb) => {
                let s = String::from_utf8_lossy(i);

                let vb = VerboseError::append(s.into_owned(), kind, vb);
                CIFParseError::NomVerbose(vb)
            }
            e @ CIFParseError::Utf8(_) => {
                warn!("Dropping UTF error: {}", e);
                Self::from_error_kind(i, kind)
            }
            CIFParseError::MandatoryFieldMissing => {
                unimplemented!("CIFParseError::append: MandatoryFieldMissing")
            }
        }
    }
}

impl<A: smallvec::Array<Item = u8>> std::convert::From<smallstr::FromUtf8Error<A>>
    for CIFParseError
{
    fn from(e: smallstr::FromUtf8Error<A>) -> Self {
        CIFParseError::Utf8(e.utf8_error())
    }
}
