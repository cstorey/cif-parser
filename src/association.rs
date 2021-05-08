use bytes::Bytes;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Association {
    record: Bytes,
}

impl Association {
    pub(crate) fn from_record(record: Bytes) -> Self {
        Self { record }
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
        let example = Association::from_record(Bytes::from(assoc.as_ref()));
        drop(example);
    }
}
