use bitflags::bitflags;
use chrono::offset::TimeZone;
use chrono::{Date, Duration, NaiveTime};
use chrono_tz::{Europe::London, Tz};
use nom::{
    branch::alt, bytes::streaming::*, character::is_digit, character::streaming::char,
    combinator::map, IResult,
};

use crate::errors::CIFParseError;

bitflags! {
    pub struct Days: u8 {
        const MON = 0b00000001;
        const TUE = 0b0000010;
        const WED = 0b00000100;
        const THU = 0b00001000;
        const FRI = 0b00010000;
        const SAT = 0b00100000;
        const SUN = 0b01000000;
    }
}

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
    field_name: &'static str,
    inner: impl Fn(&'a [u8]) -> IResult<&'a [u8], Option<T>, CIFParseError>,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], T, CIFParseError> {
    move |i: &'a [u8]| -> IResult<&'a [u8], T, CIFParseError> {
        match inner(i)? {
            (rest, Some(val)) => Ok((rest, val)),

            (_rest, None) => Err(nom::Err::Error(CIFParseError::MandatoryFieldMissing(
                field_name, i,
            ))),
        }
    }
}

pub fn mandatory_str<'a>(
    field_name: &'static str,
    nchars: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a str, CIFParseError> {
    mandatory(field_name, string(nchars))
}

pub fn date_ddmmyy<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Date<Tz>, CIFParseError> {
    move |i: &'a [u8]| -> IResult<&'a [u8], Date<Tz>, CIFParseError> {
        let (i, dd) = take_while_m_n(2usize, 2, is_digit)(i)?;
        let (i, mm) = take_while_m_n(2usize, 2, is_digit)(i)?;
        let (i, yy) = take_while_m_n(2usize, 2, is_digit)(i)?;
        let dt = London.ymd(
            lexical_core::parse::<i32>(yy).map_err(CIFParseError::from_unrecoverable)? + 2000,
            lexical_core::parse(mm).map_err(CIFParseError::from_unrecoverable)?,
            lexical_core::parse(dd).map_err(CIFParseError::from_unrecoverable)?,
        );
        Ok((i, dt))
    }
}

pub fn date_yymmdd<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Date<Tz>, CIFParseError> {
    move |i: &'a [u8]| -> IResult<&'a [u8], Date<Tz>, CIFParseError> {
        let (i, yy) = take_while_m_n(2usize, 2, is_digit)(i)?;
        let (i, mm) = take_while_m_n(2usize, 2, is_digit)(i)?;
        let (i, dd) = take_while_m_n(2usize, 2, is_digit)(i)?;
        let dt = London.ymd(
            lexical_core::parse::<i32>(yy).map_err(CIFParseError::from_unrecoverable)? + 2000,
            lexical_core::parse(mm).map_err(CIFParseError::from_unrecoverable)?,
            lexical_core::parse(dd).map_err(CIFParseError::from_unrecoverable)?,
        );
        Ok((i, dt))
    }
}

pub fn time<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], NaiveTime, CIFParseError> {
    move |i: &'a [u8]| -> IResult<&'a [u8], NaiveTime, CIFParseError> {
        let start = i;
        let (i, hh) = take_while_m_n(2usize, 2, is_digit)(i)?;
        let (i, mm) = take_while_m_n(2usize, 2, is_digit)(i)?;

        let dt = NaiveTime::from_hms_opt(
            lexical_core::parse(hh).map_err(CIFParseError::from_unrecoverable)?,
            lexical_core::parse(mm).map_err(CIFParseError::from_unrecoverable)?,
            0,
        )
        .ok_or_else(|| CIFParseError::InvalidTime(start))
        .map_err(CIFParseError::from_unrecoverable)?;
        Ok((i, dt))
    }
}
pub fn time_half<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], NaiveTime, CIFParseError> {
    let time_p = time();
    move |i: &'a [u8]| -> IResult<&'a [u8], NaiveTime, CIFParseError> {
        let (i, t) = time_p(i)?;
        let (i, seconds) = alt((
            map(char(' '), |_| Duration::seconds(0)),
            map(char('H'), |_| Duration::seconds(30)),
        ))(i)?;

        Ok((i, t + seconds))
    }
}

#[inline(never)]
pub fn days<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Days, CIFParseError> {
    #[inline(never)]
    move |i: &'a [u8]| -> IResult<&'a [u8], Days, CIFParseError> {
        fn is_bit_char(c: u8) -> bool {
            c == b' ' || c == b'0' || c == b'1'
        }
        let (i, slice) = take_while_m_n(7, 7, is_bit_char)(i)?;
        let days = slice
            .iter()
            .zip(&[
                Days::MON,
                Days::TUE,
                Days::WED,
                Days::THU,
                Days::FRI,
                Days::SAT,
                Days::SUN,
            ])
            .fold(
                Days::empty(),
                |days, (ch, day)| if ch == &b'1' { days | *day } else { days },
            );
        Ok((i, days))
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
        let p = mandatory("field", inner);
        let (rest, result) = p(b"Hi").expect("parse");
        assert_eq!((rest, result), (b"Hi" as &[u8], ()));
    }

    #[test]
    fn mandatory_includes_field_name_in_error() {
        fn inner<'a>(i: &'a [u8]) -> IResult<&'a [u8], Option<()>, CIFParseError> {
            Ok((i, None))
        };
        let p = mandatory("field_name", inner);

        let err = p(b"  ").expect_err("should fail");
        assert!(
            format!("{:?}", err).contains("field_name"),
            "Error {:?} should contain field name: {:?}",
            err,
            "field_name"
        )
    }

    #[test]
    fn date_should_parse_ddmmyy() {
        let s = b"060315!!";
        let (rest, result) = date_ddmmyy()(s).expect("parse");
        assert_eq!((rest, result), (b"!!" as &[u8], London.ymd(2015, 3, 6)));
    }
    #[test]
    fn date_should_parse_yymmdd() {
        let s = b"150306!!";
        let (rest, result) = date_yymmdd()(s).expect("parse");
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

    #[test]
    fn time_half_should_parse_hhmm_as_start() {
        let s = b"2151 ";
        let (rest, result) = time_half()(s).expect("parse");
        assert_eq!(
            (rest, result),
            (b"" as &[u8], NaiveTime::from_hms(21, 51, 0))
        );
    }
    #[test]
    fn time_half_should_parse_hhmmh_as_half_minute() {
        let s = b"2151H";
        let (rest, result) = time_half()(s).expect("parse");
        assert_eq!(
            (rest, result),
            (b"" as &[u8], NaiveTime::from_hms(21, 51, 30))
        );
    }

    #[test]
    fn days_should_parse_bitwise_weekdays() {
        let s = b"1111100";
        let (rest, result) = days()(s).expect("parse");
        assert_eq!(
            (rest, result),
            (
                b"" as &[u8],
                Days::MON | Days::TUE | Days::WED | Days::THU | Days::FRI
            )
        );
    }
    #[test]
    fn days_should_parse_bitwise_saturday() {
        let s = b"0000010";
        let (rest, result) = days()(s).expect("parse");
        assert_eq!((rest, result), (b"" as &[u8], Days::SAT));
    }
    #[test]
    fn days_should_parse_bitwise_sunday() {
        let s = b"0000001";
        let (rest, result) = days()(s).expect("parse");
        assert_eq!((rest, result), (b"" as &[u8], Days::SUN));
    }
    #[test]
    fn days_should_parse_empty() {
        let s = b"       !";
        let (rest, result) = days()(s).expect("parse");
        assert_eq!((rest, result), (b"!" as &[u8], Days::empty()));
    }
}
