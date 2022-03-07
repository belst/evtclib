use evtclib::{Compression, EliteSpec, Profession};

macro_rules! test {
    ($name:ident, $log:expr, $profession:expr, $elite:expr $(,)?) => {
        #[test]
        fn $name() {
            let log = evtclib::process_file($log, Compression::Zip).unwrap();
            assert!(log
                .players()
                .any(|p| p.profession() == $profession && p.elite() == Some($elite)));
        }
    };
}

test!(
    willbender,
    "tests/logs/eod-specs/willbender-catalyst-mechanist-20220307.zevtc",
    Profession::Guardian,
    EliteSpec::Willbender,
);

test!(
    vindicator,
    "tests/logs/eod-specs/Vindicator-20220307.zevtc",
    Profession::Revenant,
    EliteSpec::Vindicator,
);

test!(
    bladesworn,
    "tests/logs/eod-specs/Bladesworn-20220307.zevtc",
    Profession::Warrior,
    EliteSpec::Bladesworn,
);

test!(
    mechanist,
    "tests/logs/eod-specs/willbender-catalyst-mechanist-20220307.zevtc",
    Profession::Engineer,
    EliteSpec::Mechanist,
);

test!(
    untamed,
    "tests/logs/eod-specs/Untamed-20220307.zevtc",
    Profession::Ranger,
    EliteSpec::Untamed,
);

test!(
    specter,
    "tests/logs/eod-specs/harbinger-specter-mechanist-20220307.zevtc",
    Profession::Thief,
    EliteSpec::Specter,
);

test!(
    catalyst,
    "tests/logs/eod-specs/willbender-catalyst-mechanist-20220307.zevtc",
    Profession::Elementalist,
    EliteSpec::Catalyst,
);

test!(
    virtuoso,
    "tests/logs/eod-specs/Virtuoso-20220307.zevtc",
    Profession::Mesmer,
    EliteSpec::Virtuoso,
);

test!(
    harbinger,
    "tests/logs/eod-specs/harbinger-specter-mechanist-20220307.zevtc",
    Profession::Necromancer,
    EliteSpec::Harbinger,
);
