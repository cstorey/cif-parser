use chrono::offset::TimeZone;
use chrono::{Date, NaiveTime};
use chrono_tz::{Europe::London, Tz};
use lexical_core;
use nom::{bytes::streaming::*, character::is_digit, IResult};

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

pub fn date<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Date<Tz>, CIFParseError> {
    move |i: &'a [u8]| -> IResult<&'a [u8], Date<Tz>, CIFParseError> {
        let (i, dd) = take_while_m_n(2usize, 2, is_digit)(i)?;
        let (i, mm) = take_while_m_n(2usize, 2, is_digit)(i)?;
        let (i, yy) = take_while_m_n(2usize, 2, is_digit)(i)?;
        let dt = London.ymd(
            lexical_core::atoi32(yy).map_err(into_nom_wrapped)? + 2000,
            lexical_core::atou32(mm).map_err(into_nom_wrapped)?,
            lexical_core::atou32(dd).map_err(into_nom_wrapped)?,
        );
        Ok((i, dt))
    }
}

pub fn time<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], NaiveTime, CIFParseError> {
    move |i: &'a [u8]| -> IResult<&'a [u8], NaiveTime, CIFParseError> {
        let start = i;
        let (i, hh) = take_while_m_n(2usize, 2, is_digit)(i)?;
        let (i, mm) = take_while_m_n(2usize, 2, is_digit)(i)?;
        eprintln!(
            "Parsing time: {}:{}",
            lexical_core::atou32(hh).map_err(into_nom_wrapped)?,
            lexical_core::atou32(mm).map_err(into_nom_wrapped)?,
        );
        let dt = NaiveTime::from_hms_opt(
            lexical_core::atou32(hh).map_err(into_nom_wrapped)?,
            lexical_core::atou32(mm).map_err(into_nom_wrapped)?,
            0,
        )
        .ok_or_else(|| CIFParseError::InvalidTime(start))
        .map_err(into_nom_wrapped)?;
        Ok((i, dt))
    }
}

fn into_nom_wrapped<'a, E>(e: E) -> nom::Err<CIFParseError<'a>>
where
    CIFParseError<'a>: From<E>,
{
    let e: CIFParseError = e.into();
    nom::Err::Error(e)
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

    #[test]
    fn date_should_parse_ddmmyy() {
        let s = b"060315!!";
        let (rest, result) = date()(s).expect("parse");
        assert_eq!((rest, result), (b"!!" as &[u8], London.ymd(2015, 3, 6)));
    }

    #[test]
    fn time_should_parse_hhmm() {
        let s = b"2151!!";
        let (rest, result) = time()(s).expect("parse");
        assert_eq!(
            (rest, result),
            (b"!!" as &[u8], NaiveTime::from_hms(21, 51, 0))
        );
    }
}
