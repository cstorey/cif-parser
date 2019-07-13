use std::borrow::Cow;

use nom::{bytes::streaming::*, character::is_space, error::*, IResult};
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScheduleExtra<'a> {
    pub uic_code: Cow<'a, str>,
    pub atoc_code: Cow<'a, str>,
    pub applicable_timetable_code: Cow<'a, str>,
}

pub(super) fn parse_schedule_extra<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], ScheduleExtra, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], ScheduleExtra, E> {
        let (i, _) = tag("BX")(i)?;
        let (i, _traction_class) = take(4usize)(i)?;
        let (i, uic_code) = take(5usize)(i)?;
        let (i, atoc_code) = take(2usize)(i)?;
        let (i, applicable_timetable_code) = take(1usize)(i)?;
        let (i, _reserved) = take(8usize)(i)?;
        let (i, _reserved) = take(1usize)(i)?;
        let (i, _spare) = take_while_m_n(57, 57, is_space)(i)?;

        Ok((
            i,
            ScheduleExtra {
                uic_code: String::from_utf8_lossy(uic_code),
                atoc_code: String::from_utf8_lossy(atoc_code),
                applicable_timetable_code: String::from_utf8_lossy(applicable_timetable_code),
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_parse_schedule_extra() {
        let p = parse_schedule_extra::<VerboseError<_>>();
        let i = b"BX         SEY                                                                  ";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            val,
            ScheduleExtra {
                uic_code: "     ".into(),
                atoc_code: "SE".into(),
                applicable_timetable_code: "Y".into(),
            }
        )
    }

}
