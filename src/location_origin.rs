use std::fmt;

use chrono::NaiveTime;

use crate::{
    errors::CIFParseError,
    helpers::{string_of_slice, string_of_slice_opt, time_from_slice, time_half_from_slice},
    reader::CifLine,
    Tiploc,
};

#[derive(Clone, Eq, PartialEq)]
pub struct LocationOrigin {
    record: CifLine,
}

impl LocationOrigin {
    pub(crate) fn from_record(record: CifLine) -> Self {
        Self { record }
    }
    pub fn buf(&self) -> &CifLine {
        &self.record
    }

    pub fn tiploc(&self) -> Result<Tiploc, CIFParseError> {
        let s = string_of_slice(&self.record[2..9])?;
        Ok(Tiploc::of_string(s.to_owned()))
    }
    pub fn tiploc_suffix(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[9..10])?)
    }
    pub fn scheduled_departure_time(&self) -> Result<NaiveTime, CIFParseError> {
        time_half_from_slice(&self.record[10..15])
    }
    pub fn public_departure(&self) -> Result<NaiveTime, CIFParseError> {
        time_from_slice(&self.record[15..19])
    }
    pub fn platform(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[19..22])?)
    }
    pub fn line(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[22..25])?)
    }
    pub fn eng_allowance(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[25..27])?)
    }
    pub fn path_allowance(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[27..29])?)
    }
    pub fn activity(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[29..41])?)
    }
    pub fn perf_allowance(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[41..43])?)
    }
}

impl fmt::Debug for LocationOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("LocationOrigin");
        s.field("tiploc_suffix", &self.tiploc_suffix());
        s.field("scheduled_departure_time", &self.scheduled_departure_time());
        s.field("public_departure", &self.public_departure());
        s.field("platform", &self.platform());
        s.field("line", &self.line());
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
    fn should_parse_location_origin() {
        let i = b"LOCHRX    0015 00156  FL     TB                                                 ";
        assert_eq!(80, i.len());
        let example = LocationOrigin::from_record(*i);
        println!("{:?}", example);
        assert_eq!(example.tiploc().unwrap(), Tiploc::from("CHRX"));
        assert_eq!(example.tiploc_suffix().unwrap(), None);
        assert_eq!(
            example.scheduled_departure_time().unwrap(),
            NaiveTime::from_hms_opt(0, 15, 0).unwrap()
        );
        assert_eq!(
            example.public_departure().unwrap(),
            NaiveTime::from_hms_opt(0, 15, 0).unwrap()
        );
        assert_eq!(example.platform().unwrap(), Some("6"));
        assert_eq!(example.line().unwrap(), Some("FL"));
        assert_eq!(example.eng_allowance().unwrap(), None);
        assert_eq!(example.path_allowance().unwrap(), None);
        assert_eq!(example.activity().unwrap(), Some("TB"));
        assert_eq!(example.perf_allowance().unwrap(), None);
    }
}
