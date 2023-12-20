use std::{cmp, collections::BTreeMap};

use cif_parser::{Reader, Record};
use fallible_iterator::FallibleIterator;
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
    tracing_subscriber::fmt::try_init().unwrap_or_default();

    let mut nitems = BTreeMap::<Kind, usize>::new();

    Reader::new(SAMPLE_FILE)
        .map(|r| match r {
            Record::Header(_) => Ok(Kind::Header),
            Record::TiplocInsert(_) => Ok(Kind::TiplocInsert),
            Record::TiplocAmend(_) => Ok(Kind::TiplocAmend),
            Record::Association(_) => Ok(Kind::Association),
            Record::Schedule(_) => Ok(Kind::Schedule),
            Record::ScheduleExtra(_) => Ok(Kind::ScheduleExtra),
            Record::LocationOrigin(_) => Ok(Kind::LocationOrigin),
            Record::LocationIntermediate(_) => Ok(Kind::LocationIntermediate),
            Record::LocationTerminating(_) => Ok(Kind::LocationTerminating),
            Record::ChangeEnRoute(_) => Ok(Kind::ChangeEnRoute),
            Record::Trailer(_) => Ok(Kind::Trailer),
            Record::Unrecognised(bs) => Ok(Kind::Unrecognised(
                String::from_utf8_lossy(&bs[0..cmp::min(bs.len(), 2)]).into_owned(),
            )),
        })
        .for_each(|kind| {
            *nitems.entry(kind).or_default() += 1;
            Ok(())
        })
        .expect("success");

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
