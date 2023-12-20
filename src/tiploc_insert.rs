use std::fmt;

use helpers::string_of_slice_opt;

use crate::helpers::*;
use crate::reader::CifLine;
use crate::tiploc::Tiploc;
use crate::{errors::CIFParseError, helpers};

#[derive(Clone, Eq, PartialEq)]
pub struct TiplocInsert {
    record: CifLine,
}

impl TiplocInsert {
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
    pub fn nlc(&self) -> Result<&str, CIFParseError> {
        Ok(string_of_slice(&self.record[11..17])?)
    }
    pub fn nlc_check(&self) -> Result<&str, CIFParseError> {
        Ok(string_of_slice(&self.record[17..18])?)
    }
    pub fn tps_description(&self) -> Result<&str, CIFParseError> {
        let s = string_of_slice(&self.record[18..44])?;
        Ok(s)
    }
    pub fn stanox(&self) -> Result<&str, CIFParseError> {
        Ok(string_of_slice(&self.record[44..49])?)
    }
    pub fn crs(&self) -> Result<Option<&str>, CIFParseError> {
        let s = string_of_slice_opt(&self.record[53..56])?;
        Ok(s)
    }
    pub fn nlc_desc(&self) -> Result<Option<&str>, CIFParseError> {
        let s = string_of_slice_opt(&self.record[56..72])?;
        Ok(s)
    }
}

impl fmt::Debug for TiplocInsert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("TiplocInsert");
        s.field("tiploc", &self.tiploc());
        s.field("nlc", &self.nlc());
        s.field("nlc_check", &self.nlc_check());
        s.field("tps_description", &self.tps_description());
        s.field("stanox", &self.stanox());
        s.field("crs", &self.crs());
        s.field("nlc_desc", &self.nlc_desc());
        s.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_tiploc_insert() {
        let insert =
            b"TIBLTNODR24853600DBOLTON-UPON-DEARNE        24011   0BTDBOLTON ON DEARNE        ";
        assert_eq!(80, insert.len());
        let example = TiplocInsert::from_record(*insert);
        assert_eq!(example.tiploc().unwrap(), Tiploc::of_str("BLTNODR"));
        assert_eq!(example.nlc().unwrap(), "853600");
        assert_eq!(example.nlc_check().unwrap(), "D");
        assert_eq!(example.tps_description().unwrap(), "BOLTON-UPON-DEARNE");
        assert_eq!(example.stanox().unwrap(), "24011");
        assert_eq!(example.crs().unwrap(), Some("BTD"));
        assert_eq!(example.nlc_desc().unwrap(), Some("BOLTON ON DEARNE"));
    }

    #[test]
    fn should_parse_example_2() {
        let insert =
            b"TIAACHEN 00081601LAACHEN                    00005   0                           ";
        assert_eq!(80, insert.len());
        let example = TiplocInsert::from_record(*insert);
        assert_eq!(example.tiploc().unwrap(), Tiploc::of_str("AACHEN"));
        assert_eq!(example.nlc().unwrap(), "081601");
        assert_eq!(example.nlc_check().unwrap(), "L");
        assert_eq!(example.tps_description().unwrap(), "AACHEN");
        assert_eq!(example.stanox().unwrap(), "00005");
        assert_eq!(example.crs().unwrap(), None);
        assert_eq!(example.nlc_desc().unwrap(), None);
    }
}
