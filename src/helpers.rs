#![cfg(test)]
use nom::{bytes::streaming::*, error::*, IResult};

pub fn string<'a, E: ParseError<&'a [u8]>>(
    nchars: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Option<&'a str>, E> {
    move |i: &'a [u8]| -> IResult<&'a [u8], Option<&'a str>, E> {
        let (rest, val) = take(nchars)(i)?;
        let val = match std::str::from_utf8(val) {
            Ok(val) => val.trim_end(),
            Err(e) => unimplemented!("str::from_utf8: {:?}", e),
        };
        Ok((rest, Some(val).filter(|val| !val.is_empty())))
    }
}

pub fn mandatory<'a, T, E: ParseError<&'a [u8]>>(
    inner: impl Fn(&'a [u8]) -> IResult<&'a [u8], Option<T>, E>,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], T, E> {
    move |i: &'a [u8]| -> IResult<&'a [u8], T, E> {
        match inner(i)? {
            (rest, Some(val)) => Ok((rest, val)),

            (rest, None) => Err(nom::Err::Error(E::from_error_kind(rest, ErrorKind::IsA))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::error::VerboseError;

    #[test]
    fn string_parser_should_read_value() {
        // string of length 3;
        let p = string::<VerboseError<_>>(3);
        let (rest, result) = p(b"ABC").expect("should parse");
        assert_eq!((rest, result), (b"" as &[u8], Some("ABC")));
    }

    #[test]
    fn string_parser_should_read_part() {
        // string of length 3;
        let p = string::<VerboseError<_>>(3);
        let (rest, result) = p(b"AB ").expect("should parse");
        assert_eq!((rest, result), (b"" as &[u8], Some("AB")));
    }

    #[test]
    fn string_parser_should_empty() {
        // string of length 3;
        let p = string::<VerboseError<_>>(3);
        let (rest, result) = p(b"   ").expect("should parse");
        assert_eq!((rest, result), (b"" as &[u8], None));
    }

    #[test]
    fn string_parser_return_remainder() {
        let p = string::<VerboseError<_>>(3);
        let (rest, result) = p(b"A  DEF").expect("should parse");
        assert_eq!((rest, result), (b"DEF" as &[u8], Some("A")));
    }

    #[test]
    fn mandatory_should_return_ok_on_success() {
        fn inner<'a>(i: &'a [u8]) -> IResult<&'a [u8], Option<()>, VerboseError<&[u8]>> {
            Ok((i, Some(())))
        };
        let p = mandatory(inner);
        let (rest, result) = p(b"Hi").expect("parse");
        assert_eq!((rest, result), (b"Hi" as &[u8], ()));
    }

}
