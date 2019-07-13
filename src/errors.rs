use std::fmt;

use failure_derive::*;
use nom::error::*;

#[derive(Debug, Fail)]
pub enum CIFParseError {
    NomVerbose(VerboseError<String>),
}

impl fmt::Display for CIFParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        const SNIPPET_LEN: usize = 240;

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
        }
    }
}

impl nom::error::ParseError<&[u8]> for CIFParseError {
    fn from_error_kind(i: &[u8], kind: nom::error::ErrorKind) -> Self {
        let s = String::from_utf8_lossy(i);
        let vb = VerboseError::from_error_kind(s.into_owned(), kind);
        CIFParseError::NomVerbose(vb)
    }

    fn append(i: &[u8], kind: nom::error::ErrorKind, other: Self) -> Self {
        match other {
            CIFParseError::NomVerbose(vb) => {
                let s = String::from_utf8_lossy(i);

                let vb = VerboseError::append(s.into_owned(), kind, vb);
                CIFParseError::NomVerbose(vb)
            }
        }
    }
}
