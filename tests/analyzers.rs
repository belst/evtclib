//! Test for (some) analyzer functions.
//!
//! Even if those tests do not test the actual functionality, they ensure that the API is usable.

use evtclib::{Compression, Outcome};

#[test]
fn test_xera_failed() {
    let log = evtclib::process_file(
        "tests/logs/analyzers/xera-failed-20200714.zevtc",
        Compression::Zip,
    )
    .unwrap();

    let analyzer = log.analyzer().expect("No analyzer for Xera!");

    assert_eq!(analyzer.outcome(), Some(Outcome::Failure));
}

#[test]
fn test_xera_succeeded() {
    let log = evtclib::process_file(
        "tests/logs/analyzers/xera-success-20200714.zevtc",
        Compression::Zip,
    )
    .unwrap();

    let analyzer = log.analyzer().expect("No analyzer for Xera!");

    assert_eq!(analyzer.outcome(), Some(Outcome::Success));
}
