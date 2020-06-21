use std::{collections::BTreeSet, iter};

use nom::{Err, Offset};
use suppositions::generators::*;
use suppositions::*;

use cif_parser::parse;
static SAMPLE_FILE: &[u8] = include_bytes!("sample.cif");

fn splits() -> impl Generator<Item = BTreeSet<usize>> {
    collections(usizes().upto(SAMPLE_FILE.len()))
}

#[test]
fn should_parse_incrementally() {
    let mut expected = Vec::new();
    let mut slice = SAMPLE_FILE;
    while !slice.is_empty() {
        let (rest, val) = parse(slice).expect("parse");
        slice = rest;
        expected.push(val)
    }

    property(splits()).check(|split| {
        let mut start = 0usize;
        let mut actual = Vec::new();
        eprintln!("Split: {:?}", split);
        for off in split.into_iter().chain(iter::once(SAMPLE_FILE.len())) {
            let view_len = 128;
            let (view, ellip) = if SAMPLE_FILE[start..off].len() < view_len {
                (String::from_utf8_lossy(&SAMPLE_FILE[start..off]), "")
            } else {
                (
                    String::from_utf8_lossy(&SAMPLE_FILE[start..start + view_len]),
                    "…",
                )
            };
            eprintln!("Parsing: {}{}", view, ellip);

            loop {
                let slice = &SAMPLE_FILE[start..off];

                match parse(slice) {
                    Ok((rest, val)) => {
                        start += slice.offset(rest);
                        actual.push(val);
                        eprintln!("Off now: {:#?}", start);
                    }

                    Err(Err::Incomplete(need)) => {
                        eprintln!("Incomplete; need: {:?}", need);
                        break;
                    }
                    Err(other) => {
                        let view_len = 128;
                        let (view, ellip) = if slice.len() < view_len {
                            (String::from_utf8_lossy(slice), "")
                        } else {
                            (String::from_utf8_lossy(&slice[0..view_len]), "…")
                        };
                        panic!(
                            "Got unexpected error when parsing slice: {}{}\n{:?}",
                            view, ellip, other
                        )
                    }
                }
            }
        }

        assert_eq!(actual, expected);
    });
}
