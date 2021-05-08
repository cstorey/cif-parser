use std::fmt;

use bytes::Bytes;

#[derive(Clone, Eq, PartialEq)]
pub struct Trailer {
    record: Bytes,
}

impl Trailer {
    pub(crate) fn from_record(record: Bytes) -> Self {
        Self { record }
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
        let example = Trailer::from_record(Bytes::from(assoc.as_ref()));
        drop(example);
    }
}
