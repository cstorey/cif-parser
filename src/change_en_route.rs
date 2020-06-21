use std::borrow::Cow;

use nom::{bytes::streaming::*, character::is_space, error::*, IResult};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChangeEnRoute<'a> {
    pub tiploc: Cow<'a, str>,
    pub train_category: Cow<'a, str>,
    pub train_identity: Cow<'a, str>,
    pub headcode: Cow<'a, str>,
    pub course_indicator: Cow<'a, str>,
    pub service_code: Cow<'a, str>,
    pub biz_sector: Cow<'a, str>,
    pub timing_load: Cow<'a, str>,
    pub speed: Cow<'a, str>,
    pub operating_chars: Cow<'a, str>,
    pub class: Cow<'a, str>,
    pub sleepers: Cow<'a, str>,
    pub reservations: Cow<'a, str>,
    pub connect: Cow<'a, str>,
    pub catering: Cow<'a, str>,
    pub branding: Cow<'a, str>,
    pub traction: Cow<'a, str>,
    pub uic_code: Cow<'a, str>,
    pub retail_id: Cow<'a, str>,
}

pub(super) fn parse_change_en_route<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], ChangeEnRoute, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], ChangeEnRoute, E> {
        let (i, _) = tag("CR")(i)?;
        let (i, tiploc) = take(8usize)(i)?;
        let (i, train_category) = take(2usize)(i)?;
        let (i, train_identity) = take(4usize)(i)?;
        let (i, headcode) = take(4usize)(i)?;
        let (i, course_indicator) = take(1usize)(i)?;
        let (i, service_code) = take(8usize)(i)?;
        let (i, biz_sector) = take(1usize)(i)?;
        let (i, _power_type) = take(3usize)(i)?;
        let (i, timing_load) = take(4usize)(i)?;
        let (i, speed) = take(3usize)(i)?;
        let (i, operating_chars) = take(6usize)(i)?;
        let (i, class) = take(1usize)(i)?;
        let (i, sleepers) = take(1usize)(i)?;
        let (i, reservations) = take(1usize)(i)?;
        let (i, connect) = take(1usize)(i)?;
        let (i, catering) = take(4usize)(i)?;
        let (i, branding) = take(4usize)(i)?;
        let (i, traction) = take(4usize)(i)?;
        let (i, uic_code) = take(5usize)(i)?;
        let (i, retail_id) = take(8usize)(i)?;
        let (i, _spare) = take_while_m_n(5, 5, is_space)(i)?;

        Ok((
            i,
            ChangeEnRoute {
                tiploc: String::from_utf8_lossy(tiploc),
                train_category: String::from_utf8_lossy(train_category),
                train_identity: String::from_utf8_lossy(train_identity),
                headcode: String::from_utf8_lossy(headcode),
                course_indicator: String::from_utf8_lossy(course_indicator),
                service_code: String::from_utf8_lossy(service_code),
                biz_sector: String::from_utf8_lossy(biz_sector),
                timing_load: String::from_utf8_lossy(timing_load),
                speed: String::from_utf8_lossy(speed),
                operating_chars: String::from_utf8_lossy(operating_chars),
                class: String::from_utf8_lossy(class),
                sleepers: String::from_utf8_lossy(sleepers),
                reservations: String::from_utf8_lossy(reservations),
                connect: String::from_utf8_lossy(connect),
                catering: String::from_utf8_lossy(catering),
                branding: String::from_utf8_lossy(branding),
                traction: String::from_utf8_lossy(traction),
                uic_code: String::from_utf8_lossy(uic_code),
                retail_id: String::from_utf8_lossy(retail_id),
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_change_en_route() {
        let p = parse_change_en_route::<VerboseError<_>>();
        let i = b"CRCTRDJN  DT3Q27    152495112 D      030                                        ";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        let rest = String::from_utf8_lossy(rest);
        assert_eq!(
            (val, &*rest),
            (
                ChangeEnRoute {
                    tiploc: "CTRDJN  ".into(),
                    train_category: "DT".into(),
                    train_identity: "3Q27".into(),
                    headcode: "    ".into(),
                    course_indicator: "1".into(),
                    service_code: "52495112".into(),
                    biz_sector: " ".into(),
                    timing_load: "    ".into(),
                    speed: "030".into(),
                    operating_chars: "      ".into(),
                    class: " ".into(),
                    sleepers: " ".into(),
                    reservations: " ".into(),
                    connect: " ".into(),
                    catering: "    ".into(),
                    branding: "    ".into(),
                    traction: "    ".into(),
                    uic_code: "     ".into(),
                    retail_id: "        ".into(),
                },
                "",
            )
        )
    }
}
