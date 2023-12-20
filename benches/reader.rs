use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
    Throughput,
};
use std::{
    convert::TryInto,
    fs::{self, File},
    io::ErrorKind,
    path::Path,
};

static SAMPLE: &[u8] = include_bytes!("../tests/sample.cif");
static SAMPLE_LARGER: &[u8] = include_bytes!("../tests/sample-larger.cif");

fn read_sample(b: &mut Criterion) {
    bench_reader(b.benchmark_group("small in-memory"), SAMPLE)
}

fn read_larger(b: &mut Criterion) {
    bench_reader(b.benchmark_group("larger in-memory"), SAMPLE_LARGER)
}

fn bench_file_reader(b: &mut Criterion) {
    bench_file(b, Path::new("tests/sample.cif"));
    bench_file(b, Path::new("tests/sample-larger.cif"));
    bench_file(b, Path::new("tests/sample-full.cif"))
}

fn bench_file(b: &mut Criterion, path: &Path) {
    let stat: fs::Metadata = match fs::metadata(path) {
        Ok(stat) => stat,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            eprintln!("Skipping group, data file {:?} missing", path);
            return;
        }
        Err(err) => panic!("metadata {:?}: {}", path, err),
    };
    let mut group = b.benchmark_group(path.to_string_lossy());
    group.throughput(Throughput::Bytes(stat.len()));
    group.bench_function("file read", |b| {
        b.iter(|| {
            let file = File::open(path).unwrap_or_else(|e| panic!("metadata {:?}: {}", path, e));
            let mut rdr = cif_parser::Reader::new(file);
            while let Some(data) = rdr.read_next().expect("read") {
                black_box(data);
            }
        })
    });
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

criterion_group!(benches, read_sample, read_larger, bench_file_reader);
criterion_main!(benches);
