use std::borrow::Cow;

use bitflags::bitflags;
use chrono::NaiveDate;
use chrono::{Duration, NaiveTime};

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
        Ok((
            rest,
            string_of_slice_opt(val).map_err(CIFParseError::from_unrecoverable)?,
        ))
    }
}

pub(crate) fn string_of_slice(val: &[u8]) -> Result<&str, std::str::Utf8Error> {
    let s = std::str::from_utf8(val)?.trim_end();
    Ok(s)
}
pub(crate) fn string_of_slice_opt(val: &[u8]) -> Result<Option<&str>, std::str::Utf8Error> {
    let s = string_of_slice(val)?;
    Ok(Some(s).filter(|val| !val.is_empty()))
}

pub fn mandatory<'a, T>(
    field_name: &'static str,
    inner: impl Fn(&'a [u8]) -> IResult<&'a [u8], Option<T>, CIFParseError>,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], T, CIFParseError> {
    move |i: &'a [u8]| -> IResult<&'a [u8], T, CIFParseError> {
        match inner(i)? {
            (rest, Some(val)) => Ok((rest, val)),

            (_rest, None) => Err(nom::Err::Error(CIFParseError::MandatoryFieldMissing(
                field_name,
                i.into(),
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

pub(crate) fn ddmmyy_from_slice(slice: &[u8]) -> Result<NaiveDate, CIFParseError> {
    let dd = lexical_core::parse(&slice[0..2])?;
    let mm = lexical_core::parse(&slice[2..4])?;
    let yy: i32 = lexical_core::parse(&slice[4..6])?;
    if let Some(dt) = NaiveDate::from_ymd_opt(yy + 2000, mm, dd) {
        Ok(dt)
    } else {
        Err(CIFParseError::InvalidTime(Cow::from(slice.to_owned())))
    }
}

pub(crate) fn yymmdd_from_slice(slice: &[u8]) -> Result<NaiveDate, CIFParseError> {
    let yy: i32 = lexical_core::parse(&slice[0..2])?;
    let mm = lexical_core::parse(&slice[2..4])?;
    let dd = lexical_core::parse(&slice[4..6])?;
    if let Some(dt) = NaiveDate::from_ymd_opt(yy + 2000, mm, dd) {
        Ok(dt)
    } else {
        Err(CIFParseError::InvalidTime(Cow::from(slice.to_owned())))
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
        .ok_or_else(|| CIFParseError::InvalidTime(start.into()))
        .map_err(CIFParseError::from_unrecoverable)?;
        Ok((i, dt))
    }
}

pub fn opt_time<'a>() -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Option<NaiveTime>, CIFParseError>
{
    alt((map(time(), Some), map(tag("    "), |_| None)))
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

pub fn opt_time_half<'a>(
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Option<NaiveTime>, CIFParseError> {
    alt((map(time_half(), Some), map(tag("     "), |_| None)))
}

pub(crate) fn days_from_slice(slice: &[u8]) -> Result<Days, CIFParseError> {
    const DAYS: &[Days] = &[
        Days::MON,
        Days::TUE,
        Days::WED,
        Days::THU,
        Days::FRI,
        Days::SAT,
        Days::SUN,
    ];

    let mut days = Days::empty();
    for (ch, day) in slice.iter().zip(DAYS) {
        match *ch {
            b'0' | b' ' => {}
            b'1' => {
                days |= *day;
            }
            _ => return Err(CIFParseError::InvalidItem),
        }
    }
    Ok(days)
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
    fn string_of_slice_should_read_value() {
        // string of length 3;
        let result = string_of_slice_opt(b"ABC").expect("should parse");
        assert_eq!(result, Some("ABC"))
    }

    #[test]
    fn string_of_slice_should_read_part() {
        // string of length 3;
        let result = string_of_slice_opt(b"AB ").expect("should parse");
        assert_eq!(result, Some("AB"))
    }

    #[test]
    fn string_of_slice_should_empty() {
        // string of length 3;
        let result = string_of_slice_opt(b"   ").expect("should parse");
        assert_eq!(result, None)
    }

    #[test]
    fn string_of_slice_return_remainder() {
        let p = string(3);
        let (rest, result) = p(b"A  DEF").expect("should parse");
        assert_eq!((rest, result), (b"DEF" as &[u8], Some("A")));
    }

    #[test]
    fn mandatory_should_return_ok_on_success() {
        fn inner<'a>(i: &'a [u8]) -> IResult<&'a [u8], Option<()>, CIFParseError> {
            Ok((i, Some(())))
        }
        let p = mandatory("field", inner);
        let (rest, result) = p(b"Hi").expect("parse");
        assert_eq!((rest, result), (b"Hi" as &[u8], ()));
    }

    #[test]
    fn mandatory_includes_field_name_in_error() {
        fn inner<'a>(i: &'a [u8]) -> IResult<&'a [u8], Option<()>, CIFParseError> {
            Ok((i, None))
        }
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
        let result = days_from_slice(s).expect("parse");
        assert_eq!(
            result,
            Days::MON | Days::TUE | Days::WED | Days::THU | Days::FRI
        );
    }
    #[test]
    fn days_should_parse_bitwise_saturday() {
        let s = b"0000010";
        let result = days_from_slice(s).expect("parse");
        assert_eq!(result, Days::SAT);
    }
    #[test]
    fn days_should_parse_bitwise_sunday() {
        let s = b"0000001";
        let result = days_from_slice(s).expect("parse");
        assert_eq!(result, Days::SUN);
    }
    #[test]
    fn days_should_parse_empty() {
        let s = b"       !";
        let result = days_from_slice(s).expect("parse");
        assert_eq!(result, Days::empty());
    }
}
