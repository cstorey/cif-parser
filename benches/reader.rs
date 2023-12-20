use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
    Throughput,
};
use std::{convert::TryInto, fs};

static SAMPLE: &[u8] = include_bytes!("../tests/sample.cif");
static SAMPLE_LARGER: &[u8] = include_bytes!("../tests/sample-larger.cif");

fn read_sample(b: &mut Criterion) {
    bench_reader(b.benchmark_group("smaller"), SAMPLE)
}

fn read_larger(b: &mut Criterion) {
    bench_reader(b.benchmark_group("larger"), SAMPLE_LARGER)
}

fn bench_reader(mut group: BenchmarkGroup<WallTime>, data: &[u8]) {
    group.throughput(Throughput::Bytes(
        data.len().try_into().expect("data len to byte count"),
    ));
    group.bench_function("read", |b| {
        b.iter(|| {
            let mut rdr = cif_parser::Reader::new(data);
            while let Some(data) = rdr.read_next().expect("read") {
                black_box(data);
            }
        })
    });
}

criterion_group!(benches, read_sample, read_larger);
criterion_main!(benches);
