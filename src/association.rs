use crate::reader::CifLine;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Association {
    record: CifLine,
}

impl Association {
    pub(crate) fn from_record(record: CifLine) -> Self {
        Self { record }
    }
    pub fn buf(&self) -> &CifLine {
        &self.record
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn should_parse_association() {
        let assoc =
            b"AANY80987Y808801601041602121111100JJSPRST     TP                               P";
        assert_eq!(80, assoc.len());
        let _example = Association::from_record(*assoc);
    }
}
