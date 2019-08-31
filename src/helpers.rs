use nom::{bytes::streaming::*, IResult};

use crate::errors::CIFParseError;

pub fn string<'a>(
    nchars: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Option<&'a str>, CIFParseError> {
    move |i: &'a [u8]| -> IResult<&'a [u8], Option<&'a str>, CIFParseError> {
        let (rest, val) = take(nchars)(i)?;
        let val = match std::str::from_utf8(val) {
            Ok(val) => val.trim_end(),
            Err(e) => unimplemented!("str::from_utf8: {:?}", e),
        };
        Ok((rest, Some(val).filter(|val| !val.is_empty())))
    }
}

pub fn mandatory<'a, T>(
    inner: impl Fn(&'a [u8]) -> IResult<&'a [u8], Option<T>, CIFParseError>,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], T, CIFParseError> {
    move |i: &'a [u8]| -> IResult<&'a [u8], T, CIFParseError> {
        match inner(i)? {
            (rest, Some(val)) => Ok((rest, val)),

            (_rest, None) => Err(nom::Err::Error(CIFParseError::MandatoryFieldMissing(i))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_parser_should_read_value() {
        // string of length 3;
        let p = string(3);
        let (rest, result) = p(b"ABC").expect("should parse");
        assert_eq!((rest, result), (b"" as &[u8], Some("ABC")));
    }

    #[test]
    fn string_parser_should_read_part() {
        // string of length 3;
        let p = string(3);
        let (rest, result) = p(b"AB ").expect("should parse");
        assert_eq!((rest, result), (b"" as &[u8], Some("AB")));
    }

    #[test]
    fn string_parser_should_empty() {
        // string of length 3;
        let p = string(3);
        let (rest, result) = p(b"   ").expect("should parse");
        assert_eq!((rest, result), (b"" as &[u8], None));
    }

    #[test]
    fn string_parser_return_remainder() {
        let p = string(3);
        let (rest, result) = p(b"A  DEF").expect("should parse");
        assert_eq!((rest, result), (b"DEF" as &[u8], Some("A")));
    }

    #[test]
    fn mandatory_should_return_ok_on_success() {
        fn inner<'a>(i: &'a [u8]) -> IResult<&'a [u8], Option<()>, CIFParseError> {
            Ok((i, Some(())))
        };
        let p = mandatory(inner);
        let (rest, result) = p(b"Hi").expect("parse");
        assert_eq!((rest, result), (b"Hi" as &[u8], ()));
    }

}
