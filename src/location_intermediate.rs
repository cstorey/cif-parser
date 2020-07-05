use std::borrow::Cow;

use chrono::NaiveTime;
use nom::{bytes::streaming::*, character::is_space, IResult};

use crate::errors::CIFParseError;
use crate::helpers::*;
use crate::tiploc::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocationIntermediate<'a> {
    pub tiploc: Tiploc<'a>,
    pub scheduled_arrival_time: Option<NaiveTime>,
    pub scheduled_departure_time: Option<NaiveTime>,
    pub scheduled_pass: Option<NaiveTime>,
    pub public_arrival: Option<NaiveTime>,
    pub public_departure: Option<NaiveTime>,
    pub platform: Cow<'a, str>,
    pub line: Cow<'a, str>,
    pub path: Cow<'a, str>,
    pub activity: Cow<'a, str>,
    pub eng_allowance: Cow<'a, str>,
    pub path_allowance: Cow<'a, str>,
    pub perf_allowance: Cow<'a, str>,
}

pub(super) fn parse_location_intermediate<'a>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], LocationIntermediate, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], LocationIntermediate, CIFParseError> {
        let (i, _) = tag("LI")(i)?;
        let (i, tiploc) = Tiploc::parse(i)?;
        let (i, _) = take(1usize)(i)?;
        let (i, scheduled_arrival_time) = opt_time_half()(i)?;
        let (i, scheduled_departure_time) = opt_time_half()(i)?;
        let (i, scheduled_pass) = opt_time_half()(i)?;
        let (i, public_arrival) = opt_time()(i)?;
        let (i, public_departure) = opt_time()(i)?;
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
                tiploc,
                scheduled_arrival_time,
                scheduled_departure_time,
                scheduled_pass,
                public_arrival,
                public_departure,
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
                    tiploc: "WLOE".into(),
                    scheduled_arrival_time: NaiveTime::from_hms(23, 27, 0).into(),
                    scheduled_departure_time: NaiveTime::from_hms(23, 28, 0).into(),
                    scheduled_pass: None,
                    public_arrival: NaiveTime::from_hms(23, 27, 0).into(),
                    public_departure: NaiveTime::from_hms(23, 28, 0).into(),
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
