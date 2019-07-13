use std::borrow::Cow;
use std::marker::PhantomData;

use nom::{
    branch::alt, bytes::streaming::*, character::is_space, character::streaming::*,
    combinator::map, error::*, sequence::terminated, IResult,
};

pub mod records;

pub use records::parse;

#[derive(Debug, Clone, Eq, PartialEq)]
enum TransactionType {
    New,
    Delete,
    Revise,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum AssociationSTP {
    Cancellation,
    New,
    Overlay,
    Permanent,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Association<'a> {
    _phantom: PhantomData<&'a [u8]>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BasicSchedule<'a> {
    transaction_type: TransactionType,
    uid: Cow<'a, str>,
    start_date: Cow<'a, str>,
    end_date: Cow<'a, str>,
    days: Cow<'a, str>,
    bank_holiday: Cow<'a, str>,
    status: Cow<'a, str>,
    category: Cow<'a, str>,
    identity: Cow<'a, str>,
    headcode: Cow<'a, str>,

    service_code: Cow<'a, str>,
    speed: Cow<'a, str>,
    seating_class: Cow<'a, str>,
    sleepers: Cow<'a, str>,
    reservations: Cow<'a, str>,
    catering: Cow<'a, str>,
    branding: Cow<'a, str>,
    stp: AssociationSTP,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScheduleExtra<'a> {
    uic_code: Cow<'a, str>,
    atoc_code: Cow<'a, str>,
    applicable_timetable_code: Cow<'a, str>,
}

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

fn parse_association<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Association, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], Association, E> {
        let (i, _) = tag("AA")(i)?;
        let (i, _spare) = take(78usize)(i)?;

        Ok((
            i,
            Association {
                _phantom: PhantomData,
            },
        ))
    }
}

fn parse_basic_schedule<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], BasicSchedule, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], BasicSchedule, E> {
        let (i, _) = tag("BS")(i)?;
        let (i, ttype) = alt((
            map(char('N'), |_| TransactionType::New),
            map(char('D'), |_| TransactionType::Delete),
            map(char('R'), |_| TransactionType::Revise),
        ))(i)?;
        let (i, uid) = take(6usize)(i)?;
        let (i, start_date) = take(6usize)(i)?;
        let (i, end_date) = take(6usize)(i)?;
        let (i, days) = take(7usize)(i)?; // Bit string?
        let (i, bank_holiday) = take(1usize)(i)?;
        let (i, status) = take(1usize)(i)?;
        let (i, category) = take(2usize)(i)?;
        let (i, identity) = take(4usize)(i)?;
        let (i, headcode) = take(4usize)(i)?;
        let (i, _) = take(1usize)(i)?;
        let (i, service_code) = take(8usize)(i)?;
        let (i, _portion_id) = take(1usize)(i)?;
        let (i, _power_type) = take(3usize)(i)?;
        let (i, _timing_load) = take(4usize)(i)?;
        let (i, speed) = take(3usize)(i)?;
        let (i, _operating_characteristics) = take(6usize)(i)?;
        let (i, seating_class) = take(1usize)(i)?;
        let (i, sleepers) = take(1usize)(i)?;
        let (i, reservations) = take(1usize)(i)?;
        let (i, _connection) = take(1usize)(i)?;
        let (i, catering) = take(4usize)(i)?;
        let (i, branding) = take(4usize)(i)?;
        let (i, _spare) = take_while_m_n(1, 1, is_space)(i)?;
        let (i, stp) = alt((
            map(char('C'), |_| AssociationSTP::Cancellation),
            map(char('N'), |_| AssociationSTP::New),
            map(char('O'), |_| AssociationSTP::Overlay),
            map(char('P'), |_| AssociationSTP::Permanent),
        ))(i)?;

        Ok((
            i,
            BasicSchedule {
                transaction_type: ttype,
                uid: String::from_utf8_lossy(uid),
                start_date: String::from_utf8_lossy(start_date),
                end_date: String::from_utf8_lossy(end_date),
                days: String::from_utf8_lossy(days),
                bank_holiday: String::from_utf8_lossy(bank_holiday),
                status: String::from_utf8_lossy(status),
                category: String::from_utf8_lossy(category),
                identity: String::from_utf8_lossy(identity),
                headcode: String::from_utf8_lossy(headcode),
                service_code: String::from_utf8_lossy(service_code),
                speed: String::from_utf8_lossy(speed),
                seating_class: String::from_utf8_lossy(seating_class),
                sleepers: String::from_utf8_lossy(sleepers),
                reservations: String::from_utf8_lossy(reservations),
                catering: String::from_utf8_lossy(catering),
                branding: String::from_utf8_lossy(branding),
                stp: stp,
            },
        ))
    }
}

fn parse_schedule_extra<'a, E: ParseError<&'a [u8]>>(
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

fn parse_location_origin<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], LocationOrigin, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], LocationOrigin, E> {
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

fn parse_location_intermediate<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], LocationIntermediate, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], LocationIntermediate, E> {
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
    use nom::combinator::complete;

    #[test]
    fn should_parse_association() {
        let p = complete(parse_association::<VerboseError<_>>());
        let hdr =
            b"AANY80987Y808801601041602121111100JJSPRST     TP                               P";
        assert_eq!(80, hdr.len());
        let (rest, insert) = p(hdr).expect("parse_header");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            insert,
            Association {
                _phantom: PhantomData,
            }
        )
    }
    #[test]
    fn should_parse_basic_schedule() {
        let p = complete(parse_basic_schedule::<VerboseError<_>>());
        let i = b"BSRG828851510191510231100100 POO2N75    113575825 DMUE   090      S            O";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            val,
            BasicSchedule {
                transaction_type: TransactionType::Revise,
                uid: "G82885".into(),
                start_date: "151019".into(),
                end_date: "151023".into(),
                days: "1100100".into(),
                bank_holiday: " ".into(),
                status: "P".into(),
                category: "OO".into(),
                identity: "2N75".into(),
                headcode: "    ".into(),
                service_code: "13575825".into(),
                speed: "090".into(),
                seating_class: "S".into(),
                sleepers: " ".into(),
                reservations: " ".into(),
                catering: "    ".into(),
                branding: "    ".into(),
                stp: AssociationSTP::Overlay,
            }
        )
    }
    #[test]
    fn should_parse_schedule_extra() {
        let p = complete(parse_schedule_extra::<VerboseError<_>>());
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
    #[test]
    fn should_parse_location_origin() {
        let p = parse_location_origin::<VerboseError<_>>();
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

    #[test]
    fn should_parse_location_intermediate() {
        let p = parse_location_intermediate::<VerboseError<_>>();
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
