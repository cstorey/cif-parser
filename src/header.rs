use std::fmt;

use bytes::Bytes;
use chrono::{NaiveDate, NaiveDateTime};

use crate::helpers::string_of_slice_opt;
use crate::{errors::CIFParseError, helpers::ddmmyy_from_slice};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FullOrUpdate {
    Full,
    Update,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Header {
    record: Bytes,
}

impl Header {
    pub(crate) fn from_record(record: Bytes) -> Self {
        Header { record }
    }

    pub fn file_mainframe_identity(&self) -> Result<&str, CIFParseError> {
        Ok(std::str::from_utf8(&self.record[2..22])?)
    }

    pub fn extracted_at(&self) -> Result<NaiveDateTime, CIFParseError> {
        let dd = lexical_core::parse(&self.record[22..24])?;
        let mm = lexical_core::parse(&self.record[24..26])?;
        let yy: i32 = lexical_core::parse(&self.record[26..28])?;
        let h = lexical_core::parse(&self.record[28..30])?;
        let m = lexical_core::parse(&self.record[30..32])?;
        if let Some(dt) = NaiveDate::from_ymd_opt(yy + 2000, mm, dd) {
            Ok(dt.and_hms(h, m, 0))
        } else {
            Err(CIFParseError::InvalidTime(Bytes::copy_from_slice(
                &self.record[22..32],
            )))
        }
    }

    pub fn current_file(&self) -> Result<&str, CIFParseError> {
        Ok(std::str::from_utf8(&self.record[32..39])?)
    }
    pub fn last_file(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[39..46])?)
    }
    pub fn update(&self) -> Result<FullOrUpdate, CIFParseError> {
        let val = match self.record[46] {
            b'F' => FullOrUpdate::Full,
            b'U' => FullOrUpdate::Update,
            _ => return Err(CIFParseError::InvalidItem),
        };
        Ok(val)
    }

    pub fn version(&self) -> Result<&str, CIFParseError> {
        Ok(std::str::from_utf8(&self.record[47..48])?)
    }
    pub fn user_start_date(&self) -> Result<NaiveDate, CIFParseError> {
        ddmmyy_from_slice(&self.record[48..54])
    }
    pub fn user_end_date(&self) -> Result<NaiveDate, CIFParseError> {
        ddmmyy_from_slice(&self.record[54..60])
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Header");
        s.field("extracted_at", &self.extracted_at());
        s.field("current_file", &self.current_file());
        s.field("last_file", &self.last_file());
        s.field("update", &self.update());
        s.field("version", &self.version());
        s.field("user_start_date", &self.user_start_date());
        s.field("user_end_date", &self.user_end_date());
        s.finish()
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn should_parse_file_identity() {
        let header = example();
        assert_eq!(
            header.file_mainframe_identity().unwrap(),
            "TPS.UDFROC1.PD200628"
        );
    }

    #[test]
    fn should_parse_extracted_date() {
        let header = example();
        assert_eq!(
            header.extracted_at().unwrap(),
            NaiveDate::from_ymd(2020, 6, 28).and_hms(19, 34, 0),
        );
    }
    #[test]
    fn should_parse_current_file() {
        let header = example();
        assert_eq!(header.current_file().unwrap(), "DFROC1I",);
    }

    #[test]
    fn should_parse_last_file() {
        let header = example();
        assert_eq!(header.last_file().unwrap(), Some("DFROC1H"),);
    }
    #[test]
    fn should_parse_update() {
        let header = example();
        assert_eq!(header.update().unwrap(), FullOrUpdate::Update,);
    }
    #[test]
    fn should_parse_version() {
        let header = example();
        assert_eq!(header.version().unwrap(), "A",);
    }
    #[test]
    fn should_parse_user_start_date() {
        let header = example();
        assert_eq!(
            header.user_start_date().unwrap(),
            NaiveDate::from_ymd(2020, 6, 28)
        );
    }
    #[test]
    fn should_parse_user_end_date() {
        let header = example();
        assert_eq!(
            header.user_end_date().unwrap(),
            NaiveDate::from_ymd(2021, 6, 28)
        );
    }

    fn example() -> Header {
        // From sample-larger.cif
        let record = Bytes::from(
            b"HDTPS.UDFROC1.PD2006282806201934DFROC1IDFROC1HUA280620280621                    "
                as &[u8],
        );
        Header::from_record(record)
    }
}
