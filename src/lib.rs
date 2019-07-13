use std::borrow::Cow;

use nom::{
    branch::alt, bytes::streaming::*, character::is_space, character::streaming::*,
    combinator::map, error::*, sequence::terminated, IResult,
};

pub mod records;

pub use records::parse;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocationTerminating<'a> {
    tiploc: Cow<'a, str>,
    scheduled_arrival_time: Cow<'a, str>,
    public_arrival: Cow<'a, str>,
    platform: Cow<'a, str>,
    path: Cow<'a, str>,
    activity: Cow<'a, str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChangeEnRoute<'a> {
    tiploc: Cow<'a, str>,
    train_category: Cow<'a, str>,
    train_identity: Cow<'a, str>,
    headcode: Cow<'a, str>,
    course_indicator: Cow<'a, str>,
    service_code: Cow<'a, str>,
    biz_sector: Cow<'a, str>,
    timing_load: Cow<'a, str>,
    speed: Cow<'a, str>,
    operating_chars: Cow<'a, str>,
    class: Cow<'a, str>,
    sleepers: Cow<'a, str>,
    reservations: Cow<'a, str>,
    connect: Cow<'a, str>,
    catering: Cow<'a, str>,
    branding: Cow<'a, str>,
    traction: Cow<'a, str>,
    uic_code: Cow<'a, str>,
    retail_id: Cow<'a, str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Trailer;

fn parse_location_terminating<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], LocationTerminating, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], LocationTerminating, E> {
        let (i, _) = tag("LT")(i)?;
        let (i, tiploc) = take(8usize)(i)?;
        let (i, scheduled_arrival_time) = take(5usize)(i)?;
        let (i, public_arrival) = take(4usize)(i)?;
        let (i, platform) = take(3usize)(i)?;
        let (i, path) = take(3usize)(i)?;
        let (i, activity) = take(12usize)(i)?;
        let (i, _spare) = take_while_m_n(43, 43, is_space)(i)?;

        Ok((
            i,
            LocationTerminating {
                tiploc: String::from_utf8_lossy(tiploc),
                scheduled_arrival_time: String::from_utf8_lossy(scheduled_arrival_time),
                public_arrival: String::from_utf8_lossy(public_arrival),
                platform: String::from_utf8_lossy(platform),
                path: String::from_utf8_lossy(path),
                activity: String::from_utf8_lossy(activity),
            },
        ))
    }
}

fn parse_change_en_route<'a, E: ParseError<&'a [u8]>>(
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

fn parse_trailer<'a, E: ParseError<&'a [u8]>>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Trailer, E>
{
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
    fn should_parse_location_terminating() {
        let p = parse_location_terminating::<VerboseError<_>>();
        let i = b"LTTUNWELL 0125 01271     TF                                                     ";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        let rest = String::from_utf8_lossy(rest);
        assert_eq!(
            (val, &*rest),
            (
                LocationTerminating {
                    tiploc: "TUNWELL ".into(),
                    scheduled_arrival_time: "0125 ".into(),
                    public_arrival: "0127".into(),
                    platform: "1  ".into(),
                    path: "   ".into(),
                    activity: "TF          ".into(),
                },
                "",
            )
        )
    }
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
