#![cfg(feature = "benches")]
#![feature(test)]

extern crate test;

use std::convert::TryInto;

static SAMPLE: &[u8] = include_bytes!("../tests/sample.cif");
static SAMPLE_LARGER: &[u8] = include_bytes!("../tests/sample-larger.cif");

#[bench]
fn read_sample(b: &mut test::Bencher) {
    bench_reader(b, SAMPLE)
}

#[bench]
fn read_larger(b: &mut test::Bencher) {
    bench_reader(b, SAMPLE_LARGER)
}

fn bench_reader(b: &mut test::Bencher, data: &[u8]) {
    b.bytes = data.len().try_into().expect("data len to byte count");

    b.iter(|| {
        let mut rdr = cif_parser::Reader::new(data);
        while let Some(_) = rdr
            .read_next(|r| {
                test::black_box(r);
            })
            .expect("read")
        {}
    });
}
