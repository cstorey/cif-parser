use std::{cmp, collections::BTreeMap};

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
    ScheduleExtra,
    LocationOrigin,
    LocationIntermediate,
    LocationTerminating,
    ChangeEnRoute,
    Trailer,
    Unrecognised(String),
}

#[test]
fn should_read_file() {
    env_logger::try_init().unwrap_or_default();

    let mut rdr = Reader::new(SAMPLE_FILE);

    let mut nitems = BTreeMap::<Kind, usize>::new();
    while let Some(()) = rdr
        .read_next(|r| match r {
            Record::Header(_) => *nitems.entry(Kind::Header).or_default() += 1,
            Record::TiplocInsert(_) => *nitems.entry(Kind::TiplocInsert).or_default() += 1,
            Record::TiplocAmend(_) => *nitems.entry(Kind::TiplocAmend).or_default() += 1,
            Record::Association(_) => *nitems.entry(Kind::Association).or_default() += 1,
            Record::Schedule(_) => *nitems.entry(Kind::Schedule).or_default() += 1,
            Record::ScheduleExtra(_) => *nitems.entry(Kind::ScheduleExtra).or_default() += 1,
            Record::LocationOrigin(_) => *nitems.entry(Kind::LocationOrigin).or_default() += 1,
            Record::LocationIntermediate(_) => {
                *nitems.entry(Kind::LocationIntermediate).or_default() += 1
            }
            Record::LocationTerminating(_) => {
                *nitems.entry(Kind::LocationTerminating).or_default() += 1
            }
            Record::ChangeEnRoute(_) => *nitems.entry(Kind::ChangeEnRoute).or_default() += 1,
            Record::Trailer(_) => *nitems.entry(Kind::Trailer).or_default() += 1,
            Record::Unrecognised(bs) => {
                *nitems
                    .entry(Kind::Unrecognised(
                        String::from_utf8_lossy(&bs[0..cmp::min(bs.len(), 2)]).into_owned(),
                    ))
                    .or_default() += 1
            }
        })
        .expect("read")
    {}

    let mut expected = BTreeMap::new();
    expected.insert(Kind::Header, 1);
    expected.insert(Kind::Association, 62);
    expected.insert(Kind::Schedule, 113);
    expected.insert(Kind::ScheduleExtra, 70);
    expected.insert(Kind::LocationOrigin, 70);
    expected.insert(Kind::LocationIntermediate, 2545);
    expected.insert(Kind::LocationTerminating, 70);
    expected.insert(Kind::ChangeEnRoute, 12);
    expected.insert(Kind::Trailer, 1);

    assert_eq!(expected, nitems);
}
