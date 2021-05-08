use std::fmt;

use bytes::Bytes;

use crate::{
    errors::CIFParseError,
    helpers::{string_of_slice, string_of_slice_opt},
};

#[derive(Clone, Eq, PartialEq)]
pub struct ScheduleExtra {
    record: Bytes,
}

impl ScheduleExtra {
    pub(crate) fn from_record(record: Bytes) -> Self {
        Self { record }
    }

    pub fn uic_code(&self) -> Result<Option<&str>, CIFParseError> {
        Ok(string_of_slice_opt(&self.record[6..11])?)
    }
    pub fn atoc_code(&self) -> Result<&str, CIFParseError> {
        Ok(string_of_slice(&self.record[11..13])?)
    }
    pub fn applicable_timetable_code(&self) -> Result<&str, CIFParseError> {
        Ok(string_of_slice(&self.record[13..14])?)
    }
}

impl fmt::Debug for ScheduleExtra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("ScheduleExtra");
        s.field("uic_code", &self.uic_code());
        s.field("atoc_code", &self.atoc_code());
        s.field(
            "applicable_timetable_code",
            &self.applicable_timetable_code(),
        );

        s.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_parse_schedule_extra() {
        let extra =
            b"BX         SEY                                                                  ";
        assert_eq!(80, extra.len());
        let example = ScheduleExtra::from_record(Bytes::from(extra.as_ref()));
        assert_eq!(example.uic_code().unwrap(), None);
        assert_eq!(example.atoc_code().unwrap(), "SE");
        assert_eq!(example.applicable_timetable_code().unwrap(), "Y");
    }
    #[test]
    fn should_parse_schedule_extra_2() {
        let extra =
            b"BX    47410ZZY                                                                  ";
        assert_eq!(80, extra.len());
        let example = ScheduleExtra::from_record(Bytes::from(extra.as_ref()));
        assert_eq!(example.uic_code().unwrap(), Some("47410"));
        assert_eq!(example.atoc_code().unwrap(), "ZZ");
        assert_eq!(example.applicable_timetable_code().unwrap(), "Y");
    }
}
