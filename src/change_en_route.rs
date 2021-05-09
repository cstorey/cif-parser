use std::fmt;

use bytes::Bytes;

use crate::{
    helpers::{string_of_slice, string_of_slice_opt},
    CIFParseError, Tiploc,
};

#[derive(Clone, Eq, PartialEq)]
pub struct ChangeEnRoute {
    record: Bytes,
}

impl ChangeEnRoute {
    pub(crate) fn from_record(record: Bytes) -> Self {
        Self { record }
    }
    pub fn buf(&self) -> &Bytes {
        &self.record
    }

    pub fn tiploc(&self) -> Result<Tiploc, CIFParseError> {
        Ok(Tiploc::from(string_of_slice(&self.record[2..10])?))
    }
    pub fn train_category(&self) -> Result<&str, CIFParseError> {
        Ok(string_of_slice(&self.record[10..12])?)
    }
    pub fn train_identity(&self) -> Result<&str, CIFParseError> {
        Ok(string_of_slice(&self.record[12..16])?)
    }
    pub fn headcode(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[16..20])?)
    }
    pub fn course_indicator(&self) -> Result<&str, CIFParseError> {
        Ok(string_of_slice(&self.record[20..21])?)
    }
    pub fn service_code(&self) -> Result<&str, CIFParseError> {
        Ok(string_of_slice(&self.record[21..29])?)
    }
    pub fn biz_sector(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[29..30])?)
    }
    pub fn timing_load(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[33..37])?)
    }
    pub fn speed(&self) -> Result<&str, CIFParseError> {
        Ok(string_of_slice(&self.record[37..40])?)
    }
    pub fn operating_chars(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[40..46])?)
    }
    pub fn class(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[46..47])?)
    }
    pub fn sleepers(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[47..48])?)
    }
    pub fn reservations(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[48..49])?)
    }
    pub fn connect(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[49..50])?)
    }
    pub fn catering(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[50..54])?)
    }
    pub fn branding(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[54..58])?)
    }
    pub fn traction(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[58..62])?)
    }
    pub fn uic_code(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[62..67])?)
    }
    pub fn retail_id(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[67..75])?)
    }
}
impl fmt::Debug for ChangeEnRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("TiplocInsert");
        s.field("tiploc", &self.tiploc());
        s.field("train_category", &self.train_category());
        s.field("train_identity", &self.train_identity());
        s.field("headcode", &self.headcode());
        s.field("course_indicator", &self.course_indicator());
        s.field("service_code", &self.service_code());
        s.field("biz_sector", &self.biz_sector());
        s.field("timing_load", &self.timing_load());
        s.field("speed", &self.speed());
        s.field("operating_chars", &self.operating_chars());
        s.field("class", &self.class());
        s.field("sleepers", &self.sleepers());
        s.field("reservations", &self.reservations());
        s.field("connect", &self.connect());
        s.field("catering", &self.catering());
        s.field("branding", &self.branding());
        s.field("traction", &self.traction());
        s.field("uic_code", &self.uic_code());
        s.field("retail_id", &self.retail_id());
        s.finish()
    }
}

#[cfg(test)]
mod test {
    use crate::Tiploc;

    use super::*;

    #[test]
    fn should_parse_change_en_route() {
        let i = b"CRCTRDJN  DT3Q27    152495112 D      030                                        ";
        assert_eq!(80, i.len());
        let example = ChangeEnRoute::from_record(Bytes::from(i.as_ref()));
        println!("{:?}", example);

        assert_eq!(example.tiploc().unwrap(), Tiploc::from("CTRDJN"));
        assert_eq!(example.train_category().unwrap(), "DT");
        assert_eq!(example.train_identity().unwrap(), "3Q27");
        assert_eq!(example.headcode().unwrap(), None);
        assert_eq!(example.course_indicator().unwrap(), "1");
        assert_eq!(example.service_code().unwrap(), "52495112");
        assert_eq!(example.biz_sector().unwrap(), None);
        assert_eq!(example.timing_load().unwrap(), None);
        assert_eq!(example.speed().unwrap(), "030");
        assert_eq!(example.operating_chars().unwrap(), None);
        assert_eq!(example.class().unwrap(), None);
        assert_eq!(example.sleepers().unwrap(), None);
        assert_eq!(example.reservations().unwrap(), None);
        assert_eq!(example.connect().unwrap(), None);
        assert_eq!(example.catering().unwrap(), None);
        assert_eq!(example.branding().unwrap(), None);
        assert_eq!(example.traction().unwrap(), None);
        assert_eq!(example.uic_code().unwrap(), None);
        assert_eq!(example.retail_id().unwrap(), None);
    }
}
