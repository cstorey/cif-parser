use std::fmt;

use crate::reader::CifLine;

#[derive(Clone, Eq, PartialEq)]
pub struct Trailer {
    record: CifLine,
}

impl Trailer {
    pub(crate) fn from_record(record: CifLine) -> Self {
        Self { record }
    }
    pub fn buf(&self) -> &CifLine {
        &self.record
    }
}

impl fmt::Debug for Trailer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Trailer");
        s.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_trailer() {
        let assoc =
            b"ZZ                                                                              ";
        assert_eq!(80, assoc.len());
        let _example = Trailer::from_record(*assoc);
    }
}
