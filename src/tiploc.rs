use std::fmt;

use nom::{bytes::streaming::*, error::ParseError, IResult};
use smallvec::SmallVec;

#[derive(Clone, Eq, PartialEq)]
pub struct Tiploc(SmallVec<[u8; 7]>);

impl Tiploc {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        let (i, name) = take(7usize)(i)?;
        let mut buf = [0u8; 7];
        buf.copy_from_slice(name);
        Ok((i, Tiploc(SmallVec::from_buf(buf))))
    }
}

impl std::convert::From<&str> for Tiploc {
    fn from(s: &str) -> Self {
        Tiploc(SmallVec::from_slice(s.as_bytes()))
    }
}

impl fmt::Debug for Tiploc {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Tiploc")
            .field(&String::from_utf8_lossy(&self.0).trim_end())
            .finish()
    }
}
