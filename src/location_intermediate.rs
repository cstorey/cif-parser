use std::borrow::Cow;

use nom::{bytes::streaming::*, character::is_space, IResult};

use crate::errors::CIFParseError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocationIntermediate<'a> {
    tiploc: Cow<'a, str>,
    scheduled_arrival_time: Cow<'a, str>,
    scheduled_departure_time: Cow<'a, str>,
    scheduled_pass: Cow<'a, str>,
    public_arrival: Cow<'a, str>,
    public_departure: Cow<'a, str>,
    platform: Cow<'a, str>,
    line: Cow<'a, str>,
    path: Cow<'a, str>,
    activity: Cow<'a, str>,
    eng_allowance: Cow<'a, str>,
    path_allowance: Cow<'a, str>,
    perf_allowance: Cow<'a, str>,
}

pub(super) fn parse_location_intermediate<'a>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], LocationIntermediate, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], LocationIntermediate, CIFParseError> {
        let (i, _) = tag("LI")(i)?;
        let (i, tiploc) = take(8usize)(i)?;
        let (i, scheduled_arrival_time) = take(5usize)(i)?;
        let (i, scheduled_departure_time) = take(5usize)(i)?;
        let (i, scheduled_pass) = take(5usize)(i)?;
        let (i, public_arrival) = take(4usize)(i)?;
        let (i, public_departure) = take(4usize)(i)?;
        let (i, platform) = take(3usize)(i)?;
        let (i, line) = take(3usize)(i)?;
        let (i, path) = take(3usize)(i)?;
        let (i, activity) = take(12usize)(i)?;
        let (i, eng_allowance) = take(2usize)(i)?;
        let (i, path_allowance) = take(2usize)(i)?;
        let (i, perf_allowance) = take(2usize)(i)?;
        let (i, _spare) = take_while_m_n(20, 20, is_space)(i)?;

        Ok((
            i,
            LocationIntermediate {
                tiploc: String::from_utf8_lossy(tiploc),
                scheduled_arrival_time: String::from_utf8_lossy(scheduled_arrival_time),
                scheduled_departure_time: String::from_utf8_lossy(scheduled_departure_time),
                scheduled_pass: String::from_utf8_lossy(scheduled_pass),
                public_arrival: String::from_utf8_lossy(public_arrival),
                public_departure: String::from_utf8_lossy(public_departure),
                platform: String::from_utf8_lossy(platform),
                line: String::from_utf8_lossy(line),
                path: String::from_utf8_lossy(path),
                activity: String::from_utf8_lossy(activity),
                eng_allowance: String::from_utf8_lossy(eng_allowance),
                path_allowance: String::from_utf8_lossy(path_allowance),
                perf_allowance: String::from_utf8_lossy(perf_allowance),
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_location_intermediate() {
        let p = parse_location_intermediate();
        let i = b"LIWLOE    2327 2328      23272328C        T                                     ";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        let rest = String::from_utf8_lossy(rest);
        assert_eq!(
            (val, &*rest),
            (
                LocationIntermediate {
                    tiploc: "WLOE    ".into(),
                    scheduled_arrival_time: "2327 ".into(),
                    scheduled_departure_time: "2328 ".into(),
                    scheduled_pass: "     ".into(),
                    public_arrival: "2327".into(),
                    public_departure: "2328".into(),
                    platform: "C  ".into(),
                    line: "   ".into(),
                    path: "   ".into(),
                    eng_allowance: "  ".into(),
                    path_allowance: "  ".into(),
                    activity: "T           ".into(),
                    perf_allowance: "  ".into(),
                },
                "",
            )
        )
    }
}
