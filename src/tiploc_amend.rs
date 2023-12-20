use std::fmt;

use crate::{
    errors::CIFParseError,
    helpers::{string_of_slice, string_of_slice_opt},
    reader::CifLine,
    Tiploc,
};

#[derive(Clone, Eq, PartialEq)]
pub struct TiplocAmend {
    record: CifLine,
}

impl TiplocAmend {
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
    pub fn new_tiploc(&self) -> Result<Option<Tiploc>, CIFParseError> {
        if let Some(s) = string_of_slice_opt(&self.record[72..79])? {
            Ok(Some(Tiploc::of_string(s.to_owned())))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Debug for TiplocAmend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("TiplocAmend");
        s.field("tiploc", &self.tiploc());
        s.field("nlc", &self.nlc());
        s.field("nlc_check", &self.nlc_check());
        s.field("tps_description", &self.tps_description());
        s.field("stanox", &self.stanox());
        s.field("crs", &self.crs());
        s.field("nlc_desc", &self.nlc_desc());
        s.field("new_tiploc", &self.new_tiploc());
        s.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_tiploc_amend_a() {
        let amend =
            b"TAMBRK94200590970AMILLBROOK SIG E942        86536   0                           ";
        assert_eq!(80, amend.len());
        let example = TiplocAmend::from_record(*amend);

        assert_eq!(example.tiploc().expect("tiploc"), Tiploc::of_str("MBRK942"));
        assert_eq!(example.nlc().expect("nlc"), "590970");
        assert_eq!(example.nlc_check().expect("nlc_check"), "A");
        assert_eq!(
            example.tps_description().expect("tps_description"),
            "MILLBROOK SIG E942"
        );
        assert_eq!(example.stanox().expect("stanox"), "86536");
        assert_eq!(example.crs().expect("crs"), None);
        assert_eq!(example.nlc_desc().expect("nlc_desc"), None);
        assert_eq!(example.new_tiploc().expect("new_tiploc"), None);
    }
}
