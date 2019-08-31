use std::borrow::Cow;

use nom::{
    branch::alt, bytes::streaming::*, character::is_space, character::streaming::*,
    combinator::map, error::*, IResult,
};

use super::{TransactionType, STP};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BasicSchedule<'a> {
    pub transaction_type: TransactionType,
    pub uid: Cow<'a, str>,
    pub start_date: Cow<'a, str>,
    pub end_date: Cow<'a, str>,
    pub days: Cow<'a, str>,
    pub bank_holiday: Cow<'a, str>,
    pub status: Cow<'a, str>,
    pub category: Cow<'a, str>,
    pub identity: Cow<'a, str>,
    pub headcode: Cow<'a, str>,
    pub service_code: Cow<'a, str>,
    pub speed: Cow<'a, str>,
    pub seating_class: Cow<'a, str>,
    pub sleepers: Cow<'a, str>,
    pub reservations: Cow<'a, str>,
    pub catering: Cow<'a, str>,
    pub branding: Cow<'a, str>,
    pub stp: STP,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScheduleCancellation<'a> {
    pub transaction_type: TransactionType,
    pub uid: Cow<'a, str>,
    pub start_date: Cow<'a, str>,
    pub end_date: Cow<'a, str>,
    pub days: Cow<'a, str>,
    pub bank_holiday: Cow<'a, str>,
}

pub(super) fn parse_basic_schedule<'a, E: ParseError<&'a [u8]>>(
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
            map(char('N'), |_| STP::New),
            map(char('O'), |_| STP::Overlay),
            map(char('P'), |_| STP::Permanent),
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

pub(super) fn parse_schedule_cancellation<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], ScheduleCancellation, E> {
    |i: &'a [u8]| -> IResult<&'a [u8], ScheduleCancellation, E> {
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
        let (i, _spare) = take_while_m_n(11, 11, is_space)(i)?;
        let (i, _course_indicator) = take(1usize)(i)?;
        let (i, _spare) = take_while_m_n(38, 38, is_space)(i)?;
        let (i, _stp) = tag("C")(i)?;

        Ok((
            i,
            ScheduleCancellation {
                transaction_type: ttype,
                uid: String::from_utf8_lossy(uid),
                start_date: String::from_utf8_lossy(start_date),
                end_date: String::from_utf8_lossy(end_date),
                days: String::from_utf8_lossy(days),
                bank_holiday: String::from_utf8_lossy(bank_holiday),
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_basic_schedule() {
        let p = parse_basic_schedule::<VerboseError<_>>();
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
                stp: STP::Overlay,
            }
        )
    }

    #[test]
    fn should_parse_cancellation_schedule() {
        let i = b"\
BSNC670061905191907280000001            1                                      C\n\
ZZ";

        let p = parse_schedule_cancellation::<VerboseError<_>>();
        eprintln!("{}", String::from_utf8_lossy(&[]));
        let (rest, val) = p(i).expect("parse");
        assert_eq!(String::from_utf8_lossy(rest), "\nZZ");
        assert_eq!(
            val,
            ScheduleCancellation {
                transaction_type: TransactionType::New,
                uid: "C67006".into(),
                start_date: "190519".into(),
                end_date: "190728".into(),
                days: "0000001".into(),
                bank_holiday: " ".into(),
            }
        )
    }

}
