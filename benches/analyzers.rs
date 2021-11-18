use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use evtclib::Compression;

macro_rules! analyzer_benchmark {
    ($fname:ident, $boss:expr, $log:expr) => {
        fn $fname(c: &mut Criterion) {
            let log = evtclib::process_file($log, Compression::Zip).unwrap();
            let analyzer = log.analyzer().unwrap();

            let mut group = c.benchmark_group(&format!("analyzers/{}", $boss));
            group.throughput(Throughput::Elements(1));

            group.bench_function("is_cm", |b| {
                b.iter(|| {
                    black_box(analyzer.is_cm());
                })
            });

            group.bench_function("outcome", |b| {
                b.iter(|| {
                    black_box(analyzer.outcome());
                })
            });
            group.finish();
        }
    }
}

macro_rules! benchmarks {
    ($(($fname:ident, $boss:expr, $log:expr),)*) => {
        $(analyzer_benchmark!($fname, $boss, $log);)*

        criterion_group! {
            name = benches;
            config = Criterion::default();
            targets = $($fname,)*
        }

        criterion_main!(benches);
    }
}

benchmarks! {
    (raid_generic, "generic-raid", "tests/logs/vg-20200421.zevtc"),
    (raid_xera, "xera", "tests/logs/xera-20200415.zevtc"),
    (raid_cairn, "cairn", "tests/logs/cairn-20200426.zevtc"),
    (raid_mo, "mo", "tests/logs/mo-20200426.zevtc"),
    (raid_samarog, "samarog", "tests/logs/samarog-20200426.zevtc"),
    (raid_deimos, "deimos", "tests/logs/deimos-20200428.zevtc"),
    (raid_sh, "sh", "tests/logs/desmina-20200425.zevtc"),
    (raid_river, "river", "tests/logs/river-20210412.zevtc"),
    (raid_dhuum, "dhuum", "tests/logs/dhuum-20200428.zevtc"),
    (raid_ca, "ca", "tests/logs/ca-20200426.zevtc"),
    (raid_largos, "largos", "tests/logs/largos-20200426.zevtc"),
    (raid_qadim, "qadim", "tests/logs/qadim-20200427.zevtc"),
    (raid_adina, "adina", "tests/logs/adina-20200427.zevtc"),
    (raid_sabir, "sabir", "tests/logs/sabir-20200427.zevtc"),
    (raid_qadimp, "qadimp", "tests/logs/qadimp-20200427.zevtc"),

    (fractal_generic, "generic-fractal", "tests/logs/ensolyss-20200427.zevtc"),
    (fractal_ai, "ai", "tests/logs/ai-20200922.zevtc"),
    (fractal_skorvald, "skorvald", "tests/logs/skorvald-20200920.zevtc"),

    (strike_generic, "generic-strike", "tests/logs/whisper-20200424.zevtc"),
}
