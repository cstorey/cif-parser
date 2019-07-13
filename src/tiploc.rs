use smallstr::SmallString;
use nom::{bytes::streaming::*, character::is_space, error::*, IResult};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tiploc(SmallString<[u8; 7]>);

impl Tiploc {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
       let (i, name) = take(7usize)(i)?;
       let ss = SmallString::from_buf(name)?;
       Ok((i, Tiploc(ss)))
    }
}