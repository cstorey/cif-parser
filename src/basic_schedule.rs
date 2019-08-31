use std::borrow::Cow;

use chrono::Date;
use chrono_tz::Tz;
use nom::{
    branch::alt, bytes::streaming::*, character::is_space, character::streaming::*,
    combinator::map, IResult,
};

use crate::errors::CIFParseError;
use crate::helpers::{date_yymmdd, mandatory_str, string};

use super::{TransactionType, STP};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BasicSchedule<'a> {
    pub transaction_type: TransactionType,
    pub uid: &'a str,
    pub start_date: Date<Tz>,
    pub end_date: Date<Tz>,
    pub days: &'a str,
    pub bank_holiday: Option<&'a str>,
    pub status: &'a str,
    pub category: &'a str,
    pub identity: &'a str,
    pub headcode: Option<&'a str>,
    pub service_code: &'a str,
    pub speed: &'a str,
    pub seating_class: &'a str,
    pub sleepers: Option<&'a str>,
    pub reservations: Option<&'a str>,
    pub catering: Option<&'a str>,
    pub branding: Option<&'a str>,
    pub stp: STP,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScheduleCancellation<'a> {
    pub transaction_type: TransactionType,
    pub uid: Cow<'a, str>,
    pub start_date: Date<Tz>,
    pub end_date: Date<Tz>,
    pub days: Cow<'a, str>,
    pub bank_holiday: Cow<'a, str>,
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
        let (i, end_date) = date_yymmdd()(i)?;
        let (i, days) = mandatory_str("days", 7usize)(i)?; // Bit string?
        let (i, bank_holiday) = string(1usize)(i)?;
        let (i, status) = mandatory_str("status", 1usize)(i)?;
        let (i, category) = mandatory_str("category", 2usize)(i)?;
        let (i, identity) = mandatory_str("identity", 4usize)(i)?;
        let (i, headcode) = string(4usize)(i)?;
        let (i, _course_indicator) = string(1usize)(i)?;
        let (i, service_code) = mandatory_str("service_code", 8usize)(i)?;
        let (i, _portion_id) = string(1usize)(i)?;
        let (i, _power_type) = mandatory_str("_power_type", 3usize)(i)?;
        let (i, _timing_load) = string(4usize)(i)?;
        let (i, speed) = mandatory_str("speed", 3usize)(i)?;
        let (i, _operating_characteristics) = string(6usize)(i)?;
        let (i, seating_class) = mandatory_str("seating_class", 1usize)(i)?;
        let (i, sleepers) = string(1usize)(i)?;
        let (i, reservations) = string(1usize)(i)?;
        let (i, _connection) = string(1usize)(i)?;
        let (i, catering) = string(4usize)(i)?;
        let (i, branding) = string(4usize)(i)?;
        let (i, _spare) = take_while_m_n(1, 1, is_space)(i)?;
        let (i, stp) = alt((
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

pub(super) fn parse_schedule_cancellation<'a>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], ScheduleCancellation, CIFParseError> {
    |i: &'a [u8]| -> IResult<&'a [u8], ScheduleCancellation, CIFParseError> {
        let (i, _) = tag("BS")(i)?;
        let (i, ttype) = alt((
            map(char('N'), |_| TransactionType::New),
            map(char('D'), |_| TransactionType::Delete),
            map(char('R'), |_| TransactionType::Revise),
        ))(i)?;
        let (i, uid) = take(6usize)(i)?;
        let (i, start_date) = date_yymmdd()(i)?;
        let (i, end_date) = date_yymmdd()(i)?;
        let (i, days) = take(7usize)(i)?; // Bit string?
        let (i, bank_holiday) = take(1usize)(i)?;
        let (i, _spare) = take_while_m_n(11, 11, is_space)(i)?;
        let (i, _course_indicator) = take(1usize)(i)?;
        let (i, _spare) = take_while_m_n(38, 38, is_space)(i)?;
        let (i, _stp) = tag("C")(i)?;

        Ok((
            i,
            ScheduleCancellation {
                transaction_type: ttype,
                uid: String::from_utf8_lossy(uid),
                start_date: start_date,
                end_date: end_date,
                days: String::from_utf8_lossy(days),
                bank_holiday: String::from_utf8_lossy(bank_holiday),
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
                end_date: London.ymd(2015, 10, 23),
                days: "1100100".into(),
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

        let p = parse_schedule_cancellation();
        eprintln!("{}", String::from_utf8_lossy(&[]));
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "\nZZ");
        assert_eq!(
            val,
            ScheduleCancellation {
                transaction_type: TransactionType::New,
                uid: "C67006".into(),
                start_date: London.ymd(2019, 5, 19),
                end_date: London.ymd(2019, 7, 28),
                days: "0000001".into(),
                bank_holiday: " ".into(),
            }
        )
    }

}
