use std::fmt;

use bytes::Bytes;
use chrono::NaiveDate;

use crate::helpers::{days_from_slice, string_of_slice_opt, yymmdd_from_slice, Days};
use crate::{errors::CIFParseError, helpers::string_of_slice};

use super::{Stp, TransactionType};

#[derive(Clone, Eq, PartialEq)]
pub struct BasicSchedule {
    record: Bytes,
}

impl BasicSchedule {
    pub(crate) fn from_record(record: Bytes) -> Self {
        Self { record }
    }

    pub fn transaction_type(&self) -> Result<TransactionType, CIFParseError> {
        match self.record[2] {
            b'N' => Ok(TransactionType::New),
            b'D' => Ok(TransactionType::Delete),
            b'R' => Ok(TransactionType::Revise),
            _ => Err(CIFParseError::InvalidItem),
        }
    }
    pub fn uid(&self) -> Result<&str, CIFParseError> {
        Ok(string_of_slice(&self.record[3..9])?)
    }
    pub fn start_date(&self) -> Result<NaiveDate, CIFParseError> {
        yymmdd_from_slice(&self.record[9..15])
    }
    pub fn end_date(&self) -> Result<Option<NaiveDate>, CIFParseError> {
        if let Some(s) = string_of_slice_opt(&self.record[15..21])? {
            let dt = yymmdd_from_slice(s.as_bytes())?;
            Ok(Some(dt))
        } else {
            Ok(None)
        }
    }
    pub fn days(&self) -> Result<Days, CIFParseError> {
        days_from_slice(&self.record[21..28])
    }
    pub fn bank_holiday(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[28..29])?)
    }
    pub fn status(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[29..30])?)
    }
    pub fn category(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[30..32])?)
    }
    pub fn identity(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[32..36])?)
    }
    pub fn headcode(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[36..40])?)
    }
    pub fn service_code(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[41..49])?)
    }
    pub fn speed(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[57..60])?)
    }
    pub fn seating_class(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[66..67])?)
    }
    pub fn sleepers(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[67..68])?)
    }
    pub fn reservations(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[68..69])?)
    }
    pub fn catering(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[70..74])?)
    }
    pub fn branding(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[74..78])?)
    }
    pub fn stp(&self) -> Result<Stp, CIFParseError> {
        match self.record[79] {
            b'C' => Ok(Stp::Cancellation),
            b'N' => Ok(Stp::New),
            b'O' => Ok(Stp::Overlay),
            b'P' => Ok(Stp::Permanent),
            _ => Err(CIFParseError::InvalidItem),
        }
    }
}

impl fmt::Debug for BasicSchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("BasicSchedule");
        s.field("transaction_type", &self.transaction_type());
        s.field("uid", &self.uid());
        s.field("start_date", &self.start_date());
        s.field("end_date", &self.end_date());
        s.field("days", &self.days());
        s.field("bank_holiday", &self.bank_holiday());
        s.field("status", &self.status());
        s.field("category", &self.category());
        s.field("identity", &self.identity());
        s.field("headcode", &self.headcode());
        s.field("service_code", &self.service_code());
        s.field("speed", &self.speed());
        s.field("seating_class", &self.seating_class());
        s.field("sleepers", &self.sleepers());
        s.field("reservations", &self.reservations());
        s.field("catering", &self.catering());
        s.field("branding", &self.branding());
        s.field("stp", &self.stp());

        s.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_basic_schedule() {
        let sched =
            b"BSRG828851510191510231100100 POO2N75    113575825 DMUE   090      S            O";
        assert_eq!(80, sched.len());
        let example = BasicSchedule::from_record(Bytes::from(sched.as_ref()));
        assert_eq!(example.transaction_type().unwrap(), TransactionType::Revise);
        assert_eq!(example.uid().unwrap(), "G82885");
        assert_eq!(
            example.start_date().unwrap(),
            NaiveDate::from_ymd(2015, 10, 19)
        );
        assert_eq!(
            example.end_date().unwrap(),
            NaiveDate::from_ymd(2015, 10, 23).into()
        );
        assert_eq!(example.days().unwrap(), Days::MON | Days::TUE | Days::FRI);
        assert_eq!(example.bank_holiday().unwrap(), None);
        assert_eq!(example.status().unwrap(), Some("P"));
        assert_eq!(example.category().unwrap(), Some("OO"));
        assert_eq!(example.identity().unwrap(), Some("2N75"));
        assert_eq!(example.headcode().unwrap(), None);
        assert_eq!(example.service_code().unwrap(), Some("13575825"));
        assert_eq!(example.speed().unwrap(), Some("090"));
        assert_eq!(example.seating_class().unwrap(), Some("S"));
        assert_eq!(example.sleepers().unwrap(), None);
        assert_eq!(example.reservations().unwrap(), None);
        assert_eq!(example.catering().unwrap(), None);
        assert_eq!(example.branding().unwrap(), None);
        assert_eq!(example.stp().unwrap(), Stp::Overlay);
    }

    #[test]
    fn should_parse_cancellation_schedule() {
        let sched =
            b"BSNC670061905191907280000001            1                                      C";

        assert_eq!(80, sched.len());
        let example = BasicSchedule::from_record(Bytes::from(sched.as_ref()));
        assert_eq!(example.transaction_type().unwrap(), TransactionType::New);
        assert_eq!(example.uid().unwrap(), "C67006");
        assert_eq!(
            example.start_date().unwrap(),
            NaiveDate::from_ymd(2019, 5, 19)
        );
        assert_eq!(
            example.end_date().unwrap(),
            Some(NaiveDate::from_ymd(2019, 7, 28))
        );
        assert_eq!(example.days().unwrap(), Days::SUN);
        assert_eq!(example.bank_holiday().unwrap(), None);
        assert_eq!(example.status().unwrap(), None);
        assert_eq!(example.category().unwrap(), None);
        assert_eq!(example.identity().unwrap(), None);
        assert_eq!(example.headcode().unwrap(), None);
        assert_eq!(example.service_code().unwrap(), None);
        assert_eq!(example.speed().unwrap(), None);
        assert_eq!(example.seating_class().unwrap(), None);
        assert_eq!(example.sleepers().unwrap(), None);
        assert_eq!(example.reservations().unwrap(), None);
        assert_eq!(example.catering().unwrap(), None);
        assert_eq!(example.branding().unwrap(), None);
        assert_eq!(example.stp().unwrap(), Stp::Cancellation);
    }

    #[test]
    fn should_parse_l63173() {
        let sched =
            b"BSRL631731905191909290000001 POO2Y16    122214000 EMU375 075D                  P";
        assert_eq!(80, sched.len());
        let example = BasicSchedule::from_record(Bytes::from(sched.as_ref()));
        assert_eq!(example.transaction_type().unwrap(), TransactionType::Revise);
        assert_eq!(example.uid().unwrap(), "L63173");
        assert_eq!(
            example.start_date().unwrap(),
            NaiveDate::from_ymd(2019, 5, 19)
        );
        assert_eq!(
            example.end_date().unwrap(),
            NaiveDate::from_ymd(2019, 9, 29).into()
        );
        assert_eq!(example.days().unwrap(), Days::SUN);
        assert_eq!(example.bank_holiday().unwrap(), None);
        assert_eq!(example.status().unwrap(), "P".into());
        assert_eq!(example.category().unwrap(), "OO".into());
        assert_eq!(example.identity().unwrap(), "2Y16".into());
        assert_eq!(example.headcode().unwrap(), None);
        assert_eq!(example.service_code().unwrap(), "22214000".into());
        assert_eq!(example.speed().unwrap(), "075".into());
        assert_eq!(example.seating_class().unwrap(), None);
        assert_eq!(example.sleepers().unwrap(), None);
        assert_eq!(example.reservations().unwrap(), None);
        assert_eq!(example.catering().unwrap(), None);
        assert_eq!(example.branding().unwrap(), None);
        assert_eq!(example.stp().unwrap(), Stp::Permanent);
    }
    #[test]
    fn should_parse_h19351() {
        let sched =
            b"BSRH193511905201911011111100 F          1         D  600 060                   P";
        assert_eq!(80, sched.len());
        let example = BasicSchedule::from_record(Bytes::from(sched.as_ref()));
        assert_eq!(example.transaction_type().unwrap(), TransactionType::Revise);
        assert_eq!(example.uid().unwrap(), "H19351");
        assert_eq!(
            example.start_date().unwrap(),
            NaiveDate::from_ymd(2019, 5, 20)
        );
        assert_eq!(
            example.end_date().unwrap(),
            NaiveDate::from_ymd(2019, 11, 1).into()
        );
        assert_eq!(
            example.days().unwrap(),
            Days::MON | Days::TUE | Days::WED | Days::THU | Days::FRI
        );
        assert_eq!(example.bank_holiday().unwrap(), None);
        assert_eq!(example.status().unwrap(), "F".into());
        assert_eq!(example.category().unwrap(), None);
        assert_eq!(example.identity().unwrap(), None);
        assert_eq!(example.headcode().unwrap(), None);
        assert_eq!(example.service_code().unwrap(), None);
        assert_eq!(example.speed().unwrap(), "060".into());
        assert_eq!(example.seating_class().unwrap(), None);
        assert_eq!(example.sleepers().unwrap(), None);
        assert_eq!(example.reservations().unwrap(), None);
        assert_eq!(example.catering().unwrap(), None);
        assert_eq!(example.branding().unwrap(), None);
        assert_eq!(example.stp().unwrap(), Stp::Permanent);
    }
    //
    #[test]
    fn should_parse_c02189() {
        let sched =
            b"BSNC021891905191912080000001 BBS0B00    122180008                              P";
        assert_eq!(80, sched.len());
        let example = BasicSchedule::from_record(Bytes::from(sched.as_ref()));
        assert_eq!(example.transaction_type().unwrap(), TransactionType::New);
        assert_eq!(example.uid().unwrap(), "C02189");
        assert_eq!(
            example.start_date().unwrap(),
            NaiveDate::from_ymd(2019, 5, 19)
        );
        assert_eq!(
            example.end_date().unwrap(),
            NaiveDate::from_ymd(2019, 12, 8).into()
        );
        assert_eq!(example.days().unwrap(), Days::SUN);
        assert_eq!(example.bank_holiday().unwrap(), None);
        assert_eq!(example.status().unwrap(), "B".into());
        assert_eq!(example.category().unwrap(), Some("BS"));
        assert_eq!(example.identity().unwrap(), Some("0B00"));
        assert_eq!(example.headcode().unwrap(), None);
        assert_eq!(example.service_code().unwrap(), Some("22180008"));
        assert_eq!(example.speed().unwrap(), None);
        assert_eq!(example.seating_class().unwrap(), None);
        assert_eq!(example.sleepers().unwrap(), None);
        assert_eq!(example.reservations().unwrap(), None);
        assert_eq!(example.catering().unwrap(), None);
        assert_eq!(example.branding().unwrap(), None);
        assert_eq!(example.stp().unwrap(), Stp::Permanent);
    }

    #[test]
    fn should_parse_s48587() {
        let sched =
            b"BSDS48587190525                                                                N";
        assert_eq!(80, sched.len());
        let example = BasicSchedule::from_record(Bytes::from(sched.as_ref()));
        assert_eq!(example.transaction_type().unwrap(), TransactionType::Delete);
        assert_eq!(example.uid().unwrap(), "S48587");
        assert_eq!(
            example.start_date().unwrap(),
            NaiveDate::from_ymd(2019, 5, 25)
        );
        assert_eq!(example.end_date().unwrap(), None);
        assert_eq!(example.days().unwrap(), Days::empty());
        assert_eq!(example.bank_holiday().unwrap(), None);
        assert_eq!(example.status().unwrap(), None);
        assert_eq!(example.category().unwrap(), None);
        assert_eq!(example.identity().unwrap(), None);
        assert_eq!(example.headcode().unwrap(), None);
        assert_eq!(example.service_code().unwrap(), None);
        assert_eq!(example.speed().unwrap(), None);
        assert_eq!(example.seating_class().unwrap(), None);
        assert_eq!(example.sleepers().unwrap(), None);
        assert_eq!(example.reservations().unwrap(), None);
        assert_eq!(example.catering().unwrap(), None);
        assert_eq!(example.branding().unwrap(), None);
        assert_eq!(example.stp().unwrap(), Stp::New);
    }
}
