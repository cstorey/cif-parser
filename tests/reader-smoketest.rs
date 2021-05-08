use std::collections::BTreeMap;

use cif_parser::{Reader, Record};
// This file just has to be larger than our default buffer size, to show that
// we refill correctly.
static SAMPLE_FILE: &[u8] = include_bytes!("sample-larger.cif");

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Kind {
    Header,
    TiplocInsert,
    TiplocAmend,
    Association,
    Schedule,
    ChangeEnRoute,
    Trailer,
    Unrecognised(String),
}

#[test]
fn should_read_file() {
    let mut rdr = Reader::new(SAMPLE_FILE);

    let mut nitems = BTreeMap::<Kind, usize>::new();
    while let Some(()) = rdr
        .read_next(|r| match r {
            Record::Header(_) => *nitems.entry(Kind::Header).or_default() += 1,
            Record::TiplocInsert(_) => *nitems.entry(Kind::TiplocInsert).or_default() += 1,
            Record::TiplocAmend(_) => *nitems.entry(Kind::TiplocAmend).or_default() += 1,
            Record::Association(_) => *nitems.entry(Kind::Association).or_default() += 1,
            Record::Schedule(_) => *nitems.entry(Kind::Schedule).or_default() += 1,
            Record::ChangeEnRoute(_) => *nitems.entry(Kind::ChangeEnRoute).or_default() += 1,
            Record::Trailer(_) => *nitems.entry(Kind::Trailer).or_default() += 1,
            Record::Unrecognised(s) => {
                *nitems
                    .entry(Kind::Unrecognised(s[0..2].to_owned()))
                    .or_default() += 1
            }
        })
        .expect("read")
    {}

    let mut expected = BTreeMap::new();
    expected.insert(Kind::Header, 1);
    expected.insert(Kind::Association, 62);
    expected.insert(Kind::Schedule, 113);
    expected.insert(Kind::Trailer, 1);

    assert_eq!(expected, nitems);
}
