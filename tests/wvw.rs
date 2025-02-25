//! Tests for WvW log parsing.
//!
//! WvW logs are a bit special in some regards (no proper boss ID, players with autogenerated
//! names), so it is good to have some basic testing for those.

#[test]
fn test_smoke() {
    let log = "./tests/logs/wvw-20211112.zevtc";
    let log = evtclib::process_file(log, evtclib::Compression::Zip).unwrap();
    assert!(log.is_generic());
    assert_eq!(log.game_mode(), Some(evtclib::GameMode::WvW));
}
