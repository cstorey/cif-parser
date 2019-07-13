use std::borrow::Cow;

use nom::{
    branch::alt, bytes::streaming::*, character::is_space, character::streaming::*,
    combinator::map, IResult,
};

use crate::errors::CIFParseError;
use crate::{TransactionType, STP};

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
    stp: STP,
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
            map(char('C'), |_| STP::Cancellation),
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

#[cfg(test)]
mod test {
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

}
