use std::borrow::Cow;

use nom::{bytes::streaming::*, character::is_space, IResult};

use crate::errors::CIFParseError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocationOrigin<'a> {
    tiploc: Cow<'a, str>,
    scheduled_departure_time: Cow<'a, str>,
    public_departure: Cow<'a, str>,
    platform: Cow<'a, str>,
    line: Cow<'a, str>,
    eng_allowance: Cow<'a, str>,
    path_allowance: Cow<'a, str>,
    activity: Cow<'a, str>,
    perf_allowance: Cow<'a, str>,
}

pub(super) fn parse_location_origin<'a>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], LocationOrigin, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], LocationOrigin, CIFParseError> {
        let (i, _) = tag("LO")(i)?; // 1-2
        let (i, tiploc) = take(8usize)(i)?; // 3-10
        let (i, scheduled_departure_time) = take(5usize)(i)?; // 11-15
        let (i, public_departure) = take(4usize)(i)?; // 16-19
        let (i, platform) = take(3usize)(i)?; // 20-22
        let (i, line) = take(3usize)(i)?; // 23-25
        let (i, eng_allowance) = take(2usize)(i)?;
        let (i, path_allowance) = take(2usize)(i)?;
        let (i, activity) = take(12usize)(i)?;
        let (i, perf_allowance) = take(2usize)(i)?;
        let (i, _spare) = take_while_m_n(37, 37, is_space)(i)?;

        Ok((
            i,
            LocationOrigin {
                tiploc: String::from_utf8_lossy(tiploc),
                scheduled_departure_time: String::from_utf8_lossy(scheduled_departure_time),
                public_departure: String::from_utf8_lossy(public_departure),
                platform: String::from_utf8_lossy(platform),
                line: String::from_utf8_lossy(line),
                eng_allowance: String::from_utf8_lossy(eng_allowance),
                path_allowance: String::from_utf8_lossy(path_allowance),
                activity: String::from_utf8_lossy(activity),
                perf_allowance: String::from_utf8_lossy(perf_allowance),
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_location_origin() {
        let p = parse_location_origin();
        let i = b"LOCHRX    0015 00156  FL     TB                                                 ";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        let rest = String::from_utf8_lossy(rest);
        assert_eq!(
            (val, &*rest),
            (
                LocationOrigin {
                    tiploc: "CHRX    ".into(),
                    scheduled_departure_time: "0015 ".into(),
                    public_departure: "0015".into(),
                    platform: "6  ".into(),
                    line: "FL ".into(),
                    eng_allowance: "  ".into(),
                    path_allowance: "  ".into(),
                    activity: "TB          ".into(),
                    perf_allowance: "  ".into(),
                },
                "",
            )
        )
    }

}
