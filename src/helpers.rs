use bitflags::bitflags;
use bytes::Bytes;
use chrono::NaiveDate;
use chrono::NaiveTime;

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

pub(crate) fn string_of_slice(val: &[u8]) -> Result<&str, std::str::Utf8Error> {
    let s = std::str::from_utf8(val)?.trim_end();
    Ok(s)
}
pub(crate) fn string_of_slice_opt(val: &[u8]) -> Result<Option<&str>, std::str::Utf8Error> {
    let s = string_of_slice(val)?;
    Ok(Some(s).filter(|val| !val.is_empty()))
}

pub(crate) fn ddmmyy_from_slice(slice: &[u8]) -> Result<NaiveDate, CIFParseError> {
    let dd = lexical_core::parse(&slice[0..2])?;
    let mm = lexical_core::parse(&slice[2..4])?;
    let yy: i32 = lexical_core::parse(&slice[4..6])?;
    if let Some(dt) = NaiveDate::from_ymd_opt(yy + 2000, mm, dd) {
        Ok(dt)
    } else {
        Err(CIFParseError::InvalidTime(Bytes::copy_from_slice(slice)))
    }
}

pub(crate) fn yymmdd_from_slice(slice: &[u8]) -> Result<NaiveDate, CIFParseError> {
    let yy: i32 = lexical_core::parse(&slice[0..2])?;
    let mm = lexical_core::parse(&slice[2..4])?;
    let dd = lexical_core::parse(&slice[4..6])?;
    if let Some(dt) = NaiveDate::from_ymd_opt(yy + 2000, mm, dd) {
        Ok(dt)
    } else {
        Err(CIFParseError::InvalidTime(Bytes::copy_from_slice(slice)))
    }
}

pub(crate) fn time_from_slice(slice: &[u8]) -> Result<NaiveTime, CIFParseError> {
    let hh = &slice[0..2];
    let mm = &slice[2..4];
    let dt = NaiveTime::from_hms_opt(lexical_core::parse(hh)?, lexical_core::parse(mm)?, 0)
        .ok_or_else(|| CIFParseError::InvalidTime(Bytes::copy_from_slice(slice)))?;
    Ok(dt)
}

pub(crate) fn time_from_slice_opt(slice: &[u8]) -> Result<Option<NaiveTime>, CIFParseError> {
    if slice[0] == b' ' {
        Ok(None)
    } else {
        let t = time_from_slice(slice)?;
        Ok(Some(t))
    }
}

pub(crate) fn time_half_from_slice(slice: &[u8]) -> Result<NaiveTime, CIFParseError> {
    let hh = lexical_core::parse(&slice[0..2])?;
    let mm = lexical_core::parse(&slice[2..4])?;
    let ss = match slice[4] {
        b' ' => 0,
        b'H' => 30,
        _ => return Err(CIFParseError::InvalidTime(Bytes::copy_from_slice(slice))),
    };
    let dt = NaiveTime::from_hms_opt(hh, mm, ss)
        .ok_or_else(|| CIFParseError::InvalidTime(Bytes::copy_from_slice(slice)))?;
    Ok(dt)
}
pub(crate) fn time_half_from_slice_opt(slice: &[u8]) -> Result<Option<NaiveTime>, CIFParseError> {
    if slice[0] == b' ' {
        Ok(None)
    } else {
        let t = time_half_from_slice(slice)?;
        Ok(Some(t))
    }
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
