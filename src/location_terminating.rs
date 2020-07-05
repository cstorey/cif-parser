use std::borrow::Cow;

use chrono::NaiveTime;
use nom::{bytes::streaming::*, character::is_space, IResult};

use crate::errors::*;
use crate::helpers::*;
use crate::tiploc::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocationTerminating<'a> {
    pub tiploc: Tiploc<'a>,
    pub scheduled_arrival_time: NaiveTime,
    pub public_arrival: NaiveTime,
    pub platform: Cow<'a, str>,
    pub path: Cow<'a, str>,
    pub activity: Cow<'a, str>,
}

pub(super) fn parse_location_terminating<'a>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], LocationTerminating, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], LocationTerminating, CIFParseError> {
        let (i, _) = tag("LT")(i)?;
        let (i, tiploc) = Tiploc::parse(i)?;
        let (i, _) = take(1usize)(i)?;
        let (i, scheduled_arrival_time) = time_half()(i)?;
        let (i, public_arrival) = time()(i)?;
        let (i, platform) = take(3usize)(i)?;
        let (i, path) = take(3usize)(i)?;
        let (i, activity) = take(12usize)(i)?;
        let (i, _spare) = take_while_m_n(43, 43, is_space)(i)?;

        Ok((
            i,
            LocationTerminating {
                tiploc,
                scheduled_arrival_time,
                public_arrival,
                platform: String::from_utf8_lossy(platform),
                path: String::from_utf8_lossy(path),
                activity: String::from_utf8_lossy(activity),
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_location_terminating() {
        let p = parse_location_terminating();
        let i = b"LTTUNWELL 0125 01271     TF                                                     ";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        let rest = String::from_utf8_lossy(rest);
        assert_eq!(
            (val, &*rest),
            (
                LocationTerminating {
                    tiploc: "TUNWELL".into(),
                    scheduled_arrival_time: NaiveTime::from_hms(1, 25, 0),
                    public_arrival: NaiveTime::from_hms(1, 27, 0),
                    platform: "1  ".into(),
                    path: "   ".into(),
                    activity: "TF          ".into(),
                },
                "",
            )
        )
    }
}
