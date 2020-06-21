use chrono::Date;
use chrono_tz::Tz;
use nom::{
    branch::alt, bytes::streaming::*, character::is_space, character::streaming::*,
    combinator::map, IResult,
};

use crate::errors::CIFParseError;
use crate::helpers::{date_yymmdd, days, mandatory_str, string, Days};

use super::{TransactionType, STP};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BasicSchedule<'a> {
    pub transaction_type: TransactionType,
    pub uid: &'a str,
    pub start_date: Date<Tz>,
    pub end_date: Option<Date<Tz>>,
    pub days: Days,
    pub bank_holiday: Option<&'a str>,
    pub status: Option<&'a str>,
    pub category: Option<&'a str>,
    pub identity: Option<&'a str>,
    pub headcode: Option<&'a str>,
    pub service_code: Option<&'a str>,
    pub speed: Option<&'a str>,
    pub seating_class: Option<&'a str>,
    pub sleepers: Option<&'a str>,
    pub reservations: Option<&'a str>,
    pub catering: Option<&'a str>,
    pub branding: Option<&'a str>,
    pub stp: STP,
}

pub(super) fn parse_basic_schedule<'a>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], BasicSchedule, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], BasicSchedule, CIFParseError> {
        let (i, _) = tag("BS")(i)?;
        let (i, ttype) = alt((
            map(char('N'), |_| TransactionType::New),
            map(char('D'), |_| TransactionType::Delete),
            map(char('R'), |_| TransactionType::Revise),
        ))(i)?;
        let (i, uid) = mandatory_str("uid", 6usize)(i)?;
        let (i, start_date) = date_yymmdd()(i)?;
        let (i, end_date) = alt((
            map(date_yymmdd(), Some),
            map(take_while_m_n(6, 6, is_space), |_| None),
        ))(i)?;
        let (i, days) = days()(i)?; // Bit string?
        let (i, bank_holiday) = string(1usize)(i)?;
        let (i, status) = string(1usize)(i)?;
        let (i, category) = string(2usize)(i)?;
        let (i, identity) = string(4usize)(i)?;
        let (i, headcode) = string(4usize)(i)?;
        let (i, _course_indicator) = string(1usize)(i)?;
        let (i, service_code) = string(8usize)(i)?;
        let (i, _portion_id) = string(1usize)(i)?;
        let (i, _power_type) = string(3usize)(i)?;
        let (i, _timing_load) = string(4usize)(i)?;
        let (i, speed) = string(3usize)(i)?;
        let (i, _operating_characteristics) = string(6usize)(i)?;
        let (i, seating_class) = string(1usize)(i)?;
        let (i, sleepers) = string(1usize)(i)?;
        let (i, reservations) = string(1usize)(i)?;
        let (i, _connection) = string(1usize)(i)?;
        let (i, catering) = string(4usize)(i)?;
        let (i, branding) = string(4usize)(i)?;
        let (i, _spare) = take_while_m_n(1, 1, is_space)(i)?;
        let (i, stp) = alt((
            map(char('C'), |_| STP::Cancellation),
            map(char('N'), |_| STP::New),
            map(char('O'), |_| STP::Overlay),
            map(char('P'), |_| STP::Permanent),
        ))(i)?;

        Ok((
            i,
            BasicSchedule {
                transaction_type: ttype,
                uid: uid,
                start_date: start_date,
                end_date: end_date,
                days: days,
                bank_holiday: bank_holiday,
                status: status,
                category: category,
                identity: identity,
                headcode: headcode,
                service_code: service_code,
                speed: speed,
                seating_class: seating_class,
                sleepers: sleepers,
                reservations: reservations,
                catering: catering,
                branding: branding,
                stp: stp,
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
    fn should_parse_basic_schedule() {
        let p = parse_basic_schedule();
        let i = b"BSRG828851510191510231100100 POO2N75    113575825 DMUE   090      S            O";
        assert_eq!(80, i.len());
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            val,
            BasicSchedule {
                transaction_type: TransactionType::Revise,
                uid: "G82885".into(),
                start_date: London.ymd(2015, 10, 19),
                end_date: London.ymd(2015, 10, 23).into(),
                days: Days::MON | Days::TUE | Days::FRI,
                bank_holiday: None,
                status: "P".into(),
                category: "OO".into(),
                identity: "2N75".into(),
                headcode: None,
                service_code: "13575825".into(),
                speed: "090".into(),
                seating_class: "S".into(),
                sleepers: None,
                reservations: None,
                catering: None,
                branding: None,
                stp: STP::Overlay,
            }
        )
    }

    #[test]
    fn should_parse_cancellation_schedule() {
        let i = b"\
BSNC670061905191907280000001            1                                      C\n\
ZZ";

        let p = parse_basic_schedule();
        eprintln!("{}", String::from_utf8_lossy(&[]));
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "\nZZ");
        assert_eq!(
            val,
            BasicSchedule {
                transaction_type: TransactionType::New,
                uid: "C67006".into(),
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
                stp: STP::Cancellation,
            }
        )
    }

    #[test]
    fn should_parse_l63173() {
        let i = b"BSRL631731905191909290000001 POO2Y16    122214000 EMU375 075D                  P";
        let p = parse_basic_schedule();
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            val,
            BasicSchedule {
                transaction_type: TransactionType::Revise,
                uid: "L63173",
                start_date: London.ymd(2019, 5, 19),
                end_date: London.ymd(2019, 9, 29).into(),
                days: Days::SUN,
                bank_holiday: None,
                status: "P".into(),
                category: "OO".into(),
                identity: "2Y16".into(),
                headcode: None,
                service_code: "22214000".into(),
                speed: "075".into(),
                seating_class: None,
                sleepers: None,
                reservations: None,
                catering: None,
                branding: None,
                stp: STP::Permanent
            },
        )
    }
    #[test]
    fn should_parse_h19351() {
        let i = b"BSRH193511905201911011111100 F          1         D  600 060                   P";
        let p = parse_basic_schedule();
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            val,
            BasicSchedule {
                transaction_type: TransactionType::Revise,
                uid: "H19351",
                start_date: London.ymd(2019, 5, 20),
                end_date: London.ymd(2019, 11, 1).into(),
                days: Days::MON | Days::TUE | Days::WED | Days::THU | Days::FRI,
                bank_holiday: None,
                status: "F".into(),
                category: None,
                identity: None,
                headcode: None,
                service_code: None,
                speed: "060".into(),
                seating_class: None,
                sleepers: None,
                reservations: None,
                catering: None,
                branding: None,
                stp: STP::Permanent
            },
        )
    }
    //
    #[test]
    fn should_parse_c02189() {
        let i = b"BSNC021891905191912080000001 BBS0B00    122180008                              P";
        let p = parse_basic_schedule();
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            val,
            BasicSchedule {
                transaction_type: TransactionType::New,
                uid: "C02189",
                start_date: London.ymd(2019, 5, 19),
                end_date: London.ymd(2019, 12, 8).into(),
                days: Days::SUN,
                bank_holiday: None,
                status: "B".into(),
                category: Some("BS"),
                identity: Some("0B00"),
                headcode: None,
                service_code: Some("22180008"),
                speed: None,
                seating_class: None,
                sleepers: None,
                reservations: None,
                catering: None,
                branding: None,
                stp: STP::Permanent
            },
        )
    }

    #[test]
    fn should_parse_s48587() {
        let i = b"BSDS48587190525                                                                N";
        let p = parse_basic_schedule();
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "");
        assert_eq!(
            val,
            BasicSchedule {
                transaction_type: TransactionType::Delete,
                uid: "S48587",
                start_date: London.ymd(2019, 5, 25),
                end_date: None,
                days: Days::empty(),
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
                stp: STP::New,
            },
        )
    }
}
