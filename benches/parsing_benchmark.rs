use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{fs, io, io::Read};
use zip::ZipArchive;

const QADIM_LOG: &str = "tests/logs/qadim-20200427.zevtc";

/// This benchmark tests the overall performance of process_file.
///
/// This is important because for many applications that just want to read a log, this is the
/// easiest and preferred way to do it. We want to ensure that we internally use a fast method
/// (such as a buffered reader or a memory mapped file) so that the downstream application will
/// receive the log fast.
fn zipped_qadim_benchmark(c: &mut Criterion) {
    c.bench_function("on-disk zipped Qadim", |b| {
        b.iter(|| evtclib::process_file(black_box(QADIM_LOG), evtclib::Compression::Zip).unwrap())
    });
}

/// This benchmark tests the process_stream on a pre-loaded zipped log.
///
/// This is important because it contains the slowdown imposed by decompression, but without I/O
/// slowdowns. This is the most realistic target to strive for with our process_file function, as
/// we can try to work around I/O (assuming prefetching, memory mapped files, ...), but we probably
/// have to de-compress logs at some point.
fn zipped_qadim_ram_benchmark(c: &mut Criterion) {
    let log_data = &fs::read(QADIM_LOG).unwrap();

    c.bench_function("in-memory zipped Qadim", |b| {
        b.iter(|| {
            evtclib::process_stream(io::Cursor::new(log_data), evtclib::Compression::Zip).unwrap()
        })
    });
}

/// This benchmark tests the process_stream on a pre-extracted log.
///
/// This is important because it gets rid of any I/O and decompression slowdown. This probably
/// measures our parsing performance most accurately, assuming that all data is readily available,
/// the only slowdown that remains is our parsing and processing algorithm.
fn unzipped_qadim_benchmark(c: &mut Criterion) {
    let mut log_data = Vec::new();
    let zip_data = fs::read(QADIM_LOG).unwrap();
    let mut archive = ZipArchive::new(io::Cursor::new(zip_data)).unwrap();
    archive
        .by_index(0)
        .unwrap()
        .read_to_end(&mut log_data)
        .unwrap();
    let log_data = &log_data;

    c.bench_function("in-memory unzipped Qadim", |b| {
        b.iter(|| {
            evtclib::process_stream(io::Cursor::new(log_data), evtclib::Compression::None).unwrap()
        })
    });
}

/// This benchmark tests the performance of process on a pre-parsed log.
///
/// This is important because it is the point where we can change the most. Parsing the input file
/// is a lot of bit-twiddling and there's probably not a lot of performance that we can gain
/// (especially in the case where I/O slowdown is minimal), but when going from a RawEvtc to a Log,
/// it is the first time that we actually do some work in converting all of the values from
/// low-level to high-level.
fn process_qadim(c: &mut Criterion) {
    let file = io::BufReader::new(fs::File::open(QADIM_LOG).unwrap());
    let raw_evtc = evtclib::raw::parse_zip(file).unwrap();

    c.bench_function("process Qadim", |b| {
        b.iter(|| evtclib::process(&raw_evtc).unwrap())
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(30);
    targets = zipped_qadim_benchmark, zipped_qadim_ram_benchmark, unzipped_qadim_benchmark, process_qadim
}
criterion_main!(benches);
