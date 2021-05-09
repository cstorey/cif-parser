use std::fmt;

use bytes::Bytes;
use chrono::NaiveTime;

use crate::errors::*;
use crate::helpers::*;
use crate::tiploc::*;

#[derive(Clone, Eq, PartialEq)]
pub struct LocationTerminating {
    record: Bytes,
}

impl LocationTerminating {
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
    pub fn scheduled_arrival_time(&self) -> Result<NaiveTime, CIFParseError> {
        time_half_from_slice(&self.record[10..15])
    }
    pub fn public_arrival(&self) -> Result<NaiveTime, CIFParseError> {
        time_from_slice(&self.record[15..19])
    }
    pub fn platform(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[19..22])?)
    }
    pub fn path(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[22..25])?)
    }
    pub fn activity(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[25..37])?)
    }
}

impl fmt::Debug for LocationTerminating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("LocationOrigin");
        s.field("tiploc", &self.tiploc());
        s.field("scheduled_arrival_time", &self.scheduled_arrival_time());
        s.field("public_arrival", &self.public_arrival());
        s.field("platform", &self.platform());
        s.field("path", &self.path());
        s.field("activity", &self.activity());

        s.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_location_terminating() {
        let i = b"LTTUNWELL 0125 01271     TF                                                     ";
        assert_eq!(80, i.len());
        let example = LocationTerminating::from_record(Bytes::from(i.as_ref()));
        println!("{:?}", example);
        assert_eq!(example.tiploc().unwrap(), Tiploc::from("TUNWELL"));
        assert_eq!(
            example.scheduled_arrival_time().unwrap(),
            NaiveTime::from_hms(1, 25, 0)
        );
        assert_eq!(
            example.public_arrival().unwrap(),
            NaiveTime::from_hms(1, 27, 0)
        );
        assert_eq!(example.platform().unwrap(), Some("1"));
        assert_eq!(example.path().unwrap(), None);
        assert_eq!(example.activity().unwrap(), Some("TF"));
    }
}
