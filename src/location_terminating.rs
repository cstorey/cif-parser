use std::borrow::Cow;

use nom::{bytes::streaming::*, character::is_space, error::*, IResult};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocationTerminating<'a> {
    tiploc: Cow<'a, str>,
    scheduled_arrival_time: Cow<'a, str>,
    public_arrival: Cow<'a, str>,
    platform: Cow<'a, str>,
    path: Cow<'a, str>,
    activity: Cow<'a, str>,
}

pub(super) fn parse_location_terminating<'a, E: ParseError<&'a [u8]>>(
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
}
