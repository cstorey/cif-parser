use std::marker::PhantomData;

use nom::{bytes::streaming::*, IResult};

use crate::errors::CIFParseError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Association<'a> {
    _phantom: PhantomData<&'a [u8]>,
}
pub(super) fn parse_association<'a>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Association, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], Association, CIFParseError> {
        let (i, _) = tag("AA")(i)?;
        let (i, _spare) = take(78usize)(i)?;

        Ok((
            i,
            Association {
                _phantom: PhantomData,
            },
        ))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn should_parse_association() {
        let p = parse_association();
        let hdr =
            b"AANY80987Y808801601041602121111100JJSPRST     TP                               P";
        assert_eq!(80, hdr.len());
        let (rest, insert) = p(hdr).expect("parse_header");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            insert,
            Association {
                _phantom: PhantomData,
            }
        )
    }

}
