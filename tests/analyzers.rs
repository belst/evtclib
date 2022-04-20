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

#[test]
fn test_ai_failed() {
    let log = evtclib::process_file(
        "tests/logs/analyzers/ai-failed-20200922.zevtc",
        Compression::Zip,
    )
    .unwrap();

    let analyzer = log.analyzer().expect("No analyzer for Ai!");

    assert_eq!(analyzer.outcome(), Some(Outcome::Failure));
}

#[test]
fn test_ai_succeeded() {
    let log = evtclib::process_file("tests/logs/ai-20200922.zevtc", Compression::Zip).unwrap();

    let analyzer = log.analyzer().expect("No analyzer for Ai!");

    assert_eq!(analyzer.outcome(), Some(Outcome::Success));
}

#[test]
fn test_mai_cm_succeeded() {
    let log = evtclib::process_file("tests/logs/cms/mai-trin.zevtc", Compression::Zip).unwrap();

    let analyzer = log.analyzer().expect("No analyzer for Mai Trin");

    assert_eq!(analyzer.outcome(), Some(Outcome::Success));
}

#[test]
fn test_mai_cm_failed() {
    let log =
        evtclib::process_file("tests/logs/cms/mai-trin-failed.zevtc", Compression::Zip).unwrap();

    let analyzer = log.analyzer().expect("No analyzer for Mai Trin");

    assert_eq!(analyzer.outcome(), Some(Outcome::Failure));
}

#[test]
fn test_mai_failed_pre_echo() {
    let log = evtclib::process_file(
        "tests/logs/analyzers/mai-failed-pre-echo-20220420.zevtc",
        Compression::Zip,
    )
    .unwrap();

    let analyzer = log.analyzer().expect("No analyzer for Mai Trin");

    assert_eq!(analyzer.outcome(), Some(Outcome::Failure));
}
