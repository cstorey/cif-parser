use nom::{bytes::streaming::*, character::is_space, error::*, IResult};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Trailer;

pub(super) fn parse_trailer<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Trailer, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], Trailer, E> {
        let (i, _) = tag("ZZ")(i)?;
        let (i, _spare) = take_while_m_n(78, 78, is_space)(i)?;

        Ok((i, Trailer))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_trailer() {
        let p = parse_trailer::<VerboseError<_>>();
        let i = b"ZZ                                                                              ";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        let rest = String::from_utf8_lossy(rest);
        assert_eq!((val, &*rest), (Trailer, "",))
    }
}
