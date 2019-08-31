use nom::{combinator::opt, multi::fold_many0, sequence::preceded, IResult};

use crate::errors::CIFParseError;
use crate::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Schedule<'a> {
    basic: BasicSchedule<'a>,
    extra: Option<ScheduleExtra<'a>>,
    origin: Option<LocationOrigin<'a>>,
    intermediate: Vec<LocationIntermediate<'a>>,
    changes: Vec<ChangeEnRoute<'a>>,
    terminal: Option<LocationTerminating<'a>>,
}

pub(super) fn parse_schedule<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Schedule, CIFParseError>
{
    |i: &'a [u8]| -> IResult<&'a [u8], Schedule, CIFParseError> {
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
        let folder = fold_many0(
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
                terminal,
                changes,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use chrono::offset::TimeZone;
    use chrono_tz::Europe::London;

    use super::*;

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
                    uid: "W03751".into(),
                    start_date: London.ymd(2019, 5, 19),
                    end_date: London.ymd(2019, 12, 8),
                    days: "0000001".into(),
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
                    stp: STP::Permanent,
                },
                extra: Some(ScheduleExtra {
                    uic_code: "     ".into(),
                    atoc_code: "SE".into(),
                    applicable_timetable_code: "Y".into(),
                }),
                origin: Some(LocationOrigin {
                    tiploc: "BROMLYN ".into(),
                    scheduled_departure_time: "0004 ".into(),
                    public_departure: "0004".into(),
                    platform: "1  ".into(),
                    line: "   ".into(),
                    eng_allowance: "  ".into(),
                    path_allowance: "  ".into(),
                    activity: "TB          ".into(),
                    perf_allowance: "  ".into(),
                }),
                intermediate: vec![LocationIntermediate {
                    tiploc: "SNDP    ".into(),
                    scheduled_arrival_time: "0005H".into(),
                    scheduled_departure_time: "0006 ".into(),
                    scheduled_pass: "     ".into(),
                    public_arrival: "0006".into(),
                    public_departure: "0006".into(),
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
                    tiploc: "GRVPK   ".into(),
                    scheduled_arrival_time: "0009 ".into(),
                    public_arrival: "0009".into(),
                    platform: "1  ".into(),
                    path: "   ".into(),
                    activity: "TF          ".into(),
                }),
            }
        )
    }
}
