use nom::{combinator::opt, multi::fold_many0, sequence::preceded, IResult};

use crate::errors::CIFParseError;
use crate::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Schedule<'a> {
    pub basic: BasicSchedule<'a>,
    pub extra: Option<ScheduleExtra<'a>>,
    pub origin: Option<LocationOrigin<'a>>,
    pub intermediate: Vec<LocationIntermediate<'a>>,
    pub changes: Vec<ChangeEnRoute<'a>>,
    pub terminal: Option<LocationTerminating<'a>>,
}

pub(super) fn parse_schedule<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Schedule, CIFParseError>
{
    |i: &'a [u8]| -> IResult<&'a [u8], Schedule, CIFParseError> {
        #[allow(clippy::clippy::large_enum_variant)]
        enum Intermediate<'a> {
            Location(LocationIntermediate<'a>),
            Change(ChangeEnRoute<'a>),
        }
        let (i, basic) = basic_schedule::parse_basic_schedule()(i)?;
        let (i, extra) = opt(preceded(char('\n'), schedule_extra::parse_schedule_extra()))(i)?;
        let (i, origin) = opt(preceded(
            char('\n'),
            location_origin::parse_location_origin(),
        ))(i)?;
        let intermediate_line = alt((
            map(
                location_intermediate::parse_location_intermediate(),
                Intermediate::Location,
            ),
            map(
                change_en_route::parse_change_en_route(),
                Intermediate::Change,
            ),
        ));
        let mut folder = fold_many0(
            preceded(char('\n'), intermediate_line),
            (Vec::new(), Vec::new()),
            |(mut changes, mut intermediate), it| {
                match it {
                    Intermediate::Change(cr) => changes.push(cr),
                    Intermediate::Location(li) => intermediate.push(li),
                };
                (changes, intermediate)
            },
        );
        let (i, (changes, intermediate)) = folder(i)?;
        let (i, terminal) = opt(preceded(
            char('\n'),
            location_terminating::parse_location_terminating(),
        ))(i)?;

        Ok((
            i,
            Schedule {
                basic,
                extra,
                origin,
                intermediate,
                changes,
                terminal,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use chrono::{offset::TimeZone, NaiveTime};
    use chrono_tz::Europe::London;

    use super::*;
    use crate::helpers::Days;

    #[test]
    fn should_parse_example() {
        let i = b"\
BSNW037511905191912080000001 POO2J43    124655005 EMU    090D     S            P\n\
BX         SEY                                                                  \n\
LOBROMLYN 0004 00041         TB                                                 \n\
LISNDP    0005H0006      00060006         T                                     \n\
LTGRVPK   0009 00091     TF                                                     ";

        let p = parse_schedule();
        eprintln!("{}", String::from_utf8_lossy(&[]));
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            val,
            Schedule {
                basic: BasicSchedule {
                    transaction_type: TransactionType::New,
                    uid: "W03751",
                    start_date: London.ymd(2019, 5, 19),
                    end_date: London.ymd(2019, 12, 8).into(),
                    days: Days::SUN,
                    bank_holiday: None,
                    status: "P".into(),
                    category: "OO".into(),
                    identity: "2J43".into(),
                    headcode: None,
                    service_code: "24655005".into(),
                    speed: "090".into(),
                    seating_class: "S".into(),
                    sleepers: None,
                    reservations: None,
                    catering: None,
                    branding: None,
                    stp: Stp::Permanent,
                },
                extra: Some(ScheduleExtra {
                    uic_code: None,
                    atoc_code: "SE",
                    applicable_timetable_code: "Y",
                }),
                origin: Some(LocationOrigin {
                    tiploc: "BROMLYN".into(),
                    tiploc_suffix: None,
                    scheduled_departure_time: NaiveTime::from_hms(0, 4, 0),
                    public_departure: NaiveTime::from_hms(0, 4, 0),
                    platform: "1  ".into(),
                    line: "   ".into(),
                    eng_allowance: "  ".into(),
                    path_allowance: "  ".into(),
                    activity: "TB          ".into(),
                    perf_allowance: "  ".into(),
                }),
                intermediate: vec![LocationIntermediate {
                    tiploc: "SNDP".into(),
                    scheduled_arrival_time: NaiveTime::from_hms(0, 5, 30).into(),
                    scheduled_departure_time: NaiveTime::from_hms(0, 6, 0).into(),
                    scheduled_pass: None,
                    public_arrival: NaiveTime::from_hms(0, 6, 0).into(),
                    public_departure: NaiveTime::from_hms(0, 6, 0).into(),
                    platform: "   ".into(),
                    line: "   ".into(),
                    path: "   ".into(),
                    activity: "T           ".into(),
                    eng_allowance: "  ".into(),
                    path_allowance: "  ".into(),
                    perf_allowance: "  ".into(),
                }],
                changes: vec![],
                terminal: Some(LocationTerminating {
                    tiploc: "GRVPK".into(),
                    scheduled_arrival_time: NaiveTime::from_hms(0, 9, 0),
                    public_arrival: NaiveTime::from_hms(0, 9, 0),
                    platform: "1  ".into(),
                    path: "   ".into(),
                    activity: "TF          ".into(),
                }),
            }
        )
    }
    //

    #[test]
    fn should_parse_cancellation_schedule() {
        let i = b"\
BSNC670061905191907280000001            1                                      C\n\
ZZ";

        let p = parse_schedule();
        eprintln!("{}", String::from_utf8_lossy(&[]));
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "\nZZ");
        assert_eq!(
            val,
            Schedule {
                basic: BasicSchedule {
                    transaction_type: TransactionType::New,
                    uid: "C67006",
                    start_date: London.ymd(2019, 5, 19),
                    end_date: Some(London.ymd(2019, 7, 28)),
                    days: Days::SUN,
                    bank_holiday: None,
                    status: None,
                    category: None,
                    identity: None,
                    headcode: None,
                    service_code: None,
                    speed: None,
                    seating_class: None,
                    sleepers: None,
                    reservations: None,
                    catering: None,
                    branding: None,
                    stp: Stp::Cancellation,
                },
                extra: None,
                origin: None,
                intermediate: Vec::new(),
                changes: Vec::new(),
                terminal: None,
            }
        )
    }
}
