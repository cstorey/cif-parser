use std::fmt;

use bytes::Bytes;
use chrono::NaiveTime;

use crate::errors::CIFParseError;
use crate::helpers::*;
use crate::tiploc::*;

#[derive(Clone, Eq, PartialEq)]
pub struct LocationIntermediate {
    record: Bytes,
}

impl LocationIntermediate {
    pub(crate) fn from_record(record: Bytes) -> Self {
        Self { record }
    }
    pub fn buf(&self) -> &Bytes {
        &self.record
    }

    pub fn tiploc(&self) -> Result<Tiploc, CIFParseError> {
        let s = string_of_slice(&self.record[2..9])?;
        Ok(Tiploc::of_string(s.to_owned()))
    }
    pub fn tiploc_suffix(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[9..10])?)
    }
    pub fn scheduled_arrival_time(&self) -> Result<Option<NaiveTime>, CIFParseError> {
        time_half_from_slice_opt(&self.record[10..15])
    }
    pub fn scheduled_departure_time(&self) -> Result<Option<NaiveTime>, CIFParseError> {
        time_half_from_slice_opt(&self.record[15..20])
    }
    pub fn scheduled_pass(&self) -> Result<Option<NaiveTime>, CIFParseError> {
        time_half_from_slice_opt(&self.record[20..25])
    }
    pub fn public_arrival(&self) -> Result<Option<NaiveTime>, CIFParseError> {
        time_from_slice_opt(&self.record[25..29])
    }
    pub fn public_departure(&self) -> Result<Option<NaiveTime>, CIFParseError> {
        time_from_slice_opt(&self.record[29..33])
    }
    pub fn platform(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[33..36])?)
    }
    pub fn line(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[36..39])?)
    }
    pub fn path(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[39..42])?)
    }
    pub fn activity(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[42..54])?)
    }
    pub fn eng_allowance(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[54..56])?)
    }
    pub fn path_allowance(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[56..58])?)
    }
    pub fn perf_allowance(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[58..60])?)
    }
}

impl fmt::Debug for LocationIntermediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("LocationIntermediate");
        s.field("tiploc", &self.tiploc());
        s.field("scheduled_arrival_time", &self.scheduled_arrival_time());
        s.field("scheduled_departure_time", &self.scheduled_departure_time());
        s.field("scheduled_pass", &self.scheduled_pass());
        s.field("public_arrival", &self.public_arrival());
        s.field("public_departure", &self.public_departure());
        s.field("platform", &self.platform());
        s.field("line", &self.line());
        s.field("path", &self.path());
        s.field("eng_allowance", &self.eng_allowance());
        s.field("path_allowance", &self.path_allowance());
        s.field("activity", &self.activity());
        s.field("perf_allowance", &self.perf_allowance());
        s.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_location_intermediate() {
        let i = b"LIWLOE    2327 2328      23272328C        T                                     ";
        assert_eq!(80, i.len());
        let example = LocationIntermediate::from_record(Bytes::from(i.as_ref()));
        println!("{:?}", example);
        assert_eq!(example.tiploc().unwrap(), "WLOE".into());
        assert_eq!(
            example.scheduled_arrival_time().unwrap(),
            NaiveTime::from_hms(23, 27, 0).into()
        );
        assert_eq!(
            example.scheduled_departure_time().unwrap(),
            NaiveTime::from_hms(23, 28, 0).into()
        );
        assert_eq!(example.scheduled_pass().unwrap(), None);
        assert_eq!(
            example.public_arrival().unwrap(),
            NaiveTime::from_hms(23, 27, 0).into()
        );
        assert_eq!(
            example.public_departure().unwrap(),
            NaiveTime::from_hms(23, 28, 0).into()
        );
        assert_eq!(example.platform().unwrap(), Some("C"));
        assert_eq!(example.line().unwrap(), None);
        assert_eq!(example.path().unwrap(), None);
        assert_eq!(example.eng_allowance().unwrap(), None);
        assert_eq!(example.path_allowance().unwrap(), None);
        assert_eq!(example.activity().unwrap(), Some("T"));
        assert_eq!(example.perf_allowance().unwrap(), None);
    }
    #[test]
    fn should_parse_location_intermediate_2() {
        let i = b"LIKETRSJ            1211H00000000                                               ";
        assert_eq!(80, i.len());
        let example = LocationIntermediate::from_record(Bytes::from(i.as_ref()));
        println!("{:?}", example);
        assert_eq!(example.tiploc().unwrap(), "KETRSJ".into());
        assert_eq!(example.scheduled_arrival_time().unwrap(), None);
        assert_eq!(example.scheduled_departure_time().unwrap(), None);
        assert_eq!(
            example.scheduled_pass().unwrap(),
            Some(NaiveTime::from_hms(12, 11, 30))
        );
        assert_eq!(
            example.public_arrival().unwrap(),
            Some(NaiveTime::from_hms(0, 0, 0))
        );
        assert_eq!(
            example.public_departure().unwrap(),
            Some(NaiveTime::from_hms(0, 0, 0))
        );
        assert_eq!(example.platform().unwrap(), None);
        assert_eq!(example.line().unwrap(), None);
        assert_eq!(example.path().unwrap(), None);
        assert_eq!(example.eng_allowance().unwrap(), None);
        assert_eq!(example.path_allowance().unwrap(), None);
        assert_eq!(example.activity().unwrap(), None);
        assert_eq!(example.perf_allowance().unwrap(), None);
    }
}
