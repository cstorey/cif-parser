use nom::{bytes::streaming::*, character::is_space, IResult};

use crate::errors::CIFParseError;
use crate::helpers::{mandatory_str, string};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScheduleExtra<'a> {
    pub uic_code: Option<&'a str>,
    pub atoc_code: &'a str,
    pub applicable_timetable_code: &'a str,
}

pub(super) fn parse_schedule_extra<'a>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], ScheduleExtra, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], ScheduleExtra, CIFParseError> {
        let (i, _) = tag("BX")(i)?;
        let (i, _traction_class) = string(4usize)(i)?;
        let (i, uic_code) = string(5usize)(i)?;
        let (i, atoc_code) = mandatory_str("atoc_code", 2usize)(i)?;
        let (i, applicable_timetable_code) = mandatory_str("applicable_timetable_code", 1usize)(i)?;
        let (i, _reserved) = string(8usize)(i)?;
        let (i, _reserved) = string(1usize)(i)?;
        let (i, _spare) = take_while_m_n(57, 57, is_space)(i)?;

        Ok((
            i,
            ScheduleExtra {
                uic_code,
                atoc_code,
                applicable_timetable_code,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_parse_schedule_extra() {
        let p = parse_schedule_extra();
        let i = b"BX         SEY                                                                  ";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            val,
            ScheduleExtra {
                uic_code: None,
                atoc_code: "SE",
                applicable_timetable_code: "Y",
            }
        )
    }
}
